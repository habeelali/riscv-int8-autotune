#!/usr/bin/env python3
"""Generate and QEMU-validate cache-order candidates, without hardware timing claims."""
import json
from pathlib import Path
from tune_algorithm import make_variant, layer_options
from benchmark_model import benchmark
from runner import ROOT

DEST=ROOT/'results/hardware-cache'
DEST.mkdir(exist_ok=True)
original=json.loads((ROOT/'results/algorithm/reference/manifest.json').read_text())
old=json.loads((ROOT/'results/algorithm/best.json').read_text())['best']
overrides={str(l['index']):layer_options(old,l['index'],l) for l in original['layers'] if 'workload' in l}
rows=[]
for block,pixels in [(4,2),(2,4)]:
    options=json.loads(json.dumps(overrides))
    for l in original['layers']:
        if l['index'] in [14,16,18,20,22,24,26]:
            o=options[str(l['index'])]
            o.update(packed=True,chunks=True,fold=True,cache_channel_outer=True)
            o['config'].update(block=block,pixels=pixels,unroll=1)
    name=f'hardware-cache-b{block}-p{pixels}'
    path=make_variant(original,name,overrides=options)
    row=benchmark(original,name,source_override=path/'kernels.rs',constants_override=path/'constants.h',graph_override=path/'graph.c',linker_override=ROOT/'results/algorithm/link.ld')
    row.update(variant=name,source=str((path/'kernels.rs').relative_to(ROOT)),
               parameter_manifest=str((path/'manifest.json').relative_to(ROOT)),hardware_cycles=None)
    rows.append(row)
    (DEST/'candidates.json').write_text(json.dumps(rows,indent=2)+'\n')
    print(name,'correct=',row['correct'],'instructions=',row['mean_instructions'],'text=',row['text_bytes'],flush=True)
