#!/usr/bin/env python3
"""Audit the returned raw ESP32-C3 logs; do not pool duplicate captures."""
from pathlib import Path
import csv
import hashlib
import json
import math
import re
import statistics as st

ROOT = Path(__file__).resolve().parents[1]
BASE = ROOT / 'results/hardware-esp32c3'
SOURCE = BASE / 'physical-return'
OUT = BASE / 'analysis'
OUT.mkdir(exist_ok=True)
BACKENDS = {'esp_nn_native', 'mu_scalar', 'mu_portable', 'mu_fixed',
            'mu_portable_fixed', 'rust_compact', 'rust_candidate'}

def parse(path):
    records = []
    for line in path.read_text(errors='replace').splitlines():
        match = re.search(r'(HW_[A-Z_]+) (\{.*\})', line)
        if match:
            records.append(dict(type=match[1], **json.loads(match[2])))
    return records

captures = {}
for path in sorted(SOURCE.glob('boot-*.log')):
    records = parse(path)
    layers = [r for r in records if r['type'] == 'HW_LAYER']
    runs = [r for r in records if r['type'] == 'HW_RUN']
    starts = [r for r in records if r['type'] == 'HW_BEGIN']
    ends = [r for r in records if r['type'] == 'HW_DONE']
    valid_keys = ({(r['backend'], r['sample'], r['layer']) for r in layers}
                  == {(b, s, l) for b in BACKENDS for s in range(4) for l in range(31)}
                  and {(r['backend'], r['sample'], r['round']) for r in runs}
                  == {(b, s, n) for b in BACKENDS for s in range(4) for n in range(7)})
    complete = len(starts) == len(ends) == 1 and len(layers) == 868 and len(runs) == 196 and valid_keys
    captures[path.name] = dict(sha256=hashlib.sha256(path.read_bytes()).hexdigest(),
        complete=complete, layers=len(layers), runs=len(runs),
        correct=bool(complete and ends[0].get('correct') and ends[0].get('mismatches_total') == 0
                     and all(r['mismatches'] == 0 for r in layers + runs)),
        records=records)

# Boot 3 is the first complete capture. Preserve all other captures for audit,
# but do not multiply the effective sample count with near-identical logs.
records = captures['boot-3.log']['records']
assert captures['boot-3.log']['correct']
assert next(r for r in records if r['type'] == 'HW_BEGIN')['build_id'] == '4f8d4bb97f51a2c0'
runs = [r for r in records if r['type'] == 'HW_RUN']
rows = []
medians = {}
for b in sorted(BACKENDS):
    meds = []
    for s in range(4):
        values = [r['cycles_raw'] for r in runs if r['backend'] == b and r['sample'] == s]
        median = st.median(values)
        meds.append(median)
        rows.append(dict(backend=b, sample=s, n=len(values), min_cycles=min(values),
                         median_cycles=median, max_cycles=max(values), median_ms=median/160000))
    medians[b] = meds
summary = {b: dict(mean_sample_median_cycles=st.mean(v), mean_sample_median_ms=st.mean(v)/160000)
           for b, v in medians.items()}
gmean = lambda x: math.exp(st.mean(math.log(v) for v in x))
ratios = dict(compact_over_candidate=gmean([a/b for a,b in zip(medians['rust_compact'], medians['rust_candidate'])]),
              best_c_over_candidate=gmean([min(medians[b][s] for b in BACKENDS if b.startswith('mu_') or b == 'esp_nn_native')/medians['rust_candidate'][s] for s in range(4)]))
comparisons = []
for name, capture in captures.items():
    bykey = {(r['type'],r.get('backend'),r.get('sample'),r.get('layer'),r.get('round')): r for r in capture['records']}
    pairs = [(r, bykey[k]) for r in records if 'cycles_raw' in r and
             (k := (r['type'],r.get('backend'),r.get('sample'),r.get('layer'),r.get('round'))) in bykey]
    comparisons.append(dict(capture=name, comparable_timing_records=len(pairs),
                            identical_cycle_counts=sum(a['cycles_raw']==b['cycles_raw'] for a,b in pairs)))
    del capture['records']
result = dict(selected_capture='boot-3.log', captures=captures, duplicate_audit=comparisons,
              backends=summary, geometric_mean_ratios=ratios,
              provenance_limit='Boots 3 and 4 are byte-identical; boot 5 differs in one timing record. Independence unconfirmed; no pooling.',
              scope='Returned physical-board logs, interrupts enabled, one model, four fixtures, warm inference, seven repetitions.')
(OUT/'audit.json').write_text(json.dumps(result,indent=2)+'\n')
with (OUT/'per-sample.csv').open('w',newline='') as f:
    w=csv.DictWriter(f,list(rows[0]));w.writeheader();w.writerows(rows)

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
order=sorted(summary,key=lambda b:summary[b]['mean_sample_median_cycles'])
labels={'esp_nn_native':'ESP-NN', 'mu_scalar':'muRISCV-NN scalar', 'mu_fixed':'muRISCV-NN fixed',
        'mu_portable':'muRISCV-NN portable','mu_portable_fixed':'muRISCV-NN portable fixed',
        'rust_compact':'Previous Rust','rust_candidate':'New Rust'}
fig,ax=plt.subplots(figsize=(9,4.8))
v=[summary[b]['mean_sample_median_ms'] for b in order]
ax.barh([labels[b] for b in order],v,color=['#176b45' if b=='mu_scalar' else '#2878b5' if b=='rust_candidate' else '#aeb6bf' for b in order])
for i,x in enumerate(v):ax.text(x+7,i,f'{x:.1f} ms',va='center')
ax.invert_yaxis();ax.set_xlim(0,max(v)*1.18);ax.set_xlabel('Mean of four sample medians; seven runs per sample (lower is better)')
ax.set_title('ESP32-C3 @ 160 MHz — INT8 person detector\nReturned boot-3 log; interrupts enabled')
fig.text(.02,.015,'Capture independence unresolved: duplicate logs excluded. Warm inference; not an accuracy evaluation.',fontsize=8)
fig.tight_layout(rect=(0,.04,1,1));fig.savefig(OUT/'hardware-comparison.png',dpi=180);fig.savefig(OUT/'hardware-comparison.svg');plt.close(fig)
lines=['# Returned ESP32-C3 measurement audit','',
'ESP32-C3 SuperMini, chip revision 0.4, 160 MHz, USB-powered, physical flash 4 MB; supplied image configured for 2 MB. Identification and flashing transcripts are retained in `../physical-return/`.', '',
'Boot 3 is the first complete capture: 868 unique layer checks and 196 unique timed runs, all zero mismatches, expected firmware build ID. Boots 4 and 5 also contain complete correct records. Boot 1 lacks early records and boot 2 is empty; neither is used.', '',
'Boots 3 and 4 have identical SHA-256 hashes. Boot 5 differs in only one timing record. Boot 1 also shares 1052 of 1054 comparable cycle counts with boot 3. These captures are not treated as independent repetitions. This is a provenance question to resolve before making repeated-boot stability claims; it does not identify the cause.', '',
'| Implementation | Mean sample-median cycles | Equivalent ms at 160 MHz |','|---|---:|---:|']
for b in order:lines.append(f"| {labels[b]} | {summary[b]['mean_sample_median_cycles']:,.1f} | {summary[b]['mean_sample_median_ms']:.2f} |")
lines.extend(['',f"Geometric mean across four fixtures: previous Rust / candidate = {ratios['compact_over_candidate']:.4f}×; fastest C / candidate = {ratios['best_c_over_candidate']:.4f}×.", '',
f"The candidate reduces cycles versus previous Rust by {100*(1-1/ratios['compact_over_candidate']):.2f}%, but takes {100*(1/ratios['best_c_over_candidate']-1):.2f}% more cycles than fastest C. Thus the hardware target of 1.2× versus C has not been met.", '',
'These are reported board cycle-counter measurements, not QEMU instruction counts. Time is converted from cycles at 160 MHz; logged wall-clock values agree with that frequency. QEMU measured instruction reductions do not establish hardware cycle reductions: instruction latency, cache behavior and code placement can change ranking. A controlled hardware ablation is needed to attribute the gap.', '',
'Correctness means bit-exact agreement with the stored model oracle on four fixtures and all intermediate tensors. It does not establish classification accuracy or correctness for arbitrary inputs. Interrupts remain enabled; no confidence intervals are claimed. The combined seven-backend diagnostic image cannot establish standalone deployment flash savings.', '',
'Reproduce: `python3 scripts/analyze_hardware_return.py` (requires matplotlib). Raw uploaded logs are retained without editing.'])
(OUT/'report.md').write_text('\n'.join(lines)+'\n')
print(json.dumps(result,indent=2))
