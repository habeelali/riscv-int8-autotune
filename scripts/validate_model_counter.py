#!/usr/bin/env python3
"""Independently verify full-network CSR counts with executed-insn callbacks."""
import json
import re
import subprocess
from runner import ROOT, QEMU, QFLAGS

dest = ROOT/'results/model/person-detection'
rows = json.loads((dest/'results.json').read_text())
esp = min((r for r in rows if r['kind'] in ('c','c-native')),key=lambda r:r['mean_instructions'])
checks = []
for row in rows:
    if row['kind'] not in (esp['kind'],'generated-selected'):
        continue
    build = ROOT/row['build_dir']
    assembly = (build/'assembly.txt').read_text()
    regions = re.findall(r'^[0-9a-f]+ <measure_model[^>]*>:\n(.*?)(?=\n\n)', assembly, re.M|re.S)
    full = [r for r in regions if re.search(r'<model>',r)]
    assert len(full) == 1, full
    pcs = re.findall(r'^\s*([0-9a-f]+):.*\brdinstret\b',full[0],re.M)
    assert len(pcs) == 2
    command = [QEMU,*QFLAGS,'-plugin',f'{ROOT}/qemu/count-plugin.so,0x{pcs[0]},0x{pcs[1]}',
               '-kernel',str(build/'model.elf')]
    p = subprocess.run(command,capture_output=True,text=True,timeout=300)
    assert p.returncode == 0,p.stderr
    (build/'plugin-stdout.txt').write_text(p.stdout)
    (build/'plugin-stderr.txt').write_text(p.stderr)
    counts = list(map(int,re.findall(r'PLUGIN_REGION (\d+)',p.stderr)))
    expected = [s['raw_instructions'] for s in row['samples']]
    assert counts == expected,(counts,expected)
    checks.append(dict(kind=row['kind'],plugin_counts=counts,csr_counts=expected,match=True,command=command))
    print(row['kind'],counts,flush=True)
assert len(checks) == 2
(dest/'counter-crosscheck.json').write_text(json.dumps(checks,indent=2)+'\n')
