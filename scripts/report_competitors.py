#!/usr/bin/env python3
"""Compare only executed, numerically qualified scalar upstream configurations."""
import csv,json,math
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from runner import ROOT
D=ROOT/'results';F=ROOT/'docs/figures';F.mkdir(exist_ok=True)
read=lambda p:json.loads(p.read_text())
raw=read(D/'muriscv.json');qualification=read(D/'upstream-quantization.json')
qualified={r['kind'] for r in qualification if r['qualified']}
esp={r['workload']:r for r in read(D/'baseline.json')}
previous={r['workload']:r for r in read(D/'previous-selected.json')}
primary=['conv3x3','pointwise','depthwise','fully_connected'];names=primary+[n for n in esp if n not in primary]
rows=[];controlled=[]
for n in names:
    selected=read(D/f'best-{n}.json')['best']
    valid=[r for r in raw if r['workload']==n and r['correct'] and r['kind'] in qualified]
    stock=min((r for r in valid if 'fixed' not in r['kind']),key=lambda r:r['instructions'])
    strongest=min(valid,key=lambda r:r['instructions'])
    rows.append(dict(workload=n,esp_nn_instructions=esp[n]['instructions'],previous_rust_instructions=previous[n]['instructions'],muriscv_stock_instructions=stock['instructions'],muriscv_stock_kind=stock['kind'],muriscv_best_instructions=strongest['instructions'],muriscv_best_kind=strongest['kind'],tuned_rust_instructions=selected['instructions'],improvement_vs_stock_muriscv=stock['instructions']/selected['instructions'],improvement_vs_best_muriscv=strongest['instructions']/selected['instructions'],improvement_vs_previous_rust=previous[n]['instructions']/selected['instructions'],improvement_vs_esp_nn=esp[n]['instructions']/selected['instructions'],selected=selected['variant'],muriscv_text=strongest['text_bytes'],tuned_text=selected['text_bytes'],muriscv_scratch=strongest['scratch_bytes'],tuned_scratch=selected['scratch_bytes'],muriscv_stack_watermark=strongest['stack_watermark_bytes'],tuned_stack_watermark=selected['stack_watermark_bytes'],upstream_build_dir=strongest['build_dir'],rust_build_dir=selected['build_dir']))
    candidates=read(D/f'tuning-{n}.json')
    cfg=selected['config']
    for factor,changes in [('specialized requantization',dict(fast_quant=False)),('spatial tiling',dict(pixels=1)),('channel-first loop',dict(channel_outer=False))]:
        if all(cfg[k]==v for k,v in changes.items()):continue
        match={**cfg,**changes}
        a=next((r for r in candidates if r.get('correct') and r['config']==match),None)
        if a:controlled.append(dict(workload=n,factor=factor,with_factor=selected['variant'],without_factor=a['variant'],with_instructions=selected['instructions'],without_instructions=a['instructions'],ratio=a['instructions']/selected['instructions']))
def gm(key,ns):return math.exp(sum(math.log(r[key]) for r in rows if r['workload'] in ns)/len(ns))
summary=dict(metric='Executed RV32IMC guest instructions; not hardware cycles or elapsed time',primary_geomean_vs_stock_muriscv=gm('improvement_vs_stock_muriscv',primary),all_geomean_vs_stock_muriscv=gm('improvement_vs_stock_muriscv',names),primary_geomean_vs_best_muriscv=gm('improvement_vs_best_muriscv',primary),all_geomean_vs_best_muriscv=gm('improvement_vs_best_muriscv',names),primary_geomean_vs_previous_rust=gm('improvement_vs_previous_rust',primary),primary_geomean_vs_esp_nn=gm('improvement_vs_esp_nn',primary),qualified_configurations=sorted(qualified),rows=rows,controlled_ablations=controlled)
(D/'competitor-comparison.json').write_text(json.dumps(summary,indent=2)+'\n')
for path,items in [(D/'competitor-comparison.csv',rows),(D/'controlled-ablations.csv',controlled)]:
    if items:
        with path.open('w') as f:writer=csv.DictWriter(f,list(items[0]));writer.writeheader();writer.writerows(items)
fig,ax=plt.subplots(figsize=(11,4.5));x=list(range(len(names)));width=.23
for shift,key,label,color in [(-width,'muriscv_stock_instructions','muRISCV-NN stock scalar/portable','#7489a3'),(0,'muriscv_best_instructions','muRISCV-NN incl. local rounding fix','#b79bc5'),(width,'tuned_rust_instructions','Tuned Rust','#28a88a')]:
    values=[100*r[key]/r['muriscv_best_instructions'] for r in rows]
    ax.bar([i+shift for i in x],values,width,label=label,color=color)
ax.axhline(100,color='black',lw=.8);ax.set_ylabel('Instructions (% of fastest qualified muRISCV-NN)');ax.set_xticks(x,names,rotation=25,ha='right');ax.legend(loc='upper center',bbox_to_anchor=(.5,1.18),ncol=3,fontsize=8.5);fig.tight_layout()
for ext in ('png','svg','pdf'):fig.savefig(F/f'vs-muriscv.{ext}',dpi=170)
plt.close(fig)
table='| Workload | Stock muRISCV-NN | Best incl. local fix | Tuned Rust | Best mu / Rust | Selected Rust |\n|---|---:|---:|---:|---:|---|\n'
for r in rows:table+=f"| {r['workload']} | {r['muriscv_stock_instructions']:,} | {r['muriscv_best_instructions']:,} | {r['tuned_rust_instructions']:,} | {r['improvement_vs_best_muriscv']:.3f}× | `{r['selected']}` |\n"
mem='| Workload | mu / Rust .text | mu / Rust scratch | mu / Rust stack watermark |\n|---|---:|---:|---:|\n'
for r in rows:mem+=f"| {r['workload']} | {r['muriscv_text']:,} / {r['tuned_text']:,} | {r['muriscv_scratch']:,} / {r['tuned_scratch']:,} | {r['muriscv_stack_watermark']:,} / {r['tuned_stack_watermark']:,} |\n"
ab='| Workload | Change removed from selected schedule | With | Without | Without / with |\n|---|---|---:|---:|---:|\n'
for r in controlled:ab+=f"| {r['workload']} | {r['factor']} | {r['with_instructions']:,} | {r['without_instructions']:,} | {r['ratio']:.3f}× |\n"
previous_table='| Workload | Previous Rust, remeasured | New Rust | Previous / new |\n|---|---:|---:|---:|\n'
for r in rows:previous_table+=f"| {r['workload']} | {r['previous_rust_instructions']:,} | {r['tuned_rust_instructions']:,} | {r['improvement_vs_previous_rust']:.3f}× |\n"
qtable='| Configuration | Quantization cases | Mismatches | Eligible for comparisons |\n|---|---:|---:|---|\n'
for r in qualification:qtable+=f"| {r['kind']} | {r['cases']:,} | {r['mismatches']} | {'yes' if r['qualified'] else 'no'} |\n"
reg=read(D/'muriscv-regression.json') if (D/'muriscv-regression.json').exists() else []
report=f'''# Further optimization against muRISCV-NN

## Outcome and comparison scope

The four primary workloads achieve **{summary['primary_geomean_vs_stock_muriscv']:.3f}×**
geometric-mean instruction-count improvement against the fastest tested, qualified
**unmodified** muRISCV-NN scalar/portable configuration per workload.
Against the strongest tested, qualified configuration **including our separately labeled
local assembly-rounding correction**, the result is
**{summary['primary_geomean_vs_best_muriscv']:.3f}×** on the four primary shapes and
**{summary['all_geomean_vs_best_muriscv']:.3f}×** across all eight.

These are comparisons of isolated scalar RV32IMC INT8 operators in QEMU, not
claims about all muRISCV-NN targets, RVV performance, complete models, hardware
cycles or elapsed-time speedups. Every workload and any regression remain below.

{table}

![Comparison with muRISCV-NN](figures/vs-muriscv.svg)

## Fair baseline construction

Upstream: `tum-ei-eda/muriscv-nn` commit
`f69494442858980be78180868d96c3524dbf26f9`, recorded as a submodule and lock file.
The stock source is unchanged. We call the upstream convolution, depthwise and
fully-connected wrappers, allowing their dispatch to choose specialized paths.
The FC wrapper selects per-channel quantization. Standard convolution's filter
layout, NHWC activations, depthwise multiplier one, bias, offsets, activation
clamp and per-channel multiplier/shift arrays are identical to the Rust kernels.
All inputs remain the original eight workloads and deterministic fixture seed.

Both standard scalar code and the `USE_PORTABLE_VEXT` algorithm configuration
are compiled at `-O3 -march=rv32imc -mabi=ilp32`, without LTO. The latter selects
the portable algorithms associated with vector kernels; it does not enable RVV
instructions. We do not compare a scalar-only Rust kernel to an emulator running
a different, more capable ISA. Required scratch is supplied, and setup inside
the kernel callback is counted. Exact source compilation commands and common
archive build cost are retained under `results/upstream-build/`.

The same UART, startup, linker, calibrated instruction region and three-process
repeat protocol are used by every implementation. Both C and Rust use the same
freestanding `memcpy`/`memset`, with aligned 32-bit scalar copies/fills and byte
tails. Host guard/alignment tests exercise those routines. All original selected
Rust kernels were remeasured with this common runtime for a controlled comparison
with the new optimizations; the initial measurements remain in `results/phase-before-muriscv/`.
Memory-library changes are not presented as Rust kernel improvements.

## Additional correctness gate and optional upstream assembly

A separate QEMU test checks upstream requantization on 2,240 scalar cases,
including sign/rounding boundaries and deterministic random values with
nonnegative multipliers and shifts -27..0.

{qtable}

The optional unmodified RV32 assembly path has a reproducible rounding error at
this commit. A minimal example is `val=2, multiplier=1073741824, shift=0`:
expected result **1**, unmodified assembly result **2**. This optional setting is
not enabled by the normal scalar build; the default implementation passes.
We exclude a globally failing numerical configuration even if a particular
operator fixture happens to mask its errors through shifting/clamping.

The RV32 implementation uses `q=low_product>>30` as its rounding contribution,
where the required contribution is `(low_product>>31)+((low_product>>30)&1)`,
equivalently `q-(q>>1)`. A local header overlay corrects the returned result by
subtracting `lo>>1` after the original assembly. It leaves the upstream checkout
untouched, has no input-specific special cases, and passes the same quantization
tests. The `*-fixed-asm` measurements include this correction and are explicitly
not described as unmodified upstream. Raw source, generated overlay, minimal
counterexamples, commands and QEMU output are preserved. This is an observed
bug at a pinned revision, not a claim that all versions of the project are wrong.
No issue or message was sent to upstream maintainers.

## New generated implementations

* Multi-pixel tiles (two or four adjacent outputs) share loaded weights across
  pixels while output-channel blocks share loaded inputs. 3×3 variants use a
  partial INT16 im2col buffer; 1×1 variants read the input directly. Tile filling,
  padding, channel tails, row tails and initial scratch clearing stay timed.
* Depthwise channel-first schedules interchange the channel-block and spatial
  loops, allowing the compiler to reuse filter/quantization metadata differently.
  These schedules are evaluated rather than assumed faster.
* A nonnegative-multiplier/nonpositive-shift requantization path removes general
  signed-nudge/left-shift handling. Generated entry points validate the parameter
  restrictions once per invocation. It is numerically identical under that
  contract; it is not an approximate quantizer. Rust tests compare 11,364 edge
  and deterministic random cases with the general implementation. The existing
  independent C/Python/Rust differential tests remain in place.

Matched candidates below remove one factor while retaining the selected
schedule's other parameters. Effects interact and must not be added together.
Missing pairs are not synthesized.

{ab}

## Progress relative to our previous implementation

{previous_table}

The primary-shape geometric mean over the remeasured previous Rust kernels is
{summary['primary_geomean_vs_previous_rust']:.3f}×. This controls the common runtime
change and uses actual execution, rather than comparing estimated costs.

## Code and memory costs

{mem}

.text includes the common benchmark and linked routines. Scratch is explicitly
included in each kernel's observed stack use. A 64 KiB stack is reserved for all
implementations, and executable flash plus static/reserved RAM are checked against
workload budgets when selecting generated kernels. Watermarks are observations,
not formal worst-case stack bounds. Memory increases are reported rather than
hidden by the instruction metric.

## Regression evidence and limits

Saved additional muRISCV-NN regression records: **{len(reg)}**. These use new
fixture seeds on the same shapes and reject numerical failures. Generated Rust
regressions, the independent executed-instruction callback checks, formatting,
Clippy and Rust/Python tests are recorded alongside the original suite.

The results remain subject to compiler differences (GCC C vs LLVM Rust), workload
specialization, isolated-layer scope, finite correctness samples, and QEMU's lack
of ESP32-C3 cycle/cache modeling. We have not established a general language
advantage. Matched C/Clang schedules and real hardware remain important controls.

CubeCL is a general compute-kernel framework, while RISCVML's referenced poster
is an educational workflow, so neither supplies a directly interchangeable
RV32IMC INT8 operator baseline here. MicroFlow has generated Rust operators but
its inspected quantization epilogue has a different floating-point-based contract;
we do not claim to beat it by silently changing quantization or clamping.
This extension targets an actual compatible competitor rather than declaring
victory over unrelated or numerically different systems.

## Reproduce

```bash
cargo run --locked -p autotuner -- benchmark-competitors
cargo run --locked -p autotuner -- tune all
cargo run --locked -p autotuner -- regression
python3 scripts/regression_muriscv.py
python3 scripts/validate_counter.py
python3 scripts/report_competitors.py
```

`results/competitor-comparison.json` and CSV contain every comparison;
`results/muriscv.json` contains raw successful operator measurements;
`results/muriscv-failures.json` retains rejected executions;
`results/upstream-quantization.json` records qualification and counterexamples.
'''
(ROOT/'docs/competitor-evaluation.md').write_text(report)
print(table)
print('Primary mean vs stock:',summary['primary_geomean_vs_stock_muriscv'],'vs strongest incl. fix:',summary['primary_geomean_vs_best_muriscv'])
