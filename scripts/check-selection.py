#!/usr/bin/env python3
"""Exercise every selection objective on actual saved candidate measurements."""
import json,subprocess
from pathlib import Path
from runner import ROOT
path=ROOT/'results/best-fully_connected.json';records=[]
try:
    for objective in ('min-flash','budget','pareto','min-instructions'):
        subprocess.run(['cargo','run','--quiet','--locked','-p','autotuner','--','tune','workloads/fully_connected.toml',objective],cwd=ROOT,check=True,stdout=subprocess.DEVNULL)
        chosen=json.loads(path.read_text());rows=json.loads((ROOT/'results/tuning-fully_connected.json').read_text())
        eligible=[r for r in rows if r.get('correct') and r['flash_bytes']<=r['spec']['flash_budget'] and r['static_ram_bytes']+r['reserved_stack_bytes']<=r['spec']['sram_budget']]
        metric='flash_bytes' if objective=='min-flash' else 'instructions'
        assert chosen['best'][metric]==min(r[metric] for r in eligible)
        for r in chosen['pareto']:
            assert not any(x['instructions']<=r['instructions'] and x['flash_bytes']<=r['flash_bytes'] and (x['instructions']<r['instructions'] or x['flash_bytes']<r['flash_bytes']) for x in eligible)
        records.append(dict(objective=objective,selected=chosen['best']['variant'],instructions=chosen['best']['instructions'],flash_bytes=chosen['best']['flash_bytes'],pareto_members=len(chosen['pareto']),passed=True))
finally:
    subprocess.run(['cargo','run','--quiet','--locked','-p','autotuner','--','tune','workloads/fully_connected.toml','min-instructions'],cwd=ROOT,check=True,stdout=subprocess.DEVNULL)
(ROOT/'results/selection-checks.json').write_text(json.dumps(records,indent=2)+'\n')
print('All four objectives selected correct feasible candidates and valid Pareto members.')
