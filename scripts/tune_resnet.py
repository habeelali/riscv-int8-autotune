#!/usr/bin/env python3
"""Bounded deterministic model schedule search after measured C/generic milestones."""
import copy,json
import numpy as np
from runner import ROOT,run
from resnet_model import DEST
from model_import import array_decl
from tune_algorithm import parse_constants
from benchmark_resnet import benchmark

m=json.loads((DEST/'manifest.json').read_text())
assert (DEST/'baseline-milestone.json').exists() and json.loads((DEST/'generic/result.json').read_text())['correct']
constants=parse_constants((DEST/'constants.h').read_text())
rows=[]
for schedule in range(3):
    out=DEST/f'generated-{schedule}';out.mkdir(exist_ok=True)
    manifest=copy.deepcopy(m);params=copy.deepcopy(constants)
    for l in manifest['layers']:
        if 'workload'not in l:continue
        j=l['index'];w=l['workload'];width=(w['width']+2*w['padding']-w['kernel'])//w['stride']+1
        block,pixels=[(4,2),(2,4),(8,1)][schedule]
        pixels=min(pixels,width)
        if pixels not in [1,2,4]:pixels=1
        cfg=dict(block=block,pixels=pixels,unroll=1,spatial_unroll=False,im2col=w['kernel']>1,interior=False,channel_outer=False,fast_quant=True)
        wt=params[f'r_weights_{j}'][1].reshape(w['output_channels'],-1);bias=params[f'r_bias_{j}'][1]
        if w['padding']==0:
            bias=bias+w['input_offset']*wt.sum(axis=1);w['input_offset']=0;params[f'r_bias_{j}']=('int32_t',bias)
        bound=np.abs(bias)+np.abs(wt).sum(axis=1)*(128+abs(w['input_offset']))
        assert max(bound)<=(1<<30) and all(-30<=s<=-1 for s in w['shifts'])
        l.update(config=cfg,bounded_quant=True,prevalidated=True,maximum_accumulator_absolute_bound=int(max(bound)),packed=False,chunks=False)
    (out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
    run(['cargo','run','--quiet','--locked','-p','kernel-generator','--example','algorithm_model','--',out/'manifest.json',out/'source.rs'],cwd=ROOT)
    header='#include <stdint.h>\n'+''.join(array_decl(n,v,t) for n,(t,v) in params.items())
    (out/'parameters.h').write_text(header)
    row=benchmark(m,f'generated-{schedule}',source=(out/'source.rs').read_text(),constants=header);rows.append(row)
    (DEST/'search.json').write_text(json.dumps(rows,indent=2)+'\n')
best=min(rows,key=lambda r:r['mean_instructions_raw'])
(DEST/'selection.json').write_text(json.dumps(dict(best=best,scope='QEMU instruction selection only; not hardware timing',candidates=rows),indent=2)+'\n')
print('Selected',best['kind'],flush=True)
