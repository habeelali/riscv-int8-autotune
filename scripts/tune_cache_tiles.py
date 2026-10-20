#!/usr/bin/env python3
"""Measure reusable cache tiles on the saved, already baselined ResNet graph."""
import copy, hashlib, json, time
import numpy as np
from runner import ROOT, run
from resnet_model import DEST
from model_import import array_decl
from tune_algorithm import parse_constants
from benchmark_resnet import benchmark

CONFIGS=[('tile8_b2p4',8,2,4,False),('tile16_b2p4',16,2,4,False),
         ('tile16_b2p4_sram',16,2,4,True),('tile16_b4p2',16,4,2,False)]

def prepare(m,name,tile,block,pixels,stage):
    out=DEST/name;out.mkdir(exist_ok=True)
    manifest=copy.deepcopy(m);params=parse_constants((DEST/'constants.h').read_text())
    for l in manifest['layers']:
        if 'workload' not in l:continue
        j=l['index'];w=l['workload'];b=block if w['output_channels']%block==0 else 2
        spatial=((w['height']+2*w['padding']-w['kernel'])//w['stride']+1)*((w['width']+2*w['padding']-w['kernel'])//w['stride']+1)
        t=min(tile,spatial);p=min(pixels,t)
        wt=params[f'r_weights_{j}'][1].reshape(w['output_channels'],-1)
        bias=params[f'r_bias_{j}'][1]+w['input_offset']*wt.sum(axis=1)
        # All padding is now explicitly represented by its original INT8 zero.
        # This makes the same offline correction valid for interior AND borders.
        zero=-w['input_offset'];assert -128<=zero<=127
        w['input_offset']=0
        bound=np.abs(bias)+128*np.abs(wt).sum(axis=1)
        assert max(bound)<=1<<30 and all(-30<=s<=-1 for s in w['shifts'])
        params[f'r_bias_{j}']=('int32_t',bias)
        params[f'r_weights_{j}']=('int8_t',wt.reshape(w['output_channels']//b,b,-1).transpose(0,2,1).reshape(-1))
        cfg=dict(block=b,pixels=p,unroll=1,spatial_unroll=False,im2col=True,interior=False,channel_outer=False,fast_quant=True)
        l.update(config=cfg,cache_tile=t,original_input_zero=zero,stage_weights=False,stage_all_weights=stage,packed=True,
                 bounded_quant=True,prevalidated=True,maximum_accumulator_absolute_bound=int(max(bound)),
                 explicit_stack_scratch_bytes=t*wt.shape[1])
    (out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
    run(['cargo','run','--quiet','--locked','-p','kernel-generator','--example','algorithm_model','--',out/'manifest.json',out/'source.rs'],cwd=ROOT)
    header='#include <stdint.h>\n'+''.join(array_decl(n,v,t) for n,(t,v) in params.items())
    (out/'parameters.h').write_text(header)
    return out,header

if __name__=='__main__':
    m=json.loads((DEST/'manifest.json').read_text())
    assert json.loads((DEST/'baseline-milestone.json').read_text())['correct']
    rows=[];start=time.monotonic()
    for cfg in CONFIGS:
        out,header=prepare(m,*cfg)
        graph=copy.deepcopy(m)
        if cfg[4]:
            for l in graph['layers']:
                if 'workload' in l:l['stage_all_weights']=True
        row=benchmark(graph,cfg[0],source=(out/'source.rs').read_text(),constants=header)
        row['explicit_stack_scratch_max']=max(l.get('explicit_stack_scratch_bytes',0) for l in json.loads((out/'manifest.json').read_text())['layers'])
        rows.append(row)
        (DEST/'cache-tile-search.json').write_text(json.dumps(dict(candidates=rows,elapsed_seconds=time.monotonic()-start,selection='Hardware selection pending; retain all QEMU results'),indent=2)+'\n')
