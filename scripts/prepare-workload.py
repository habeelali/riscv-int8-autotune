#!/usr/bin/env python3
"""Ensure each tuned specification has its own correct C and generic measurements."""
import json,sys
from runner import ROOT,load,benchmark,save
from benchmark_generic import source
path=sys.argv[1];w=load(path)
for kind,filename in [('c','baseline'),('generic','generic')]:
    src=None
    if kind=='generic':
        src=ROOT/'generated'/f'{w["name"]}_generic.rs';src.write_text(source(w))
    dest=ROOT/'results'/f'{filename}.json'
    rows=json.loads(dest.read_text())
    if any(r['spec']==w and r['correct'] for r in rows):continue
    r=benchmark(path,kind,src)
    rows=[old for old in rows if old['workload']!=w['name']]+[r]
    save(rows,dest)
    print(w['name'],kind,r['instructions'],flush=True)
