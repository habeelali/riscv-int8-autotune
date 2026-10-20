#!/usr/bin/env python3
"""Render full-model results; every value comes from executed QEMU records."""
import csv
import json
import math
import re
import shutil

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import numpy as np

from runner import ROOT

dest = ROOT/'results/model/person-detection'
manifest = json.loads((dest/'manifest.json').read_text())
rows = json.loads((dest/'results.json').read_text())
assert len(rows) == 16 and all(r['correct'] for r in rows)
assert all(len(r['samples']) == 4 and len(r['layers']) == 124 for r in rows)
by_kind = {r['kind']:r for r in rows}
rust = by_kind['generated-selected']
stock = min((r for r in rows if r['kind'].removesuffix('-native') in ('muriscv','muriscv-portable')), key=lambda r:r['mean_instructions'])
strong = min((r for r in rows if r['kind'].startswith('muriscv')), key=lambda r:r['mean_instructions'])
esp = min((r for r in rows if r['kind'] in ('c','c-native')),key=lambda r:r['mean_instructions'])
counts = lambda r: r['mean_instructions']
stock_ratio, strong_ratio, esp_ratio = (counts(r)/counts(rust) for r in (stock,strong,esp))
saved = 1-counts(rust)/counts(strong)
selection = json.loads((dest/'selection.json').read_text())
checks = json.loads((dest/'counter-crosscheck.json').read_text())
assert len(checks) == 2 and all(c['match'] for c in checks)
summary = dict(model=manifest['model'],samples=4,original_images=2,held_out_mirrors=2,
               operators=31,generated_kernel_layers=28,model_builds=len(rows),
               qemu_processes=3*len(rows),
               intermediate_tensor_comparisons=3*len(rows)*4*31,
               speedup_vs_stock_muriscv=stock_ratio,speedup_vs_strongest_muriscv=strong_ratio,
               speedup_vs_esp_nn=esp_ratio,instruction_reduction_vs_strongest_muriscv=saved,
               selected=rust['cache_key'],counter_crosscheck=True)
(dest/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')

names = {'c':'ESP-NN', 'muriscv':'muRISCV-NN scalar', 'muriscv-portable':'muRISCV-NN portable algorithm',
         'muriscv-fixed-asm':'muRISCV-NN scalar + local rounding fix',
         'muriscv-portable-fixed-asm':'muRISCV-NN portable + local rounding fix',
         'generic':'Generic Rust', 'generated-selected':'Rust selected per layer',
         **{f'generated-{i}':f'Rust schedule {i}' for i in range(4)}}
names.update({k+'-native':v+' (native first depthwise)' for k,v in list(names.items()) if k in ('c','muriscv','muriscv-portable','muriscv-fixed-asm','muriscv-portable-fixed-asm')})
figures = ROOT/'docs/figures'
figures.mkdir(exist_ok=True)
fig = plt.figure(figsize=(12,7),facecolor='#f5f7fb')
grid = fig.add_gridspec(2,2,height_ratios=[1.5,1],hspace=.55,wspace=.35)
ax = fig.add_subplot(grid[0,0])
chosen = [esp,stock,strong,rust]
labels = ['ESP-NN','muRISCV-NN\nstock best','muRISCV-NN\nincluding local fix','Tuned Rust']
colors = ['#8195b4','#5685d9','#9272cd','#139f89']
bars = ax.barh(labels,[counts(r)/1e6 for r in chosen],color=colors,height=.62)
ax.invert_yaxis()
ax.set_xlim(0,72)
ax.set_xlabel('Million executed guest instructions / complete inference')
for b,r in zip(bars,chosen):
    ax.text(b.get_width()+.7,b.get_y()+b.get_height()/2,f'{counts(r)/1e6:.2f} M',va='center',fontsize=10)
ax.set_title('Pretrained 96×96 person detector · all 31 operators',loc='left',fontsize=11,pad=12)
ax.spines[['top','right']].set_visible(False)
ax = fig.add_subplot(grid[0,1])
remaining = round(100*counts(rust)/counts(strong))
for offset,amount,color,title in [(0,100,'#9272cd','Strongest tested muRISCV-NN'),(13,remaining,'#139f89','Tuned Rust')]:
    x = np.tile(np.arange(10),10)+offset
    y = np.repeat(np.arange(10),10)
    ax.scatter(x,y,s=29,color='#dfe5ed')
    ax.scatter(x[:amount],y[:amount],s=29,color=color)
    ax.text(offset+4.5,11,title,ha='center',fontsize=9)
ax.set_ylim(13,-2)
ax.set_aspect('equal')
ax.axis('off')
ax.set_title(f'{saved:.1%} fewer instructions · {strong_ratio:.3f}× ratio',fontsize=12,pad=12)
ax.text(11,-1,'Each dot ≈ 1% of the muRISCV-NN instruction count',ha='center',fontsize=9,color='#43526a')
ax = fig.add_subplot(grid[1,:])
ratios=[]
for layer in manifest['layers']:
    j=layer['index']
    a=sum(l['instructions'] for l in strong['layers'] if l['layer']==j)
    b=sum(l['instructions'] for l in rust['layers'] if l['layer']==j)
    ratios.append(a/b)
ax.bar(range(31),ratios,color=['#139f89' if v>1.00001 else '#b0bbca' for v in ratios])
ax.axhline(1,color='#52627a',lw=1,ls='--')
ax.set_xticks(range(31))
ax.set_xlabel('Operator index (27 pooling, 29 reshape, 30 softmax share the same C implementation)')
ax.set_ylabel('muRISCV-NN / Rust\ninstruction ratio')
ax.set_ylim(0,2.3)
ax.spines[['top','right']].set_visible(False)
fig.suptitle('Real model, real QEMU execution, bit-exact intermediate tensors',fontsize=15,y=.99)
fig.text(.5,.015,'RV32IMC scalar · mean of 2 upstream images + 2 held-out mirrors · instruction counts, not hardware timing',ha='center',fontsize=10)
for suffix in ('png','svg','pdf'):
    fig.savefig(figures/f'real-model.{suffix}',dpi=180,bbox_inches='tight',facecolor=fig.get_facecolor())
plt.close(fig)

report = f'''# Complete pretrained person-detector evaluation

The generated Rust convolution/depthwise kernels execute the pinned TensorFlow
Lite Micro 96×96 INT8 MobileNet person detector with a **{strong_ratio:.3f}×**
instruction-count improvement versus the fastest tested qualified muRISCV-NN path
(including its separately documented local assembly rounding correction),
**{stock_ratio:.3f}×** versus the best unmodified scalar/portable path, and
**{esp_ratio:.3f}×** versus ESP-NN. More plainly, the strongest muRISCV-NN comparison
uses **{saved:.1%} fewer executed instructions** for Rust.

These are full-network QEMU guest instruction measurements, not hardware cycles,
latency, energy, dataset accuracy, or a comparison against a full TFLM runtime.
All backends use the same static graph runner; convolution/depthwise kernels
are substituted. Pooling, reshape, padding and softmax are included in the whole
inference count. The Rust backend is a mixed-language network: the 28 learned
layers use Rust, and common graph/pooling/softmax code uses C.

![Full-model instructions and layer ratios](figures/real-model.png)

## Model and independent reference

* Upstream: TensorFlow Lite Micro, commit
  `{json.loads((ROOT/'models/person-detection/source.json').read_text())['commit']}`.
* Original model SHA-256: `{manifest['original_sha256']}`.
* Weights are pretrained upstream weights. There is no training, pruning, model
  replacement, weight approximation or change to quantization scales.
* Independent oracle: **{manifest['reference']}**. No delegate, one host thread,
  all intermediate tensors preserved. Every layer output is compared byte for
  byte against this separately executed interpreter.
* The original upstream model has invalid `quantized_dimension=3` on 14 rank-one
  bias tensors. Current LiteRT rejects it. The importer repairs only these 14
  axis metadata fields to zero. Exactly 14 bytes differ. Weights, bias values,
  scales, zero points, operator graph and options are unchanged. The original,
  compatible copy and exact repair offsets are preserved; a unit test checks
  that these are the only byte changes.
* Input images are the pinned upstream `person.bmp` and `no_person.bmp`, decoded
  as 96×96 grayscale and quantized with `pixel - 128` as used by this example.
  Horizontal flips are reserved for validation, not schedule selection.

## Full-model measurements

Arithmetic means over four inputs. Each complete firmware runs in three fresh
QEMU processes with exactly matching outputs and counts. Startup, image decoding,
oracle comparisons, UART printing and the separate profiling pass are excluded.
Padding materialization, graph dispatch, pooling, reshape and softmax are included.
The empty measurement wrapper cost is subtracted from the full-network region.

| Implementation | Instructions / inference | `.text` bytes | Static RAM bytes | Stack watermark bytes | Strongest mu / implementation |
|---|---:|---:|---:|---:|---:|
'''
for r in rows:
    report += f"| {names[r['kind']]} | {counts(r):,.2f} | {r['text_bytes']:,} | {r['static_ram_bytes']:,} | {max(s['stack_watermark_bytes'] for s in r['samples']):,} | {counts(strong)/counts(r):.3f}× |\n"
report += '''
The selected Rust model increases code size versus the C baselines; this is not
a free performance gain. Static RAM includes two full activation buffers and a
shared explicit-padding buffer. All implementations use the same allocation
strategy. Stack watermarks are actual measured usage, separate from the 65,536
byte reserved stack region in the linker script. ELF read-only size additionally
contains all test images and every intermediate oracle; it is not deployable
model storage. The original `.tflite` is 300,568 bytes. No allocation occurs during
inference. No RVV, packed SIMD or handwritten Rust kernel assembly is used.

## Actual predictions and held-out checks

The two image names are fixture descriptions, not fabricated correctness labels.
The reference model itself predicts `no_person` on the person fixture; this error
is preserved. Passing these tests means faithfully executing the pretrained
model, not demonstrating useful dataset accuracy. Two images and their flips
cannot establish model accuracy or broad performance generalization.

| Input | Used for schedule selection | Reference INT8 output `[no_person, person]` | Reference predicted class | All 16 backends bit-exact at all 31 layers |
|---|---|---|---|---|
'''
for s in manifest['samples']:
    report += f"| {s['name']} | {'No (held-out flip)' if s['held_out'] else 'Yes'} | `{s['output']}` | {s['prediction']} | Yes |\n"
report += '\n| Input | Best stock mu instructions | Strongest mu incl. fix | Selected Rust | Strongest mu / Rust |\n|---|---:|---:|---:|---:|\n'
for index,s in enumerate(manifest['samples']):
    a,b,c = (r['samples'][index]['instructions'] for r in (stock,strong,rust))
    report += f"| {s['name']} | {a:,} | {b:,} | {c:,} | {b/c:.3f}× |\n"
report += '''
## Lowering, search and regressions

There are 31 operators: 28 convolution/depthwise layers, one average pool, one
reshape and one softmax. The first layer is a depthwise convolution with input
channels=1 and depth multiplier=8. It is lowered to an exactly equivalent ordinary
convolution with eight output channels and the weight layout transposed. This
lowering is tested for every backend. Additionally, **all five C configurations
are tested with the original native depthwise-multiplier-8 first operator** and
its original weight layout. The headline comparisons choose the faster complete
model for each library, so a slower C lowering cannot inflate the result.
Odd SAME stride-2 padding is
materialized in a common buffer using the input zero point; symmetric padding is
handled inside each kernel. No tensor dimension, stride or activation is dropped.

Four genuinely different Rust schedule families are compiled as full models and
validated before selection. They explore output-channel blocks, adjacent-pixel
reuse, channel-first depthwise loops, unrolling and partial im2col. Quantization
remains exact. Per-layer selection minimizes mean profiled instructions on the
two original images only. The selected source is then independently compiled and
the entire graph remeasured, including on held-out flips. The headline result is
the actual selected-model region, not a sum of optimistic per-layer minima.

Generic Rust is substantially slower than both C libraries. All losing schedule
measurements and sources are retained. Schedule 2's wider pixel tiles and depthwise blocks grow
code and loses to schedule 0 overall. The selected per-layer mixture improves
on the best uniform schedule, with a further code-size tradeoff.

| Layer | Operator | Selected schedule | Strongest mu instructions | Rust instructions | Ratio |
|---|---|---:|---:|---:|---:|
'''
layer_rows=[]
for l in manifest['layers']:
    j=l['index']
    a=sum(v['instructions'] for v in strong['layers'] if v['layer']==j)/4
    b=sum(v['instructions'] for v in rust['layers'] if v['layer']==j)/4
    schedule=selection['schedules'].get(str(j),'shared C')
    report += f"| {j} | {l['op']} | {schedule} | {a:,.2f} | {b:,.2f} | {a/b:.3f}× |\n"
    layer_rows.append(dict(layer=j,operator=l['op'],schedule=schedule,baseline_instructions=a,rust_instructions=b,ratio=a/b))
with (dest/'layers.csv').open('w') as f:
    writer=csv.DictWriter(f,layer_rows[0].keys());writer.writeheader();writer.writerows(layer_rows)
report += f'''
## Validation and reproducibility

* **{len(rows)} full-model builds × 3 fresh QEMU runs × 4 inputs**. Every output
  tensor from all 31 operators matches LiteRT. This is
  **{summary['intermediate_tensor_comparisons']:,} intermediate-tensor comparisons**,
  including repeated process runs. It is not that many independent test images.
* The timed model output is checked before the separate per-layer verification
  pass. Instruction counts repeat exactly in all three processes.
* QEMU execution-callback plugin counts match CSR counts exactly for the complete
  ESP-NN and selected Rust inference regions on all four inputs (eight checks).
* Importer tests verify the precise metadata repair, layer geometry and held-out
  reference inputs. Existing arithmetic and memory tests also pass.
* Toolchain versions, compiler flags, binary hashes, commands, raw UART logs,
  ELF files, assembly and maps are in each measurement's `build_dir`.
* `-O3`, scalar `-march=rv32imc`, no LTO. The harness alone enables CSR access.
  QEMU uses the same `virt` / `lowrisc-ibex` configuration as the kernel suite.

```sh
python3 -m pip install -r requirements-model.txt -r requirements.txt
cargo run --locked -p autotuner -- benchmark-model
bash scripts/build-plugin.sh
python3 scripts/validate_model_counter.py
python3 scripts/report_model.py
```

Use `python3 scripts/benchmark_model.py --force` to rebuild and rerun every model.
Sources, flags, target data and tool binaries participate in the cache key.
Model results: [JSON](../results/model/person-detection/results.json),
[CSV](../results/model/person-detection/results.csv),
[layer CSV](../results/model/person-detection/layers.csv),
[counter cross-check](../results/model/person-detection/counter-crosscheck.json).
Selected reusable kernel source: [kernels.rs](../generated/person-detection/kernels.rs).

## Limits

This validates one pretrained model with a small fixture corpus on scalar
RV32IMC QEMU. It does not establish ESP32-C3 hardware latency, energy, larger-model
performance, language superiority, or superiority over MicroFlow/CubeCL. Future
work should use a labeled evaluation dataset, measure real ESP32-C3 cycles and
memory placement, and expand the importer beyond this explicitly supported graph.
'''
(ROOT/'docs/model-evaluation.md').write_text(report)
export=ROOT/'generated/person-detection';export.mkdir(parents=True,exist_ok=True)
shutil.copyfile(ROOT/rust['build_dir']/'kernels.rs',export/'kernels.rs')
shutil.copyfile(dest/'selection.json',export/'selection.json')
shutil.copyfile(dest/'manifest.json',export/'manifest.json')
for row in rows:
    source=ROOT/row['build_dir']
    evidence=dest/'evidence'/row['kind'];evidence.mkdir(parents=True,exist_ok=True)
    for name in ('qemu-0.txt','qemu-1.txt','qemu-2.txt','size.txt','attributes.txt','assembly.txt',
                 'graph.c','kernels.rs','kernels.c','plugin-stdout.txt','plugin-stderr.txt'):
        if (source/name).exists():
            shutil.copyfile(source/name,evidence/name)
readme=ROOT/'README.md'
text=readme.read_text()
block=f'''<!-- MODEL_RESULTS_START -->
**Complete pretrained model:** the 96×96 TFLM INT8 person detector (31 operators)
uses **{counts(rust)/1e6:.2f} million instructions** with selected Rust kernels,
versus **{counts(strong)/1e6:.2f} million** for the strongest tested scalar muRISCV-NN
configuration (**{strong_ratio:.3f}×**, **{saved:.1%} fewer instructions**) and
**{counts(esp)/1e6:.2f} million** for ESP-NN (**{esp_ratio:.3f}×**).
All intermediate tensors match independent LiteRT inference on two upstream
images and two held-out mirrors. [Full-model report](docs/model-evaluation.md)
includes code-size costs, metadata repair, prediction errors and real measurements.

![Complete pretrained model results](docs/figures/real-model.svg)
<!-- MODEL_RESULTS_END -->'''
if '<!-- MODEL_RESULTS_START -->' in text:
    text=re.sub(r'<!-- MODEL_RESULTS_START -->.*?<!-- MODEL_RESULTS_END -->',lambda _:block,text,flags=re.S)
else:
    text=text.replace('<!-- RESULTS_START -->',block+'\n\n<!-- RESULTS_START -->',1)
readme.write_text(text)
print(f'Full model: stock mu {stock_ratio:.3f}x; strongest mu {strong_ratio:.3f}x; ESP-NN {esp_ratio:.3f}x')
