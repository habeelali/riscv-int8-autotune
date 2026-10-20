#!/usr/bin/env python3
"""Re-measure archived first-generation selections with the current common runtime."""
import json,subprocess
from runner import ROOT,benchmark,save
rows=[];dest=ROOT/'generated/previous-selected';dest.mkdir(exist_ok=True)
for path in sorted((ROOT/'workloads').glob('*.toml')):
    name=path.stem
    # The pre-muRISCV snapshot remains pinned independently of subsequent commits.
    reference=json.loads((ROOT/'results/phase-before-muriscv'/f'best-{name}.json').read_text())['best']
    source=ROOT/reference['source']
    # Same named old variants are retained; generator upgrades preserve their schedules.
    # Git retrieves their exact bytes from the snapshot parent, not the overwritten working file.
    pin=json.loads((ROOT/'results/previous-source-pin.json').read_text())['commit']
    exported=dest/f'{name}.rs'
    content=exported.read_text() if exported.exists() else subprocess.check_output(['git','show',f'{pin}:generated/{name}/best.rs'],cwd=ROOT,text=True)
    exported.write_text(content)
    r=benchmark(path,'previous-selected',exported);rows.append(r);save(rows,ROOT/'results/previous-selected.json');print(name,r['instructions'],flush=True)
