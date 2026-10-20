#!/usr/bin/env python3
"""Parse actual hardware logs. Emulator records must not be reported as hardware."""
import argparse
import csv
import json
import math
from pathlib import Path
import re
import statistics

parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('log',type=Path)
parser.add_argument('--output',type=Path,default=Path('hardware-results'))
args=parser.parse_args();args.output.mkdir(parents=True,exist_ok=True)
records=[];parse_errors=[]
for number,line in enumerate(args.log.read_text(errors='replace').splitlines(),1):
    match=re.search(r'(HW_[A-Z_]+) (\{.*\})',line)
    if not match:continue
    try:records.append(dict(type=match[1],**json.loads(match[2])))
    except json.JSONDecodeError as e:parse_errors.append(dict(line=number,error=str(e)))
starts=[r for r in records if r['type']=='HW_BEGIN'];ends=[r for r in records if r['type']=='HW_DONE']
layers=[r for r in records if r['type']=='HW_LAYER'];runs=[r for r in records if r['type']=='HW_RUN']
infos=[r for r in records if r['type']=='HW_INFO']
identities=[r for r in records if r['type']=='HW_CAPTURE']
expected_backends=infos[0].get('backends',7) if len(infos)==1 else 7
expected_samples=infos[0].get('samples',4) if len(infos)==1 else 4
expected_ops=infos[0].get('operators',31) if len(infos)==1 else 31
expected_repeats=infos[0].get('repeats',7) if len(infos)==1 else 7
names={r['backend'] for r in runs}
layer_keys={(r['backend'],r['sample'],r['layer']) for r in layers}
run_keys={(r['backend'],r['sample'],r['round']) for r in runs}
keys_complete=(len(names)==expected_backends and
    layer_keys=={(b,s,l) for b in names for s in range(expected_samples) for l in range(expected_ops)} and
    run_keys=={(b,s,n) for b in names for s in range(expected_samples) for n in range(expected_repeats)})
stats=[]
for backend in sorted({r['backend'] for r in runs}):
    for sample in sorted({r['sample'] for r in runs if r['backend']==backend}):
        rows=[r for r in runs if r['backend']==backend and r['sample']==sample]
        cycles=[r['cycles_raw'] for r in rows]
        stats.append(dict(backend=backend,sample=sample,n=len(rows),min_cycles=min(cycles),
                          median_cycles=statistics.median(cycles),max_cycles=max(cycles),
                          median_wall_us=statistics.median(r['wall_us'] for r in rows),
                          mismatches=sum(r['mismatches'] for r in rows)))
ratios=[]
for sample in sorted({r['sample'] for r in stats}):
    group={r['backend']:r for r in stats if r['sample']==sample}
    if 'rust_candidate' in group and 'rust_compact' in group:
        c=[r['median_cycles'] for n,r in group.items() if n.startswith('mu_') or n=='esp_nn_native']
        ratios.append(dict(sample=sample,compact_over_candidate=group['rust_compact']['median_cycles']/group['rust_candidate']['median_cycles'],
                           best_c_over_candidate=min(c)/group['rust_candidate']['median_cycles'] if c else None))
for sample in sorted({r['sample'] for r in stats}):
    group={r['backend']:r for r in stats if r['sample']==sample}
    for name in sorted(n for n in group if n.startswith('rust_cache')):
        c=[r['median_cycles'] for n,r in group.items() if n.startswith('mu_') or n=='esp_nn_native']
        ratios.append(dict(sample=sample,backend=name,
            previous_candidate_over_cache=group['rust_candidate']['median_cycles']/group[name]['median_cycles'],
            best_c_over_cache=min(c)/group[name]['median_cycles'] if c else None))
complete=(len(starts)==len(ends)==1 and len(layers)==expected_backends*expected_samples*expected_ops
          and len(runs)==expected_backends*expected_samples*expected_repeats and keys_complete and not parse_errors
          and starts[0].get('build_id')==ends[0].get('build_id')
          and ends[0].get('layer_records')==len(layers) and ends[0].get('run_records')==len(runs))
correct=complete and ends[0].get('correct') and ends[0].get('mismatches_total')==0 and all(r.get('mismatches',0)==0 for r in [*layers,*runs])
summary=dict(complete=complete,correct=bool(correct),start=starts,end=ends,layer_records=len(layers),run_records=len(runs),
             parse_errors=parse_errors,statistics=stats,ratios=ratios,capture_identities=identities,
             measurement='Raw board cycle counter with RTOS interrupts; emulator logs are functional tests only')
(args.output/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
(args.output/'records.json').write_text(json.dumps(records,indent=2)+'\n')
for name,rows in [('runs',runs),('layers',layers),('statistics',stats)]:
    if rows:
        with (args.output/(name+'.csv')).open('w',newline='') as f:
            writer=csv.DictWriter(f,list(rows[0]),lineterminator='\n');writer.writeheader();writer.writerows(rows)
print(json.dumps(dict(complete=complete,correct=bool(correct),layer_records=len(layers),run_records=len(runs),output=str(args.output)),indent=2))
