#!/usr/bin/env python3
"""Summarize locally audited V5 physical captures without changing raw evidence."""
import argparse
import csv
import hashlib
import json
import math
from pathlib import Path
import statistics as st

ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--image', type=Path, help='Optionally verify a locally available merged firmware image')
args = parser.parse_args()
OUT = ROOT / 'results/hardware-v5/physical-audit'
RAW = OUT.parent / 'physical-return'
audit = json.loads((OUT / 'summary.json').read_text())
assert not audit['emulation_only']
assert audit['valid_captures'] == 3 and audit['failed_captures'] == 0
assert audit['independent_repeatability_evidence']
local = ROOT / 'hardware/esp32c3-v5/main/generated'
assert (RAW/'bundle-reference/oracle.json').read_bytes() == (local/'oracle.json').read_bytes()
manifest = json.loads((RAW/'bundle-reference/manifest.json').read_text())
assert manifest['build_id'] == audit['build_id'] == 'cdd27710ec8a49a9'
if args.image:
    assert hashlib.sha256(args.image.read_bytes()).hexdigest() == manifest['image_sha256']
# Check archive inventory as data; never execute returned scripts.
for name, record in json.loads((RAW/'inventory.sha256.json').read_text()).items():
    path = (RAW/name).resolve()
    assert path.is_relative_to(RAW.resolve())
    assert path.stat().st_size == record['bytes']
    assert hashlib.sha256(path.read_bytes()).hexdigest() == record['sha256'], name
rows = []
for cap in audit['captures']:
    for model, name in [(0, 'Person detector'), (1, 'CIFAR-10 ResNet')]:
        group = {s['backend']: s for s in cap['statistics'] if s['model'] == model}
        c = [min(group[b]['input_median_cycles'][s] for b in range(3)) / 160000 for s in range(4)]
        rust = group[3]['mean_input_median_ms']
        ratio = math.exp(st.mean(math.log(c[s] / (group[3]['input_median_cycles'][s]/160000)) for s in range(4)))
        rows.append(dict(boot=Path(cap['prefix']).name, model=name, best_tested_c_ms=st.mean(c), rust_ms=rust, c_over_rust=ratio))
models = []
for name in ['Person detector', 'CIFAR-10 ResNet']:
    r = [r for r in rows if r['model'] == name]
    ratios = [x['c_over_rust'] for x in r]
    times = [x['rust_ms'] for x in r]
    models.append(dict(model=name, best_tested_c_ms=st.mean(x['best_tested_c_ms'] for x in r), rust_ms=st.mean(times), rust_ms_min=min(times), rust_ms_max=max(times), rust_boot_range_percent=100*(max(times)-min(times))/st.mean(times), c_over_rust=math.exp(st.mean(math.log(x) for x in ratios)), ratio_min=min(ratios), ratio_max=max(ratios)))
geo = math.sqrt(math.prod(x['c_over_rust'] for x in models))
result = dict(build_id=audit['build_id'], image_sha256=manifest['image_sha256'], valid_captures=3, tensor_checks=2256, timed_inferences=480, mismatches=0, independent_reset_repeatability=True, physical_power_cycles=False, models=models, equal_model_geomean=geo, limitations=['One board, three hard-reset sessions; not power cycles or long-duration reliability.', 'Fixed schedules; four previously used validation fixtures per model, not a new held-out dataset or accuracy study.', 'Common C graph operators remain in both implementations; this is not a language-only comparison.', 'V5 QEMU stalls and earlier V4/V4.1 hardware stalls remain undiagnosed. Passing hardware captures do not prove the emulator is at fault.', 'The multi-backend diagnostic image does not establish per-backend production flash size.'])
(OUT/'comparison.json').write_text(json.dumps(result, indent=2)+'\n')
with (OUT/'model-comparison.csv').open('w', newline='') as f:
    writer=csv.DictWriter(f,list(rows[0]));writer.writeheader();writer.writerows(rows)
(OUT/'returned-file-hashes.json').write_text(json.dumps({str(p.relative_to(RAW)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(RAW.rglob('*')) if p.is_file()},indent=2)+'\n')
lines=['# V5: repeatable physical complete-model inference gains', '', 'The local V5 auditor independently accepted all three returned raw captures against the local build oracle, without emulator exemptions. All 2,256 intermediate tensor checks and 480 timed complete-model inferences passed, with zero mismatches. Exact command plans, fresh challenges, raw/event hashes, clock calibration and host/device elapsed-time agreement passed. Returned scripts were not executed.', '', '| Model | Fastest tested C | Fixed Rust | C/Rust | Rust range across resets |', '|---|---:|---:|---:|---:|']
for m in models:
    lines.append(f"| {m['model']} | {m['best_tested_c_ms']:.3f} ms | {m['rust_ms']:.3f} ms | {m['c_over_rust']:.4f}× | {m['rust_ms_min']:.6f}–{m['rust_ms_max']:.6f} ms |")
lines += ['', f'Equal-model geometric mean: **{geo:.4f}×** against the fastest tested C baseline per input. Each boot uses four input medians (five timed observations per input/backend); times above average the input medians equally across the three boots. Ratios geometrically average per-input ratios, then boots and models. This is physical elapsed inference timing at 160 MHz, supported by cycle/wall-clock checks, not QEMU instruction counts. No confidence interval is inferred from three boots.', '', 'Fastest C is corrected muRISCV-NN for person detection and stock scalar muRISCV-NN for ResNet. ESP-NN is also measured in the same image; it is slower than those controls.', '', 'The largest Rust boot-to-boot range of the averaged input medians is '+f"{max(m['rust_boot_range_percent'] for m in models):.5f}%"+'. This describes these observations only, not individual-run jitter or a population uncertainty estimate.', '', '## Scope and unresolved failure', '', 'All three sessions used the same firmware, native USB and one ESP32-C3 SuperMini, with documented esptool hard resets. They were not physical power cycles. Schedules were fixed before capture; V5 has no hardware search or staging backend. These are previously used validation fixtures, not a new independent accuracy set.', '', 'This is the first independently repeated two-model physical win in this project. It supersedes V3’s ResNet regression for this fixed V5 implementation. It does not promote the faster, staging-dependent V4.1 mixed selection, which failed on a later reset.', '', 'The intermittent V5 simulation stalls are still real unresolved failures. Successful board captures show this image completed these three hardware sessions; they neither identify a root cause nor justify declaring QEMU responsible. Earlier V4/V4.1 failures and all incomplete simulation attempts remain preserved. Further debugging must capture a genuinely stalled PC/stack before attributing the fault.', '', 'The diagnostic firmware includes all C/Rust backends, fixtures and golden tensors. Its 4 MiB merged flash file includes partition padding; neither that size nor whole-ELF size is an isolated kernel flash measurement. No new flash-saving claim is made.', '', '[Timing graph](model-comparison.svg) · [per-boot comparison](model-comparison.csv) · [machine-readable results](comparison.json) · [strict raw audit](summary.json)']
(OUT/'evaluation.md').write_text('\n'.join(lines)+'\n')
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
fig, axes=plt.subplots(1,2,figsize=(10,4.5),layout='constrained')
for ax,m in zip(axes,models):
    vals=[m['best_tested_c_ms'], m['rust_ms']]
    ax.barh(['Fastest tested C','Fixed Rust'],vals,color=['#797f88','#159c86'])
    ax.invert_yaxis();ax.set_xlim(0,max(vals)*1.25);ax.set_xlabel('Complete inference (ms; lower is better)')
    ax.set_title(f"{m['model']} · {m['c_over_rust']:.3f}×")
    for i,v in enumerate(vals):ax.text(v+max(vals)*.025,i,f'{v:.2f} ms',va='center')
fig.suptitle('ESP32-C3 · three audited hard-reset sessions\n2,256 tensor checks · 480 timed inferences · zero mismatches')
for suffix in ['svg','png']:fig.savefig(OUT/f'model-comparison.{suffix}',dpi=160)
plt.close(fig)
print(json.dumps(result,indent=2))
