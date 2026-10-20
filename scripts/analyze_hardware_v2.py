#!/usr/bin/env python3
"""Independently audit v2 raw captures; report observed timings without pooling."""
from pathlib import Path
import csv
import hashlib
import json
import math
import re
import statistics as st
import subprocess
import sys

ROOT=Path(__file__).resolve().parents[1]
BASE=ROOT/'results/hardware-cache'
SRC=BASE/'physical-return'
OUT=BASE/'analysis'
OUT.mkdir(exist_ok=True)
manifest=json.loads((SRC/'manifest.json').read_text())
assert manifest['build_id']=='6b800894f7429a74'
expected=json.loads((BASE/'emulator-functional-check.json').read_text())
assert manifest['build_id']==expected['build_id']
local_image=Path('/workspace/esp32c3-int8-cache-v2/esp32c3-int8-cache-v2.bin')
if local_image.exists():assert hashlib.sha256(local_image.read_bytes()).hexdigest()==manifest['image_sha256']
backends=set(manifest['backends'])
expected_layer_keys={(b,s,l) for b in backends for s in range(4) for l in range(31)}
expected_run_keys={(b,s,n) for b in backends for s in range(4) for n in range(7)}
captures={}
all_records={}
for f in sorted(SRC.glob('boot-*.log')):
    dest=OUT/f.stem
    subprocess.run([sys.executable,str(ROOT/'hardware/esp32c3/summarize.py'),str(f),'--output',str(dest)],check=True,stdout=subprocess.DEVNULL)
    parsed=json.loads((dest/'summary.json').read_text())
    records=json.loads((dest/'records.json').read_text())
    layers=[r for r in records if r['type']=='HW_LAYER']
    runs=[r for r in records if r['type']=='HW_RUN']
    valid_keys=({(r['backend'],r['sample'],r['layer']) for r in layers}==expected_layer_keys and
                {(r['backend'],r['sample'],r['round']) for r in runs}==expected_run_keys)
    sha=hashlib.sha256(f.read_bytes()).hexdigest()
    meta=json.loads(f.with_suffix('.capture.json').read_text())
    assert meta['log_sha256']==sha and meta['log_bytes']==f.stat().st_size
    if parsed['complete']:
        assert parsed['correct'] and valid_keys and len(parsed['capture_identities'])==1
        assert parsed['start'][0]['build_id']==manifest['build_id']
        assert parsed['start'][0]['cpu_hz']==160000000
        assert parsed['start'][0]['model_sha256']=='808cfdfc0cf3a6fa6f6fa26bfa379ea97c16d5db7334637766e39c3408502e9d'
        assert not re.search(r'HW_ERROR|Guru Meditation|panic|watchdog',f.read_text(),re.I)
        all_records[f.name]=records
    captures[f.name]=dict(complete=parsed['complete'],correct=parsed['correct'],
        layer_records=len(layers),run_records=len(runs),sha256=sha,
        capture_identities=parsed['capture_identities'],host_capture_uuid=meta['host_capture_uuid'])

assert set(all_records)=={'boot-1.log','boot-2-retry.log','boot-3.log'}
identities=[tuple((r['boot_counter'],r['nonce_hi'],r['nonce_lo']))
            for records in all_records.values() for r in records if r['type']=='HW_CAPTURE']
assert len(set(identities))==3
selected=all_records['boot-1.log']
runs=[r for r in selected if r['type']=='HW_RUN']
comparisons=[]
for name,records in all_records.items():
    bykey={(r['type'],r.get('backend'),r.get('sample'),r.get('layer'),r.get('round')):r for r in records}
    pairs=[(r,bykey[(r['type'],r.get('backend'),r.get('sample'),r.get('layer'),r.get('round'))])
           for r in selected if 'cycles_raw' in r]
    comparisons.append(dict(capture=name,identical_run_cycles=sum(a['cycles_raw']==b['cycles_raw'] for a,b in pairs if a['type']=='HW_RUN'),
        total_run_cycles=252,identical_layer_cycles=sum(a['cycles_raw']==b['cycles_raw'] for a,b in pairs if a['type']=='HW_LAYER'),total_layer_cycles=1116))
rows=[];medians={}
for b in sorted(backends):
    medians[b]=[]
    for s in range(4):
        rr=[r for r in runs if r['backend']==b and r['sample']==s]
        values=[r['cycles_raw'] for r in rr];median=st.median(values);medians[b].append(median)
        rows.append(dict(backend=b,sample=s,n=7,unique_cycle_counts=len(set(values)),min_cycles=min(values),median_cycles=median,max_cycles=max(values),median_ms=median/160000,
                         median_logged_wall_ms=st.median(r['wall_us'] for r in rr)/1000))
summary={b:dict(mean_sample_median_cycles=st.mean(v),mean_sample_median_ms=st.mean(v)/160000) for b,v in medians.items()}
gmean=lambda v:math.exp(st.mean(math.log(x) for x in v))
c_names=[b for b in backends if b.startswith('mu_') or b=='esp_nn_native']
ratios={}
for b in ('rust_cache2','rust_cache4'):
    vs_c=gmean([min(medians[c][s] for c in c_names)/medians[b][s] for s in range(4)])
    vs_old=gmean([medians['rust_candidate'][s]/medians[b][s] for s in range(4)])
    ratios[b]=dict(speedup_vs_fastest_tested_c=vs_c,speedup_vs_previous_candidate=vs_old,
        time_reduction_vs_fastest_tested_c_percent=100*(1-1/vs_c),time_reduction_vs_previous_candidate_percent=100*(1-1/vs_old))
layer_rows=[]
for l in range(31):
    row=dict(layer=l)
    for b in ['mu_scalar','rust_candidate','rust_cache4','rust_cache2']:
        row[b]=st.mean(r['cycles_raw'] for r in selected if r['type']=='HW_LAYER' and r['backend']==b and r['layer']==l)
    layer_rows.append(row)
for name,data in [('per-sample',rows),('layers',layer_rows)]:
    with (OUT/(name+'.csv')).open('w',newline='') as f:
        w=csv.DictWriter(f,list(data[0]));w.writeheader();w.writerows(data)
result=dict(selected_capture='boot-1.log',aggregation='Mean of four input medians; seven observations per input. Other boots not pooled.',
            captures=captures,distinct_device_identities=len(set(identities)),cycle_repetition_audit=comparisons,
            backends=summary,ratios=ratios,
            qualification='All 252 matched whole-model cycle counts are identical across successful captures, and each backend/input has one unique cycle value across seven rounds. Distinct boot identifiers do not establish statistical independence; no confidence intervals or independent 21-repetition claim.',
            scope='User-returned serial logs for one ESP32-C3 SuperMini, one fixed INT8 person detector, four fixtures; fixed 160 MHz, interrupts enabled, warm inference. Not an accuracy evaluation or general library comparison.')
(OUT/'audit.json').write_text(json.dumps(result,indent=2)+'\n')

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
labels={'esp_nn_native':'ESP-NN baseline','mu_scalar':'muRISCV-NN scalar','mu_fixed':'muRISCV-NN corrected',
        'mu_portable':'muRISCV-NN portable','mu_portable_fixed':'muRISCV-NN portable corrected',
        'rust_compact':'Earlier compact Rust','rust_candidate':'Previous Rust candidate',
        'rust_cache4':'Rust cache4','rust_cache2':'Rust cache2 (selected)'}
order=sorted(summary,key=lambda b:summary[b]['mean_sample_median_cycles'])
fig,ax=plt.subplots(figsize=(10,5.5));v=[summary[b]['mean_sample_median_ms'] for b in order]
ax.barh([labels[b] for b in order],v,color=['#198754' if b=='rust_cache2' else '#399acf' if b=='rust_cache4' else '#d49037' if b=='mu_scalar' else '#acb5c0' for b in order])
for i,x in enumerate(v):ax.text(x+7,i,f'{x:.1f} ms',va='center')
ax.invert_yaxis();ax.set_xlim(0,max(v)*1.17)
ax.set_title('Returned ESP32-C3 v2 measurements — INT8 person detector\nRust cache2: 1.424× versus fastest tested C; 29.8% less time')
ax.set_xlabel('Mean of four input medians at 160 MHz; lower is better')
fig.text(.02,.015,'Boot 1 only. Repeated captures have identical whole-model cycle sequences; no confidence interval claimed.',fontsize=8)
fig.tight_layout(rect=(0,.04,1,1));fig.savefig(OUT/'hardware-comparison.png',dpi=180);fig.savefig(OUT/'hardware-comparison.svg');plt.close(fig)
large=[14,16,18,20,22,24,26]
fig,ax=plt.subplots(figsize=(9,4.5))
for k,(b,col) in enumerate([('rust_candidate','#acb5c0'),('mu_scalar','#d49037'),('rust_cache2','#198754')]):
    ax.bar([i+(k-1)*.25 for i in range(len(large))],[layer_rows[l][b]/1e6 for l in large],width=.25,label=labels[b],color=col)
ax.set_xticks(range(len(large)),large);ax.set_xlabel('Large pointwise layer index');ax.set_ylabel('Million cycles (diagnostic layer pass)');ax.legend();ax.set_title('Channel-first traversal targets the large-layer bottleneck')
fig.tight_layout();fig.savefig(OUT/'large-layer-comparison.png',dpi=180);plt.close(fig)

lines=['# ESP32-C3 v2 returned-log evaluation','',
'## Observed result','',
'The returned serial logs show the new cache2 Rust implementation at 257.99 ms versus 367.25 ms for the fastest tested C configuration, scalar muRISCV-NN. Across the four fixture inputs, the geometric-mean speedup is 1.4235×, a 29.75% reduction in inference time. The previous Rust candidate in the same v2 image takes 442.84 ms; cache2 reduces its time by 41.74%. The 1.2× target is exceeded in these returned measurements, subject to the capture qualification below.', '',
'| Implementation | Mean sample-median cycles | Equivalent inference time |','|---|---:|---:|']
for b in order:lines.append(f"| {labels[b]} | {summary[b]['mean_sample_median_cycles']:,.1f} | {summary[b]['mean_sample_median_ms']:.2f} ms |")
lines.extend(['',
'Cycles are from the ESP32-C3 CPU counter, converted at the logged 160 MHz frequency. Logged esp_timer wall times agree. Both new variants beat the fastest C on every fixture; the cache4 geometric-mean speedup is 1.3735×. Comparisons use the same v2 image and graph harness. The old candidate and C timings changed slightly from v1 due to image/graph/code placement, so v2-to-v2 is the headline comparison.', '',
'## Correctness and capture audit','',
'Independently parsed raw logs: three complete captures, each with exactly 1,116 unique backend/input/layer keys and 252 unique backend/input/round keys; all reported mismatches are zero and the firmware/model identifiers match. The failed automatic-reset attempt is empty and excluded. Host sidecar log hashes match raw bytes; supplied image identification matches the locally built image. All successful logs have distinct HW_CAPTURE boot identifiers and host UUIDs.', '',
'However, all 252 whole-model cycle counts match exactly across all three captures; 1,113 of 1,116 layer counts match boot 1 in boot 2 retry. Each backend/input also has identical cycle counts in all seven rounds. This unusual determinism is retained explicitly: nonce uniqueness is not proof of independent timing evidence. We use boot 1 for the table and do not pool captures, report confidence intervals, or claim 21 independent observations. A direct capture with per-observation host timestamps and varied tick phase would strengthen the evidence. This audit does not establish why the repetition occurs.', '',
'## What changed','',
'Only large pointwise layers 14,16,18,20,22,24,26 switch to channel-first packed traversal. Each small weight block is reused across spatial tiles before advancing to the next channel block. cache2 uses two output channels and four pixels per tile; cache4 uses four channels and two pixels. This reduces the active weight working set compared with repeatedly sweeping entire 16–64 KB matrices. Arithmetic, quantization, activation and expected intermediate tensors are unchanged. These are established loop/data-layout techniques; no novelty is claimed merely for implementing them in Rust. Cache behavior is a supported hypothesis, not a hardware-counter attribution study.', '',
'The learned kernels remain scalar no_std Rust with no heap during inference. Common graph dispatch, padding, pooling, reshape and softmax are C, shared by all backends. Thus this is a Rust-kernel result for a mixed runtime, not an entirely Rust neural-network runtime.', '',
'## Memory and scope','',
'Previously executed standalone QEMU deployment builds measure cache2 at 281,880 binary bytes versus 282,452 for the prior candidate (572 bytes smaller), with unchanged 121,360 bytes static RAM including the held-out input and 708-byte measured maximum stack. These are QEMU-linked deployment sizes, not standalone ESP-IDF board-image sizes. The 4 MB combined diagnostic contains nine backends, duplicate parameter layouts and reference tensors and must not be used as a single-model size claim.', '',
'One ESP32-C3 SuperMini rev0.4, 4 MB flash, 160 MHz CPU, DIO40 MHz, USB power, RTOS interrupts enabled. One pretrained INT8 96×96 person detector and four fixtures, including two mirrored images; not dataset accuracy or cross-model evidence. ESP-NN and muRISCV-NN here are the specifically built scalar configurations. This is not evidence of beating CubeCL, MicroFlow, accelerator/vector implementations, or every ESP-NN target-specific backend.', '',
'Reproduce parsing, audits, tables and plots: `python3 scripts/analyze_hardware_v2.py`. Raw returned files remain in `../physical-return/`.'])
(OUT/'report.md').write_text('\n'.join(lines)+'\n')
print(json.dumps(dict(ratios=ratios,cycle_repetition_audit=comparisons),indent=2))
