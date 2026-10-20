#!/usr/bin/env python3
"""Exact offline zero-point correction and scalar blocked-weight model search."""
import copy
import json
import re
import shutil
import time
from pathlib import Path

import numpy as np

from benchmark_model import benchmark,graph_source
from model_import import array_decl,load_model
from runner import ROOT,run
from tune_flash import constraints

DEST=ROOT/'results/algorithm'
REFERENCE=DEST/'reference'


def parse_constants(text):
    result={}
    for typ,name,length,values in re.findall(r'static const (int8_t|int32_t) (\w+)\[(\d+)\]\s*=\s*\{([^}]*)\};',text):
        array=np.fromstring(values,sep=',',dtype=np.int64)
        assert len(array)==int(length)
        result[name]=(typ,array)
    assert result
    return result


def config(w,schedule):
    c=dict(block=4,unroll=1,spatial_unroll=w['operator']=='depthwise',im2col=False,
           interior=False,pixels=1,channel_outer=False,fast_quant=True)
    if w['operator']=='conv':
        c['block']=2 if schedule==1 else 4
        c['pixels']=2 if (w['width']+2*w['padding']-w['kernel'])//w['stride']+1>=2 else 1
        c['im2col']=w['kernel']>1
        if schedule==2 and (w['width']+2*w['padding']-w['kernel'])//w['stride']+1>=4:
            c.update(pixels=4,block=2)
        if schedule==3:
            c.update(pixels=1,im2col=False,block=8)
    elif schedule==1:
        c.update(block=1,channel_outer=True)
    elif schedule==2:
        c['block']=8
    elif schedule==3:
        c['block']=2
    return c


def make_variant(manifest,name,fold=False,packed=False,chunks=False,explicit_depthwise=False,bounded_quant=False,prevalidated=False,dw_tile=False,dw_unroll=False,overrides=None):
    path=DEST/'variants'/name
    path.mkdir(parents=True,exist_ok=True)
    output=copy.deepcopy(manifest)
    graph=copy.deepcopy(manifest)
    schedules=json.loads((ROOT/'generated/person-detection/selection.json').read_text())['schedules']
    constants=parse_constants((ROOT/'results/model/person-detection/constants.h').read_text())
    for layer in output['layers']:
        if 'workload' not in layer:
            continue
        j=layer['index'];w=layer['workload']
        options=(overrides or {}).get(str(j),{})
        if options.get('explicit_depthwise',explicit_depthwise) and w['operator']=='depthwise' and w['padding']:
            pad=w['padding'];w['height']+=2*pad;w['width']+=2*pad;w['padding']=0
            gl=graph['layers'][j]
            gl['workload'].update(height=w['height'],width=w['width'],padding=0)
            gl.update(explicit_padding=True,pad_before=pad,pad_total=2*pad)
        c=config(w,schedules[str(j)])
        c.update(options.get('config',{}))
        layer['config']=c
        fold_here=options.get('fold',fold) and w['padding']==0
        packed_here=options.get('packed',packed) and w['operator']=='conv'
        chunks_here=options.get('chunks',chunks) and w['operator']=='conv' and w['kernel']==1
        if chunks_here:assert fold_here and packed_here
        layer.update(packed=packed_here,folded=fold_here,chunks=chunks_here,original_input_offset=w['input_offset'])
        layer['cache_channel_outer']=options.get('cache_channel_outer',False)
        layer['dw_tile']=options.get('dw_tile',dw_tile) and w['operator']=='depthwise'
        layer['dw_unroll']=options.get('dw_unroll',dw_unroll)
        weight_name,bias_name=f'weights_{j}',f'bias_{j}'
        typ,weights=constants[weight_name]
        bias_typ,bias=constants[bias_name]
        oc,ic,k=w['output_channels'],w['input_channels'],w['kernel']
        sums=weights.reshape(k*k,ic).sum(axis=0) if w['operator']=='depthwise' else weights.reshape(oc,-1).sum(axis=1)
        if fold_here:
            bias=bias+w['input_offset']*sums
            assert np.all(bias>=-(1<<31)) and np.all(bias<(1<<31))
            w['input_offset']=0
            constants[bias_name]=(bias_typ,bias)
        abs_sum=np.abs(weights).reshape(k*k,ic).sum(axis=0) if w['operator']=='depthwise' else np.abs(weights).reshape(oc,-1).sum(axis=1)
        bound=np.abs(bias)+abs_sum*(128+abs(w['input_offset']))
        assert np.max(bound)<(1<<31),'INT32 accumulation bound violated'
        layer['maximum_accumulator_absolute_bound']=int(np.max(bound))
        bounded_here=options.get('bounded_quant',bounded_quant)
        if bounded_here:
            assert np.max(bound)<=(1<<30) and all(-30<=s<=-1 for s in w['shifts'])
        layer['bounded_quant']=bounded_here
        prevalidated_here=options.get('prevalidated',prevalidated)
        if prevalidated_here:assert bounded_here
        layer['prevalidated']=prevalidated_here
        if packed_here:
            block=c['block']
            assert oc%block==0
            weights=weights.reshape(oc//block,block,-1).transpose(0,2,1).copy().reshape(-1)
            constants[weight_name]=(typ,weights)
    output['parameter_contract']='Weights [OC/block,reduction,block] where packed=true; folded bias=bias+original_input_offset*sum(weights) only where padding=0. Input and output tensors unchanged.'
    (path/'manifest.json').write_text(json.dumps(output,indent=2)+'\n')
    (path/'graph-manifest.json').write_text(json.dumps(graph,indent=2)+'\n')
    old_scratch=max(l['workload']['height']*l['workload']['width']*l['workload']['input_channels'] for l in manifest['layers'] if l.get('explicit_padding'))
    new_scratch=max(l['workload']['height']*l['workload']['width']*l['workload']['input_channels'] for l in graph['layers'] if l.get('explicit_padding'))
    assert new_scratch<=old_scratch,'No extra padding arena is allowed'
    (path/'graph.c').write_text(graph_source(graph))
    header='#include <stdint.h>\n'+''.join(array_decl(name,array,typ) for name,(typ,array) in constants.items())
    (path/'constants.h').write_text(header)
    run(['cargo','run','--quiet','--locked','-p','kernel-generator','--example','algorithm_model',
         '--',path/'manifest.json',path/'kernels.rs'],cwd=ROOT)
    return path


def evaluate(manifest,name,**options):
    path=make_variant(manifest,name,**options)
    row=benchmark(manifest,'algorithm-'+name,source_override=path/'kernels.rs',constants_override=path/'constants.h',graph_override=path/'graph.c',linker_override=DEST/'link.ld')
    row.update(variant=name,source=str((path/'kernels.rs').relative_to(ROOT)),
               parameter_manifest=str((path/'manifest.json').relative_to(ROOT)))
    return row


def layer_options(row,index,original):
    if 'parameter_manifest' not in row:
        return dict(fold=False,packed=False,chunks=False,bounded_quant=False,prevalidated=False,explicit_depthwise=False,
                    config=config(original['workload'],json.loads((ROOT/'generated/person-detection/selection.json').read_text())['schedules'][str(index)]))
    layer=json.loads((ROOT/row['parameter_manifest']).read_text())['layers'][index]
    return dict(fold=layer['folded'],packed=layer['packed'],chunks=layer.get('chunks',False),
                bounded_quant=layer.get('bounded_quant',False),config=layer['config'],
                prevalidated=layer.get('prevalidated',False),
                dw_tile=layer.get('dw_tile',False),dw_unroll=layer.get('dw_unroll',False),
                explicit_depthwise=original['workload']['padding']>0 and layer['workload']['padding']==0)


def main():
    started=time.monotonic()
    manifest=load_model(ROOT/'results/model/person-detection')
    (DEST/'link.ld').write_text((ROOT/'qemu/link.ld').read_text().replace('.bss (NOLOAD)','.bss ALIGN(16) (NOLOAD)'))
    assert manifest==json.loads((REFERENCE/'manifest.json').read_text())
    reference=benchmark(manifest,'algorithm-reference',source_override=REFERENCE/'kernels.rs',linker_override=DEST/'link.ld')
    frozen=json.loads((REFERENCE/'flash-best.json').read_text())['best']
    assert reference['text_bytes']==frozen['text_bytes']
    assert all(a['instructions']==b['instructions'] for a,b in zip(reference['samples'],frozen['samples']))
    (DEST/'replayed-reference.json').write_text(json.dumps(reference,indent=2)+'\n')
    rows=[]
    pointwise=[str(l['index']) for l in manifest['layers'] if 'workload' in l and l['workload']['operator']=='conv' and l['workload']['kernel']==1]
    shared=['14','16','18','20','22']
    plans=[dict(name='fold',fold=True),dict(name='packed',packed=True),dict(name='fold-packed',fold=True,packed=True),dict(name='chunks',fold=True,packed=True,chunks=True),dict(name='explicit-fold',fold=True,explicit_depthwise=True),dict(name='explicit-chunks',fold=True,packed=True,chunks=True,explicit_depthwise=True),dict(name='bounded',bounded_quant=True),dict(name='bounded-fold',bounded_quant=True,fold=True),dict(name='bounded-packed-fold',bounded_quant=True,fold=True,packed=True),dict(name='bounded-chunks',bounded_quant=True,fold=True,packed=True,chunks=True),dict(name='bounded-explicit',bounded_quant=True,fold=True,explicit_depthwise=True)]
    for u in (2,4,8):
        for scope,layers in [('all',pointwise),('shared',shared)]:
            plans.append(dict(name=f'bounded-fold-u{u}-{scope}',bounded_quant=True,fold=True,
                              overrides={j:dict(config=dict(unroll=u)) for j in layers}))
    for block,pixels in [(2,4),(2,2),(4,2),(4,4),(8,1)]:
        plans.append(dict(name=f'chunks-b{block}-p{pixels}',bounded_quant=True,fold=True,packed=True,chunks=True,
                          overrides={j:dict(config=dict(block=min(block,manifest['layers'][int(j)]['workload']['output_channels']),pixels=pixels)) for j in pointwise}))
    dw=[str(l['index']) for l in manifest['layers'] if 'workload' in l and l['workload']['operator']=='depthwise']
    for block,pixels in [(1,2),(2,2),(4,2),(2,4),(4,1),(8,1)]:
        plans.append(dict(name=f'dw-tile-b{block}-p{pixels}',bounded_quant=True,fold=True,prevalidated=True,explicit_depthwise=True,dw_tile=True,
                          overrides={j:dict(config=dict(block=block,pixels=pixels)) for j in dw}))
    plans.append(dict(name='dw-tile-b4-p2-unrolled',bounded_quant=True,fold=True,prevalidated=True,explicit_depthwise=True,dw_tile=True,dw_unroll=True,
                      overrides={j:dict(config=dict(block=4,pixels=2)) for j in dw}))
    for plan in plans:
        name=plan.pop('name')
        row=evaluate(manifest,name,**plan)
        row['rejection_reasons']=constraints(row,reference)
        row['eligible']=not row['rejection_reasons']
        rows.append(row)
        (DEST/'candidates.json').write_text(json.dumps(rows,indent=2)+'\n')
        print(name,row['text_bytes'],'text bytes;',row['mean_instructions'],'instructions;',
              'ELIGIBLE' if row['eligible'] else 'REJECTED '+', '.join(row['rejection_reasons'][:3]),flush=True)
    for allow_explicit in (True,False):
        overrides={}
        for layer in manifest['layers']:
            if 'workload' not in layer:continue
            j=layer['index']
            def eligible_layer(r):
                if not allow_explicit and layer_options(r,j,layer)['explicit_depthwise']:return False
                return all(a['instructions']<=b['instructions'] for a,b in zip(r['layers'],reference['layers']) if a['layer']==j)
            valid=[r for r in [reference,*rows] if eligible_layer(r)]
            winner=min(valid,key=lambda r:sum(v['instructions'] for v in r['layers'] if v['layer']==j and v['sample']<2))
            overrides[str(j)]=layer_options(winner,j,layer)
        name='mixture-fast' if allow_explicit else 'mixture-no-explicit'
        row=evaluate(manifest,name,overrides=overrides)
        row['rejection_reasons']=constraints(row,reference);row['eligible']=not row['rejection_reasons'];rows.append(row)
        (DEST/'candidates.json').write_text(json.dumps(rows,indent=2)+'\n')
        print(name,row['text_bytes'],'text bytes;',row['mean_instructions'],'instructions;',
              'ELIGIBLE' if row['eligible'] else 'REJECTED '+', '.join(row['rejection_reasons'][:3]),flush=True)
        validated={j:dict(opt,prevalidated=True) for j,opt in overrides.items()}
        assert all(opt['bounded_quant'] for opt in validated.values())
        name+='-validated'
        row=evaluate(manifest,name,overrides=validated)
        row['rejection_reasons']=constraints(row,reference);row['eligible']=not row['rejection_reasons'];rows.append(row)
        (DEST/'candidates.json').write_text(json.dumps(rows,indent=2)+'\n')
        print(name,row['text_bytes'],'text bytes;',row['mean_instructions'],'instructions;',
              'ELIGIBLE' if row['eligible'] else 'REJECTED '+', '.join(row['rejection_reasons'][:3]),flush=True)
    (DEST/'initial-search.json').write_text(json.dumps(dict(reference=reference,rows=rows,search_seconds=time.monotonic()-started),indent=2)+'\n')
    eligible=[r for r in rows if r['eligible']]
    if not eligible:raise RuntimeError('No joint improvement; compact fallback retained')
    best=min(eligible,key=lambda r:(r['mean_instructions'],r['text_bytes']+r['rodata_bytes']))
    summary=dict(reference=reference,best=best,candidates=len(rows),
                 objective='minimum instructions under the prior compact image/code/RAM/stack budgets; every layer and input must not regress',
                 search_wall_seconds=time.monotonic()-started)
    (DEST/'best.json').write_text(json.dumps(summary,indent=2)+'\n')
    export=ROOT/'generated/person-detection/algorithm';export.mkdir(exist_ok=True)
    source=ROOT/best['source'];variant=source.parent
    for name in ('kernels.rs','constants.h','manifest.json','graph-manifest.json','graph.c'):
        shutil.copyfile(variant/name,export/name)
    shutil.copyfile(DEST/'best.json',export/'selection-evidence.json')
    print('Selected',best['variant'],best['mean_instructions'],'instructions and',best['text_bytes'],'text bytes',flush=True)


if __name__=='__main__':
    main()
