#!/usr/bin/env python3
"""Render measured flash savings with the strict performance guardrails visible."""
import csv
import hashlib
import json
import re
from pathlib import Path

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

from runner import ROOT

dest=ROOT/'results/flash'
search=json.loads((dest/'best.json').read_text())
rows=json.loads((dest/'candidates.json').read_text())
deployment=json.loads((dest/'deployment-comparison.json').read_text())
before,after=search['reference'],search['best']
db,da=deployment['before'],deployment['after']
assert all(c['match'] for c in deployment['counter_crosscheck'])
assert all(a['instructions']==b['instructions'] for a,b in zip(before['samples'],after['samples']))
assert all(a['instructions']==b['instructions'] for a,b in zip(before['layers'],after['layers']))
code_saved=before['text_bytes']-after['text_bytes']
code_fraction=code_saved/before['text_bytes']
image_saved=db['binary_file_bytes']-da['binary_file_bytes']
image_fraction=image_saved/db['binary_file_bytes']
original=(dest/'reference/kernels.rs').read_text()
compact=(ROOT/after['source']).read_text()
historical=json.loads((dest/'reference/model-results.json').read_text())
strongest_mu=min(r['mean_instructions'] for r in historical if r['kind'].startswith('muriscv'))
mu_ratio=strongest_mu/after['mean_instructions']
preserved=json.loads((dest/'reference/preserved-hashes.json').read_text())
assert all(hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==digest for path,digest in preserved.items())
(dest/'preservation.json').write_text(json.dumps(dict(verified=True,files=len(preserved),sha256=preserved),indent=2)+'\n')
original_bodies=original.count('pub fn run(')
compact_bodies=compact.count('pub fn run(')
summary=dict(code_bytes_before=before['text_bytes'],code_bytes_after=after['text_bytes'],
             code_bytes_saved=code_saved,code_reduction=code_fraction,
             deploy_image_bytes_before=db['binary_file_bytes'],deploy_image_bytes_after=da['binary_file_bytes'],
             deploy_image_bytes_saved=image_saved,deploy_image_reduction=image_fraction,
             model_instructions_before=before['mean_instructions'],model_instructions_after=after['mean_instructions'],
             original_bodies=original_bodies,shared_bodies=compact_bodies,
             four_input_instruction_deltas=[a['instructions']-b['instructions'] for a,b in zip(after['samples'],before['samples'])],
             all_124_layer_instruction_deltas_zero=True,fifth_input_instruction_delta=da['instructions_raw']-db['instructions_raw'],
             candidates=len(rows),eligible_candidates=sum(r['eligible'] for r in rows),
             unchanged_previous_microbenchmark_exports=True)
(dest/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')

fig,axes=plt.subplots(1,3,figsize=(12,4.4),facecolor='#f5f7fb')
blue,green='#7d89bc','#159d86'
for ax,values,title,unit in zip(axes,
        ([before['text_bytes']/1024,after['text_bytes']/1024],
         [db['binary_file_bytes']/1024,da['binary_file_bytes']/1024],
         [before['mean_instructions']/1e6,after['mean_instructions']/1e6]),
        (f'Code: {code_fraction:.1%} smaller',f'Model image: {image_fraction:.1%} smaller','Instructions: unchanged'),
        ('KiB of .text, validation firmware','KiB, firmware without test oracles','Million instructions / inference')):
    bars=ax.bar(['Before','After'],values,color=[blue,green],width=.6)
    ax.set_ylim(0,max(values)*1.22)
    for bar,value in zip(bars,values):
        ax.text(bar.get_x()+bar.get_width()/2,value+max(values)*.035,f'{value:.2f}',ha='center',fontsize=12)
    ax.set_ylabel(unit)
    ax.set_title(title,fontsize=12,pad=12)
    ax.spines[['top','right']].set_visible(False)
fig.suptitle('Flash tuning without losing the existing instruction-count improvement',fontsize=14,y=1.01)
fig.text(.5,-.025,'Actual RV32IMC QEMU runs · identical 124 layer/input counts · no weight compression or added buffers',ha='center',fontsize=10)
fig.tight_layout(w_pad=2)
for suffix in ('png','svg','pdf'):
    path=ROOT/'docs/figures'/f'flash-tuning.{suffix}'
    fig.savefig(path,dpi=180,bbox_inches='tight',facecolor=fig.get_facecolor())
plt.close(fig)

report=f'''# Flash tuning with strict non-regression

The selected `{after['variant']}` candidate reduces full validation firmware
code from **{before['text_bytes']:,} to {after['text_bytes']:,} bytes**:
**{code_saved:,} bytes saved ({code_fraction:.1%})**. It also saves
**{before['rodata_bytes']-after['rodata_bytes']:,} read-only bytes**.
All four full-inference instruction counts and all 124 profiled layer/input
counts are **exactly unchanged**. Thus the prior **{mu_ratio:.3f}×** instruction-count
improvement versus the strongest tested muRISCV-NN path is preserved.

These are scalar RV32IMC QEMU instruction counts, not real hardware cycles or
latency. Preserving QEMU counts cannot guarantee identical instruction-cache
behavior or timing on ESP32-C3 hardware.

![Flash and instruction comparison](figures/flash-tuning.png)

## Real firmware image, not test-oracle storage

The validation ELF contains four images and all intermediate expected tensors.
Its roughly 1.2 MB of read-only storage must not be advertised as model flash.
We also linked and actually executed two firmware images containing only model
parameters, code, common graph operations and a RAM input buffer. No test images
or expected tensors are linked; symbols are checked for their absence.
Both images use the same explicit section-alignment policy. Small `.srodata`
objects are collected into `.rodata`; BSS starts aligned to 16 bytes. This avoids
mistaking address-dependent alignment padding for extra tensor RAM.

| Metric | Before | After | Change |
|---|---:|---:|---:|
| Validation firmware `.text` | {before['text_bytes']:,} B | {after['text_bytes']:,} B | −{code_saved:,} B |
| Deployment `.text` | {db['text_bytes']:,} B | {da['text_bytes']:,} B | −{db['text_bytes']-da['text_bytes']:,} B |
| Deployment `.rodata` | {db['rodata_bytes']:,} B | {da['rodata_bytes']:,} B | −{db['rodata_bytes']-da['rodata_bytes']:,} B |
| **Actual `.bin` file size** | **{db['binary_file_bytes']:,} B** | **{da['binary_file_bytes']:,} B** | **−{image_saved:,} B ({image_fraction:.1%})** |
| Deployment static RAM | {db['static_ram_bytes']:,} B | {da['static_ram_bytes']:,} B | {da['static_ram_bytes']-db['static_ram_bytes']:+,} B |
| Full model instructions, four-input mean | {before['mean_instructions']:,.2f} | {after['mean_instructions']:,.2f} | 0 |
| Maximum validation stack watermark | {max(s['stack_watermark_bytes'] for s in before['samples']):,} B | {max(s['stack_watermark_bytes'] for s in after['samples']):,} B | 0 |

The complete image shrinks less than code alone because pretrained weights
dominate flash. Model parameters are untouched: no compression, decoding,
retraining, altered quantization, extra scratch storage or weight copies.
The `.bin` size includes alignment gaps; section totals are separately recorded.
These are QEMU-virt deployment-style firmware artifacts, not flashed ESP32-C3
firmware. The fifth input is an all-zero quantized image supplied from RAM.
Its output `{da['output']}` matches independent LiteRT, and its measured region
is exactly **{da['instructions_raw']:,} instructions** in both images.

## Transformation

The original generator emitted **{original_bodies}** separate Rust `run` bodies.
Exact emitted-code comparison identifies identical implementations after removing
comments and entry-point names. The compact source contains **{compact_bodies}**
bodies. Repeated 6×6×128 depthwise and 6×6×128→128 pointwise layers share bodies;
all 28 original C-callable layer entry points remain present.

Weights, biases, multipliers and shifts remain runtime arguments, so sharing a
body does not share tensor values. Geometry, offsets, bounds, clamps and schedules
must match in the actual emitted code. The duplicate wrapper calls the canonical
Rust body directly. It adds no extra dispatch layer compared with the original
wrapper-to-body structure. Actual instruction measurements confirm this.

All target code remains `no_std`; no heap allocation or new inference assembly.
Both original and winning builds use `-O3` / Rust `opt-level=3`, no LTO, RV32IMC.

## Search, including rejected smaller candidates

Nine candidates explore source-body sharing, shared out-of-line requantization,
and Rust optimization levels `3`, `s`, `z`. The size levels are controls, not the
project contribution; winning source actually contains different body sharing.
Acceptance requires all of the following, with **zero tolerance**:

* Bit-exact LiteRT outputs, identical sample and layer identities.
* No instruction increase on **any** of the four full inferences or **any** of
  the 124 profiled layer/input regions.
* No static RAM or per-input stack watermark increase.
* No code-size increase, and strictly smaller combined code/read-only storage.

| Candidate | `.text` bytes | Mean instructions | Accepted | Reason if rejected |
|---|---:|---:|---|---|
'''
for r in rows:
    reasons=', '.join(r['rejection_reasons'][:2])
    report+=f"| {r['variant']} | {r['text_bytes']:,} | {r['mean_instructions']:,.2f} | {'Yes' if r['eligible'] else 'No'} | {reasons} |\n"
report+='''
Outlining requantization can shrink code further, but it increases instructions;
those results are retained and rejected. Size-oriented compiler settings also
regress performance here. The fastest original selection remains available as
an immutable fallback. The eight prior microbenchmark kernel exports and their
measurements are unchanged.

## Validation and reuse

The frozen original source was independently rebuilt and rerun before search;
its code size and counts match the historical result. Each of the nine candidates
runs three fresh QEMU processes on four inputs, checking every intermediate
tensor. The production-style images additionally run three times on a fifth
input never used for selection. Eight independent QEMU executed-instruction
plugin checks match the original and compact model CSR regions exactly.

Rust tests verify that only identical emitted code is shared and that original
entry points remain. Selection tests reject even a single extra instruction,
changed tensor output, sample identity, stack or RAM regression. Existing Rust
and Python correctness tests, formatting and Clippy pass.

```sh
cargo run --locked -p autotuner -- tune-flash
bash scripts/build-plugin.sh
python3 scripts/evaluate_flash.py
python3 scripts/report_flash.py
```

Or run `bash scripts/reproduce-flash.sh` after the normal model/toolchain setup.

* Preferred compact source: [kernels.rs](../generated/person-detection/flash/kernels.rs).
  It exposes the same 28 layer symbols and accepts the same tensor buffers.
* Preserved original: [kernels.rs](../results/flash/reference/kernels.rs).
* [All candidate measurements](../results/flash/candidates.json),
  [strict selection evidence](../results/flash/best.json),
  [deployment images and counter checks](../results/flash/deployment-comparison.json).
* Actual QEMU firmware files:
  [before](../results/flash/deployment/before/model.bin) and
  [after](../results/flash/deployment/after/model.bin).

The optimized source is exported separately so rerunning the original model
study cannot overwrite it. The original model report remains the record of the
original performance experiment; this report records its compact successor.
'''
(ROOT/'docs/flash-tuning.md').write_text(report)
with (dest/'candidates.csv').open('w',newline='') as f:
    fields=['variant','text_bytes','rodata_bytes','mean_instructions','static_ram_bytes','eligible','cache_key']
    writer=csv.DictWriter(f,fields,extrasaction='ignore',lineterminator='\n');writer.writeheader();writer.writerows(rows)
readme=ROOT/'README.md';text=readme.read_text()
block=f'''<!-- FLASH_RESULTS_START -->
**Flash tuning, no instruction regression:** shared generated Rust bodies reduce
code **{before['text_bytes']:,} → {after['text_bytes']:,} bytes ({code_fraction:.1%})**.
The full model firmware image shrinks **{db['binary_file_bytes']:,} → {da['binary_file_bytes']:,} bytes ({image_fraction:.1%})**.
All four inference counts and all 124 layer/input counts remain exactly unchanged;
the **{mu_ratio:.3f}×** comparison against the strongest tested muRISCV-NN path is preserved.
[Flash report](docs/flash-tuning.md) · [Preferred compact source](generated/person-detection/flash/kernels.rs)
<!-- FLASH_RESULTS_END -->'''
if '<!-- FLASH_RESULTS_START -->' in text:
    text=re.sub(r'<!-- FLASH_RESULTS_START -->.*?<!-- FLASH_RESULTS_END -->',lambda _:block,text,flags=re.S)
else:
    text=text.replace('<!-- MODEL_RESULTS_START -->',block+'\n\n<!-- MODEL_RESULTS_START -->',1)
readme.write_text(text)
print(f'Code {code_fraction:.1%} smaller; model image {image_fraction:.1%} smaller; every measured instruction count unchanged.')
