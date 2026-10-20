#!/usr/bin/env python3
"""Fresh seeds for every globally qualified upstream configuration, including the local fix."""
import json,tempfile
from pathlib import Path
from runner import ROOT,benchmark,save,load
qualified=[r['kind'] for r in json.loads((ROOT/'results/upstream-quantization.json').read_text()) if r['qualified']]
rows=[]
with tempfile.TemporaryDirectory(prefix='muriscv-regression-') as d:
    for path in sorted((ROOT/'workloads').glob('*.toml')):
        original=load(path)
        for seed in (1,0xdeadbeef,123456789):
            w=dict(original,seed=seed);spec=Path(d)/(w['name']+'.toml')
            spec.write_text('\n'.join(f'{k} = {json.dumps(v)}' for k,v in w.items())+'\n')
            group=[]
            for kind in qualified:
                r=benchmark(spec,kind);rows.append(r);group.append(r)
            assert len({r['checksum'] for r in group})==1
            save(rows,ROOT/'results/muriscv-regression.json')
            print(w['name'],seed,'qualified upstream variants correct',flush=True)
print('All upstream fresh-seed regressions passed:',len(rows),'executions')
