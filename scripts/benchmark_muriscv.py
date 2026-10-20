#!/usr/bin/env python3
"""Measure upstream muRISCV-NN algorithms on the identical operator corpus."""
import argparse,json
from pathlib import Path
from runner import ROOT,benchmark,save
from muriscv_backend import KINDS
p=argparse.ArgumentParser();p.add_argument('--kinds',nargs='+',choices=KINDS,default=list(KINDS));p.add_argument('--workload');a=p.parse_args()
rows=[];failures=[]
paths=[Path(a.workload)] if a.workload else sorted((ROOT/'workloads').glob('*.toml'))
for kind in a.kinds:
    for path in paths:
        try:
            r=benchmark(path,kind);rows.append(r);save(rows,ROOT/'results/muriscv.json');print(kind,r['workload'],r['instructions'],flush=True)
        except Exception as e:
            failures.append(dict(kind=kind,workload=path.stem,error=str(e)));print(kind,path.stem,'REJECTED',str(e)[-300:],flush=True)
        (ROOT/'results/muriscv-failures.json').write_text(json.dumps(failures,indent=2)+'\n')
(ROOT/'results/muriscv-milestone.json').write_text(json.dumps(dict(validated=bool(rows),measurements=len(rows),failed=len(failures)),indent=2)+'\n')
if not rows:raise SystemExit('No valid upstream implementation')
