#!/usr/bin/env python3
"""Cross-check the CSR region against executed-insn callbacks, not TB counts."""
import json,re,subprocess
from pathlib import Path
from runner import ROOT,QEMU,QFLAGS
rows=json.loads((ROOT/'results/baseline.json').read_text())
selected=[r for r in rows if r['workload'] in ('conv_small','depthwise_small','fc_small','pointwise_small')]
selected += [json.loads(p.read_text())['best'] for p in sorted((ROOT/'results').glob('best-*.json')) if '_small' in p.stem]
if (ROOT/'results/muriscv.json').exists():
    raw=json.loads((ROOT/'results/muriscv.json').read_text())
    qualified={r['kind'] for r in json.loads((ROOT/'results/upstream-quantization.json').read_text()) if r['qualified']}
    for name in ('conv_small','depthwise_small','fc_small','pointwise_small'):
        selected.append(min((r for r in raw if r['workload']==name and r['correct'] and r['kind'] in qualified),key=lambda r:r['instructions']))
checks=[]
for r in selected:
    build=ROOT/r['build_dir'];asm=(build/'assembly.txt').read_text()
    region=asm.split('<measure>:',1)[1].split('\n\n',1)[0]
    pcs=re.findall(r'^\s*([0-9a-f]+):.*\brdinstret\b',region,re.M)
    assert len(pcs)==2,pcs
    cmd=[QEMU,*QFLAGS,'-plugin',f'{ROOT}/qemu/count-plugin.so,0x{pcs[0]},0x{pcs[1]}','-kernel',str(build/'kernel.elf')]
    p=subprocess.run(cmd,capture_output=True,text=True,timeout=120)
    assert p.returncode==0,p.stderr
    counts=list(map(int,re.findall(r'PLUGIN_REGION (\d+)',p.stderr)))
    assert counts==[r['measurement_overhead'],r['raw_instructions']],(counts,r)
    checks.append(dict(workload=r['workload'],kind=r['kind'],plugin_counts=counts,csr_counts=[r['measurement_overhead'],r['raw_instructions']],match=True))
    print(r['workload'],r['kind'],counts,flush=True)
(ROOT/'results/counter-crosscheck.json').write_text(json.dumps(checks,indent=2)+'\n')
