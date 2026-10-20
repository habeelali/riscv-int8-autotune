#!/usr/bin/env python3
"""Report audited single-session held-out gains alongside the failed reset."""
import csv,hashlib,json,math,statistics as st
from pathlib import Path
R=Path(__file__).resolve().parents[1];D=R/'results/hardware-v4_1/physical-audit';RAW=D.parent/'physical-return';a=json.loads((D/'summary.json').read_text())
assert not a['emulation_only'] and a['valid_captures']==1 and a['failed_captures']==1 and not a['independent_repeatability_evidence']
c=next(c for c in a['captures'] if c['valid']);assert c['selection_mode']=='trained'
manifest=json.loads((RAW/'manifest.json').read_text());local=json.loads((R/'hardware/esp32c3-v4_1/main/generated/provenance.json').read_text());assert a['build_id']==manifest['build_id']==local['build_id']=='03508540d7e1eee1'
assert manifest['image_sha256']==hashlib.sha256(Path('/workspace/esp32c3-generalize-v4_1.bin').read_bytes()).hexdigest()
assert (RAW/'oracle.json').read_bytes()==(R/'hardware/esp32c3-v4_1/main/generated/oracle.json').read_bytes()
rows=[];models=[]
for m,name in [(0,'Person detector'),(1,'CIFAR-10 ResNet')]:
 group={r['backend']:r for r in c['statistics'] if r['model']==m}
 for b in range(10):
  vals=[group[b]['input_median_cycles'][s]/160000 for s in [2,3]]
  rows.append(dict(model=name,backend=a['backends'][b],held_out_mean_input_median_ms=st.mean(vals),held_out_sample_2_ms=vals[0],held_out_sample_3_ms=vals[1]))
 best=[min(group[b]['input_median_cycles'][s] for b in range(3))/160000 for s in [2,3]]
 ratio=next(r['geomean_best_c_over_rust'] for r in a['comparisons'] if r['model']==['person','cifar10_resnet'][m] and r['backend']=='selected_rust' and r['split']=='held_out')
 selected=next(r for r in rows if r['model']==name and r['backend']=='selected_rust')
 models.append(dict(model=name,fastest_tested_c_mean_input_median_ms=st.mean(best),selected_rust_mean_input_median_ms=selected['held_out_mean_input_median_ms'],held_out_geomean_c_over_rust=ratio,time_reduction_percent=100*(1-1/ratio)))
gmean=math.sqrt(math.prod(r['held_out_geomean_c_over_rust'] for r in models))
meta=json.loads((RAW/'captures/boot-2.metadata.json').read_text());progress=meta['last_device_progress'];assert not meta['completed'] and progress['stage']=='warmed' and progress['backend']==6 and progress['model']==1
# Bind the failed capture sidecars too, without accepting it as a complete session.
for key,suffix in [('raw','.raw.log'),('events','.events.jsonl'),('plan','.plan.json')]:assert hashlib.sha256((RAW/('captures/boot-2'+suffix)).read_bytes()).hexdigest()==meta['hashes'][key]
result=dict(status='Single completed physical session; reset repeatability failed',models=models,single_session_two_model_geomean=gmean,completed_sessions=1,failed_sessions=1,independent_repeatability_evidence=False,complete_session_tensor_checks=1880,complete_session_timed_runs=400,training_inputs=[0,1],held_out_inputs=[2,3],aggregation='Per-input median of 5 timed RUNs; mean over 2 held-out inputs. Ratios are geometric means over those 2 input ratios, equal weight per model.',failed_capture_last_progress=progress,scope='One ESP32-C3, one completed reset session; no power-cycle or cross-board claim. No promotion of selected export.')
(D/'comparison.json').write_text(json.dumps(result,indent=2)+'\n')
with (D/'model-comparison.csv').open('w',newline='') as f:
 w=csv.DictWriter(f,list(rows[0]));w.writeheader();w.writerows(rows)
(D/'returned-file-hashes.json').write_text(json.dumps({str(p.relative_to(RAW)):hashlib.sha256(p.read_bytes()).hexdigest() for p in RAW.rglob('*') if p.is_file()},indent=2)+'\n')
lines=['# V4.1 physical results: held-out gains in one session, repeatability unresolved','',
'Local code independently audited the returned raw captures against the local oracle. Boot 1 is valid: 1,880 intermediate tensor comparisons, 96 training profiles, 400 timed whole-model RUNs, three delay calibrations, zero mismatches, and consistent host/device clocks. Boot 2 is incomplete and explicitly rejected. Uploaded executable/bytecode files were not executed.','',
'## Held-out complete-model timing from boot 1','',
'Layer selection used only inputs 0/1; inputs 2/3 below were held out. Each value is the mean of the two input medians, each median from five RUN observations. Warmup, validation, printing and random waits are outside the timed interval. Ratios are geometric means of the two per-input ratios.','',
'| Model | Fastest tested C | Selected Rust | Held-out C/Rust |','|---|---:|---:|---:|']
for r in models:lines.append(f"| {r['model']} | {r['fastest_tested_c_mean_input_median_ms']:.3f} ms | {r['selected_rust_mean_input_median_ms']:.3f} ms | {r['held_out_geomean_c_over_rust']:.4f}× |")
lines += ['',f'The equal-model geometric mean is **{gmean:.4f}× in this one completed session**. This clears 1.2× for that session; it does not satisfy the independent-reset repeatability requirement. No confidence interval or reliable cross-reset speedup is claimed.','',
'The selected Rust-only and hybrid maps are identical in this capture. All selected learned kernels remain Rust; common C graph operators still run in both. The fixed tile-16 Rust backend also beats C on held-out ResNet inputs (1.7614×), so the successful model result is not solely a post-selection layer-profile inference. This is the first completed physical held-out ResNet win, rather than just promising training profiles.','',
'## Failure on the next reset','',
'Boot 2 reused the frozen first-boot selection, then stopped during the profiling phase at seq 12, ResNet backend 6, sample 1. Last progress is `warmed`, with cumulative minimum free stack 4,652 bytes. Boot 3 was not attempted, following the stop-after-first-failure instruction. Both sessions were USB-powered; the second used an RTS hard reset, not a physical power cycle.','',
'Backend 6 is now the per-block SRAM staging kernel, not V4 whole-weight staging. Thus removing whole-weight staging did NOT eliminate the hardware failure. A hypothesis specific to writing whole weights into the shared padding buffer is insufficient to explain all observed failures. Successful warmup does not prove that the subsequent timed kernel completed. The last marker precedes a task delay, phase delay, timer reads, timed execution and result printing; the logs cannot distinguish a stall in those later stages or the transport. The stack watermark is not a proof excluding an interrupt or transient stack problem. Do not declare a root cause from the backend number alone.','',
'## Interpretation and next review','',
'Keep the numerical improvements as single-session evidence and preserve the failure. Do not promote the selected model as independently repeatable, remove backend 6 without documenting it, or silently infer that fixed-tile and selected maps share the same reliability. Wait for the ongoing independent research review before another speculative firmware iteration. Supply the updated evidence addendum so the review covers both the real held-out gains and the repeated block-staging stall.','',
'The previous Rust ResNet backend measures about 653.6 ms averaged over all four inputs in this image, versus 682.3 ms in V3. Code layout/instrumentation differs between images; causal performance comparisons use the controls in the same V4.1 image. Stock tested C weights remain in flash, so improvements do not isolate the Rust language. Diagnostic size and cumulative stack watermarks do not establish compact deployment flash or isolated per-backend peak stack.','',
'[Held-out timing graph](held-out-comparison.png), [all backend timings](model-comparison.csv), [machine-readable interpretation](comparison.json), and [strict audit](summary.json).']
(D/'evaluation.md').write_text('\n'.join(lines)+'\n')
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
fig,axes=plt.subplots(1,2,figsize=(11,4.8),layout='constrained')
for ax,(m,name) in zip(axes,[(0,'Person detector'),(1,'CIFAR-10 ResNet')]):
 values=[models[m]['fastest_tested_c_mean_input_median_ms']]+[next(r['held_out_mean_input_median_ms'] for r in rows if r['model']==name and r['backend']==b) for b in ['rust','tile16_b2p4','selected_rust']]
 labels=['Fastest tested C','Previous Rust','Fixed tile-16 Rust','Selected Rust'];ax.barh(labels,values,color=['#888888','#4477aa','#44aa99','#228833']);ax.invert_yaxis();ax.set_xlim(0,max(values)*1.2);ax.set_xlabel('Held-out complete inference time (ms)');ax.set_title(name+f"\nSelected Rust {models[m]['held_out_geomean_c_over_rust']:.3f}×")
 for i,v in enumerate(values):ax.text(v+max(values)*.015,i,f'{v:.2f}',va='center')
fig.suptitle('ESP32-C3: one audited session · two held-out inputs\nSecond reset stalled; independent repeatability is NOT established')
fig.savefig(D/'held-out-comparison.png',dpi=170);fig.savefig(D/'held-out-comparison.svg');plt.close(fig)
print(json.dumps(result,indent=2))
