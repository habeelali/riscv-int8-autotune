#!/usr/bin/env python3
"""Fresh-seed QEMU regression tests plus a deliberately incorrect kernel rejection."""
import json,tempfile,tomllib
from pathlib import Path
from runner import ROOT,benchmark,save
from benchmark_generic import source
rows=[]
with tempfile.TemporaryDirectory(prefix='int8-regression-') as d:
    d=Path(d)
    for path in sorted((ROOT/'workloads').glob('*.toml')):
        w=tomllib.loads(path.read_text())
        best=json.loads((ROOT/'results'/f'best-{w["name"]}.json').read_text())['best']
        generated=ROOT/best['source']
        for seed in (1,0xdeadbeef,123456789):
            spec=dict(w,seed=seed)
            work=d/(w['name']+'.toml')
            work.write_text('\n'.join(f'{k} = {json.dumps(v)}' for k,v in spec.items())+'\n')
            generic=d/'generic.rs';generic.write_text(source(spec))
            for kind,src in [('c',None),('generic',generic),('generated',generated)]:
                r=benchmark(work,kind,src);rows.append(r)
            assert len({r['checksum'] for r in rows[-3:]})==1
            save(rows,ROOT/'results/regression.json')
            print(w['name'],seed,'C / generic / generated correct',flush=True)
    # Negative integration test: compile a real wrong kernel and require rejection.
    w=tomllib.loads((ROOT/'workloads/fc_small.toml').read_text())
    bad=d/'incorrect.rs'
    bad.write_text('#![no_std]\n#[no_mangle] pub unsafe extern "C" fn kernel(_: *const i8,_: *const i8,_: *const i32,_: *const i32,_: *const i32,o:*mut i8){core::ptr::write_bytes(o,0,13);}\n')
    try:
        benchmark(ROOT/'workloads/fc_small.toml','negative-correctness-test',bad,True)
    except RuntimeError as e:
        assert 'Numerical mismatch' in str(e),str(e)
        first_error=str(e)
        try:
            benchmark(ROOT/'workloads/fc_small.toml','negative-correctness-test',bad,False)
        except RuntimeError as cached_error:
            assert 'Numerical mismatch (cached)' in str(cached_error)
        else: raise AssertionError('Cached incorrect implementation was accepted')
        (ROOT/'results/negative-correctness-test.json').write_text(json.dumps(dict(rejected=True,cached_rejected=True,reason=first_error),indent=2)+'\n')
    else:raise AssertionError('Incorrect implementation was accepted')
print('All QEMU regressions and the incorrect-kernel rejection passed.')
