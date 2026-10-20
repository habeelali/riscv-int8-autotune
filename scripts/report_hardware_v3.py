#!/usr/bin/env python3
"""Publish measured two-model board results; preserve wins and regressions."""
import csv
import hashlib
import json
import math
from pathlib import Path
import statistics as st

ROOT=Path(__file__).resolve().parents[1]
D=ROOT/'results/hardware-v3/physical-audit'
a=json.loads((D/'summary.json').read_text())
assert a['independent_repeatability_evidence'] and a['valid_captures']==3 and not a['emulation_only']
labels=['ESP-NN','muRISCV-NN scalar','muRISCV-NN corrected','Generated Rust']
rows=[];summary=[]
for model,name in [(0,'Person detector'),(1,'CIFAR-10 ResNet')]:
    for b in range(4):
        vals=[next(r['mean_input_median_ms'] for r in c['statistics'] if r['model']==model and r['backend']==b) for c in a['captures']]
        rows.append(dict(model=name,backend=labels[b],median_boot_mean_ms=st.median(vals),minimum_boot_mean_ms=min(vals),maximum_boot_mean_ms=max(vals)))
    ratios=[r['geomean_best_c_over_rust'] for r in a['comparisons'] if r['model']==('person' if model==0 else 'cifar10_resnet')]
    ratio=st.median(ratios)
    summary.append(dict(model=name,median_boot_speedup_vs_fastest_tested_c=ratio,speedup_min=min(ratios),speedup_max=max(ratios),time_reduction_percent=100*(1-1/ratio)))
gmean=math.exp(st.mean(math.log(r['median_boot_speedup_vs_fastest_tested_c']) for r in summary))
with (D/'model-comparison.csv').open('w',newline='') as f:
    w=csv.DictWriter(f,list(rows[0]));w.writeheader();w.writerows(rows)
result=dict(models=summary,geomean_over_two_models=gmean,correctness_checks=752*3,timed_inferences=160*3,delay_calibrations=9,independent_repeatability_evidence=True,boot_type='Three hardware resets; no USB power cycle',aggregation='Per-input median of five observations; mean across four fixtures; median/range across three fresh captures. Speedup is per-boot geometric mean across four fixtures.',scope='One ESP32-C3 SuperMini rev0.4, 160 MHz, two fixed pretrained INT8 graphs, four fixtures each; scalar tested configurations only.')
(D/'comparison.json').write_text(json.dumps(result,indent=2)+'\n')
raw=ROOT/'results/hardware-v3/physical-return'
(D/'returned-file-hashes.json').write_text(json.dumps({str(p.relative_to(raw)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(raw.rglob('*')) if p.is_file()},indent=2)+'\n')
lines=['# Independently triggered board timings: v3','',
'## Findings','',
'All three raw capture sets pass the local auditor in physical mode, using the locally generated tensor oracle rather than executable code from the returned ZIP. There are 2,256 intermediate tensor checks, 480 whole-model timing observations, nine delay calibrations and zero reported mismatches. Host sidecar hashes, every measurement request/response, randomized phase and schedule, and output fingerprints match. The image/model build ID is 1833fe83590e5171.', '',
'The person-detector result repeats across hardware resets: generated Rust takes 259.23 ms versus 376.13 ms for the fastest tested C configuration (the documented corrected scalar muRISCV-NN). This is 1.451×, or 31.08% less inference time. It is a hardware timing result, not a QEMU instruction-count result.', '',
'The second model exposes a regression: CIFAR-10 ResNet takes 682.28 ms in generated Rust versus 620.78 ms for stock scalar muRISCV-NN. Rust takes 9.91% more time, with a speedup ratio of 0.9099×. The QEMU instruction advantage did not predict this hardware ranking. Both models pass bit-exact output checks on every tested intermediate tensor.', '',
'| Model | Implementation | Median boot mean inference time | Range across three boots |','|---|---|---:|---:|']
for r in rows:lines.append(f"| {r['model']} | {r['backend']} | {r['median_boot_mean_ms']:.3f} ms | {r['minimum_boot_mean_ms']:.6f}–{r['maximum_boot_mean_ms']:.6f} ms |")
lines.extend(['',f'The equal-weight geometric mean across both models is **{gmean:.4f}×**, below the original 1.2× target. Neither the losing model nor unfavorable C comparison is excluded.', '',
'## Repeatability evidence','',
'The three captures have distinct host UUIDs, challenges, device identities, measurement schedules, requested phases and timing-sequence hashes. They observe 159,158,158 distinct tick phases respectively. Device/host elapsed-clock ratios are 1.0000362,1.0001059,0.9998492 (within 0.02%). The auditor also checks that the CPU counter agrees with the independent device microsecond timer and that randomized calibration delays match requested time. Earlier duplicate-sequence uncertainty is resolved for these v3 captures by the richer request/response evidence.', '',
'| Model | Boot 1 C/Rust | Boot 2 C/Rust | Boot 3 C/Rust |','|---|---:|---:|---:|'])
for m,n in [('person','Person detector'),('cifar10_resnet','CIFAR-10 ResNet')]:
    ratios=[r['geomean_best_c_over_rust'] for r in a['comparisons'] if r['model']==m]
    lines.append('| '+n+' | '+' | '.join(f'{v:.6f}×' for v in ratios)+' |')
lines.extend(['',
'The local timeline records one post-flash reset and two esptool RTS hard resets, with USB power unchanged. There was no physical power-cycle capture. These are fresh reset sessions on one board, not a cold-power or cross-board study. The small observed ranges are reported without invented confidence intervals. Protocol consistency is not cryptographic hardware attestation.', '',
'## Implication for optimization','',
'The cache-aware pointwise schedule is retained as a hardware-validated person-detector selection. It does not establish a general Rust runtime win. The new model has predominantly 3×3 convolutions with residual additions and a different weight/cache working set. Its current schedule was selected by QEMU guest instructions. That is a plausible source of the hardware ranking mismatch, but there is no per-layer hardware timing in this v3 capture to attribute the loss to a specific layer or transformation. V3_CHECK records are correctness checks, not timed layer profiles. The next optimization should use host-triggered per-layer timing and compare spatial tiling, im2col cost and weight reuse while retaining the current correct sources and all negative results.', '',
'## Scope and reproducibility','',
'ESP32-C3 SuperMini rev0.4, 4 MB XMC flash, CPU160 MHz, flash DIO40 MHz, USB-powered, RTOS interrupts enabled. Four fixed fixtures per model; warm inference; no sensors or accuracy evaluation. The model outputs include the reference classifier mistakes documented in the model manifests. Backends are specifically built scalar ESP-NN, scalar muRISCV-NN, its separately documented corrected optional requantization configuration, and generated no_std Rust learned kernels. Common C graph dispatch/padding/pooling/reshape/softmax/residual-add code is shared. No comparison to CubeCL, MicroFlow, RVV/DSP acceleration or all available competitor configurations is implied.', '',
'This capture does not measure standalone ESP-IDF flash or isolated stack per backend; the image contains multiple implementations and reference tensors. The stack watermarks are cumulative. Earlier QEMU-only deployment size numbers remain separately labeled and must not be presented as board deployment size.', '',
'Reproduce: run hardware/esp32c3-v3/audit.py on the three original capture prefixes with hardware/esp32c3-v3/main/generated/oracle.json, without --emulator, then `python3 scripts/report_hardware_v3.py`. Original returned files are preserved in ../physical-return/. The local audit summary and file hashes accompany this report.'])
(D/'evaluation.md').write_text('\n'.join(lines)+'\n')

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
fig,axes=plt.subplots(1,2,figsize=(10,4.8))
for ax,(model,name) in zip(axes,[(0,'Person detector'),(1,'CIFAR-10 ResNet')]):
    rr=[r for r in rows if r['model']==name]
    vals=[r['median_boot_mean_ms'] for r in rr]
    colors=['#aeb6bf','#d99a38','#d99a38','#198754' if model==0 else '#bf5250']
    ax.barh(labels,vals,color=colors);ax.invert_yaxis();ax.set_xlim(0,max(vals)*1.2)
    for i,x in enumerate(vals):ax.text(x+max(vals)*.015,i,f'{x:.1f}',va='center')
    ax.set_title(name+('\nRust 1.451×; 31.1% less time' if model==0 else '\nRust 0.910×; 9.9% more time'))
    ax.set_xlabel('Inference time (ms), lower is better')
fig.suptitle('ESP32-C3: three audited reset sessions, two pretrained INT8 models',fontsize=12)
fig.text(.02,.015,'Median across boot means of per-input medians. All correctness checks pass. One board; no power-cycle study.',fontsize=8)
fig.tight_layout(rect=(0,.04,1,.94));fig.savefig(D/'two-model-comparison.png',dpi=180);fig.savefig(D/'two-model-comparison.svg');plt.close(fig)
print(json.dumps(result,indent=2))
