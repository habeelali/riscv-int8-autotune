#!/usr/bin/env python3
"""Generate tables and figures only from saved, executed QEMU measurements."""
import csv,json,math,re
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from runner import ROOT,save
RESULTS=ROOT/'results'; DOCS=ROOT/'docs';FIG=DOCS/'figures';FIG.mkdir(exist_ok=True)
primary=['conv3x3','pointwise','depthwise','fully_connected']
base={r['workload']:r for r in json.loads((RESULTS/'baseline.json').read_text())}
generic={r['workload']:r for r in json.loads((RESULTS/'generic.json').read_text())}
best={name:json.loads((RESULTS/f'best-{name}.json').read_text())['best'] for name in base}
tuning={name:json.loads((RESULTS/f'tuning-{name}.json').read_text()) for name in base}
names=primary+[n for n in base if n not in primary]
rows=[]
for n in names:
    c,g,b=base[n],generic[n],best[n]
    assert c['correct'] and g['correct'] and b['correct']
    rows.append(dict(workload=n,c_instructions=c['instructions'],generic_instructions=g['instructions'],tuned_instructions=b['instructions'],instruction_improvement=c['instructions']/b['instructions'],generic_relative_instructions=g['instructions']/c['instructions'],variant=b['variant'],c_text_bytes=c['text_bytes'],tuned_text_bytes=b['text_bytes'],text_ratio=b['text_bytes']/c['text_bytes'],flash_bytes=b['flash_bytes'],static_ram_bytes=b['static_ram_bytes'],stack_watermark_bytes=b['stack_watermark_bytes'],reserved_stack_bytes=b['reserved_stack_bytes'],scratch_bytes=b['scratch_bytes'],compile_seconds=b['compile_seconds']))
def csv_write(path,items):
    with path.open('w') as f:
        out=csv.DictWriter(f,list(items[0]));out.writeheader();out.writerows(items)
csv_write(RESULTS/'comparison.csv',rows)
gm=lambda ns:math.exp(sum(math.log(base[n]['instructions']/best[n]['instructions']) for n in ns)/len(ns))
summary=dict(primary_geomean=gm(primary),convolution_geomean=gm(primary[:3]),all_geomean=gm(names),workloads=rows,candidates=sum(len(v) for v in tuning.values()),failed_candidates=sum('error' in r for v in tuning.values() for r in v),regressing_candidates=sum(r.get('correct',False) and r['instructions']>base[n]['instructions'] for n,rs in tuning.items() for r in rs),search_seconds=sum(json.loads((RESULTS/f'best-{n}.json').read_text())['search_wall_seconds'] for n in names),candidate_compile_seconds=sum(r.get('compile_seconds',0) for rs in tuning.values() for r in rs),metric='QEMU executed guest instructions; not hardware cycles or execution time')
(RESULTS/'comparison.json').write_text(json.dumps(summary,indent=2)+'\n')
all_rows=[dict(r,workload=n) for n,rs in tuning.items() for r in rs]
save(all_rows,RESULTS/'candidates.json')
# Real bounded-grid ablations: transform one dimension at a time from b1/u1/looped.
abl=[]
labels=[('constant dimensions',1,1,False,False,False),('blocking only',4,1,False,False,False),('reduction unroll only',1,4,False,False,False),('spatial unroll only',1,1,True,False,False),('interior split only',1,1,True,False,True),('partial im2col only',1,1,False,True,False)]
for n in names:
    for label,b,u,sp,im,interior in labels:
        match=next((r for r in tuning[n] if r.get('correct') and all(r['config'].get(k)==v for k,v in dict(block=b,unroll=u,spatial_unroll=sp,im2col=im,interior=interior,pixels=1,channel_outer=False,fast_quant=False).items())),None)
        if match:
            abl.append(dict(workload=n,transformation=label,instructions=match['instructions'],relative_to_c=base[n]['instructions']/match['instructions'],text_bytes=match['text_bytes'],scratch_bytes=match['scratch_bytes'],variant=match['variant']))
csv_write(RESULTS/'ablation.csv',abl)
(RESULTS/'ablation.json').write_text(json.dumps(abl,indent=2)+'\n')
plt.rcParams.update({'font.size':10,'axes.spines.top':False,'axes.spines.right':False})
def figure_save(fig,name):
    fig.tight_layout()
    for ext in ('svg','png','pdf'):fig.savefig(FIG/f'{name}.{ext}',dpi=160)
    plt.close(fig)
fig,ax=plt.subplots(figsize=(10,4));x=list(range(len(names)));width=.25
for off,group,label,color in [(-width,base,'ESP-NN C','#69778a'),(0,generic,'Generic Rust','#e5a04c'),(width,best,'Tuned Rust','#279d89')]:
    ax.bar([v+off for v in x],[group[n]['instructions'] for n in names],width,label=label,color=color)
ax.set_yscale('log');ax.set_ylabel('Executed guest instructions (log scale)');ax.set_xticks(x,names,rotation=28,ha='right');ax.legend();figure_save(fig,'instructions')
fig,ax=plt.subplots(figsize=(9,4));vals=[base[n]['instructions']/best[n]['instructions'] for n in names]
ax.bar(names,vals,color=['#279d89']*4+['#69778a']*4);ax.axhline(1,color='black',lw=.8);ax.axhline(1.2,color='#b45e32',ls='--',label='1.2× target');ax.set_ylabel('C instructions / tuned Rust instructions');ax.tick_params(axis='x',rotation=28)
for i,v in enumerate(vals):ax.text(i,v+.025,f'{v:.2f}×',ha='center')
ax.set_ylim(0,max(vals)*1.16);ax.legend();figure_save(fig,'instruction-improvement')
fig,axs=plt.subplots(1,2,figsize=(10,4))
for ax,n in zip(axs,['conv3x3','fully_connected']):
    for im,color,label in [(False,'#69778a','direct'),(True,'#279d89','partial im2col')]:
        rs=[r for r in tuning[n] if r.get('correct') and r['config']['im2col']==im]
        if rs:ax.scatter([r['text_bytes'] for r in rs],[r['instructions'] for r in rs],s=24,c=color,label=label)
    ax.scatter(best[n]['text_bytes'],best[n]['instructions'],marker='*',s=140,c='#cf7e31',label='selected')
    ax.set_title(n);ax.set_xlabel('Whole benchmark .text bytes');ax.set_ylabel('Executed guest instructions');ax.legend(fontsize=8)
figure_save(fig,'tradeoffs')
fig,ax=plt.subplots(figsize=(10,4));alabels=[x[0] for x in labels]
for j,n in enumerate(primary):
    points={r['transformation']:r['relative_to_c'] for r in abl if r['workload']==n}
    ax.plot(range(len(alabels)),[points.get(l,float('nan')) for l in alabels],marker='o',label=n)
ax.set_xticks(range(len(alabels)),alabels,rotation=20,ha='right');ax.axhline(1,color='black',lw=.8);ax.set_ylabel('Instruction-count improvement over C');ax.legend();figure_save(fig,'ablation')
# Assembly facts: static counts are not dynamic counts. Include exact artifacts.
asm_rows=[]
for n in primary:
    for label,config in [('selected',best[n]),('b8/u16 regression',next((r for r in tuning[n] if r.get('variant')=='b8_u16_s0'),None))]:
        if config is None:continue
        asm=(ROOT/config['build_dir']/'assembly.txt').read_text()
        chunks=re.split(r'\n(?=[0-9a-f]+ <)',asm)
        chunk=next((x for x in chunks if re.search(r'^\w+ <.*3run',x)),None)
        if chunk is None:chunk=asm
        insns=re.findall(r'^\s*[0-9a-f]+:\s+([0-9a-f]+)\s+(\w+)\s*([^\n]*)',chunk,re.M)
        asm_rows.append(dict(workload=n,label=label,variant=config['variant'],instructions=config['instructions'],text_bytes=config['text_bytes'],stack_watermark_bytes=config['stack_watermark_bytes'],static_sp_loads_stores=sum(op in ('lw','sw') and '(sp)' in args for _,op,args in insns),static_multiply_ops=sum(op in ('mul','mulh','mulhu','mulhsu') for _,op,_ in insns),static_compressed_instructions=sum(len(code)==4 for code,_,_ in insns),artifact=config['build_dir']+'/assembly.txt'))
csv_write(RESULTS/'assembly-analysis.csv',asm_rows)
# Tables.
table='| Workload | ESP-NN C | Generic Rust | Tuned Rust | C / tuned | Selected |\n|---|---:|---:|---:|---:|---|\n'
for r in rows:table+=f"| {r['workload']} | {r['c_instructions']:,} | {r['generic_instructions']:,} | {r['tuned_instructions']:,} | {r['instruction_improvement']:.3f}× | `{r['variant']}` |\n"
mem='| Workload | C / tuned .text bytes | Tuned flash bytes | Static RAM | Stack watermark | Scratch included in stack |\n|---|---:|---:|---:|---:|---:|\n'
for r in rows:mem+=f"| {r['workload']} | {r['c_text_bytes']:,} / {r['tuned_text_bytes']:,} | {r['flash_bytes']:,} | {r['static_ram_bytes']:,} | {r['stack_watermark_bytes']:,} | {r['scratch_bytes']:,} |\n"
abtable='| Workload | Constant dimensions | + block 4 only | + unroll 4 only | + spatial unroll only | + interior split | + partial im2col only |\n|---|---:|---:|---:|---:|---:|---:|\n'
for n in primary:
    d={r['transformation']:r['instructions'] for r in abl if r['workload']==n}
    abtable+='| '+n+' | '+' | '.join(f'{d[l]:,}' if l in d else 'N/A' for l in alabels)+' |\n'
reg=json.loads((RESULTS/'regression.json').read_text()) if (RESULTS/'regression.json').exists() else []
checks=json.loads((RESULTS/'counter-crosscheck.json').read_text()) if (RESULTS/'counter-crosscheck.json').exists() else []
weak=[r['workload'] for r in rows if r['workload'] in primary and r['instruction_improvement']<1.2]
target_note=('All four requested workloads individually exceed 1.2× against ESP-NN.' if not weak else 'Workloads below 1.2×: '+', '.join(weak)+'.')
report=f'''# Scalar RV32IMC INT8 Rust kernel tuning: measured evaluation

## Outcome

The four requested shapes achieved **{summary['primary_geomean']:.3f}×** geometric-mean
instruction-count improvement over ESP-NN's C3-appropriate generic optimized C path.
All eight workloads achieved **{summary['all_geomean']:.3f}×**. These are QEMU executed
instruction ratios, not measured execution-time improvements or hardware cycles.
The three primary convolution operators alone achieve {gm(primary[:3]):.3f}×,
excluding fully connected (whose C3 upstream dispatch is ANSI).
The 1.2× instruction-count target was met on both main aggregates.
{target_note}

See also [the additional muRISCV-NN comparison](competitor-evaluation.md),
including further optimization and optional upstream quantization validation.

## Prior work and contribution

[Related work](related-work.md) records ESP-NN, CMSIS-NN, muRISCV-NN, microTVM,
AutoTVM/Ansor, MicroFlow, tract, and MLonMCU, with source links and inspected commits.
Blocking, im2col, unrolling, quantization and Rust inference all have prior art.
The contribution here is a small, auditable engineering artifact combining
shape-specific Rust generation, a bounded grid, explicit resource constraints,
independent integer validation, and calibrated QEMU measurements against real
optimized upstream C. No algorithmic novelty or general Rust language advantage
is claimed.

## Baseline gate and target environment

`results/baseline-milestone.json` was saved after C passed all eight workloads,
before Rust reference measurements or optimized source generation. The original
baseline is ESP-NN commit `f1bfc45cb0edfdbe8fd4871f3abc0adfc7fe1c47`.
Convolution uses `esp_nn_conv_s8_opt`, depthwise uses
`esp_nn_depthwise_conv_s8_opt`; C3's fully connected implementation is
`esp_nn_fully_connected_per_ch_s8_ansi`. It is the library's actual scalar dispatch,
not a newly written naive comparison. The generic Rust baseline was also saved
before the grid. `results/phase-e/` preserves the first search before im2col and
interior splitting were introduced.

Toolchains: `{base['conv3x3']['toolchains']['gcc']}`;
`{base['conv3x3']['toolchains']['rustc']}`;
`{base['conv3x3']['toolchains']['qemu']}`.
C uses `-O3 -march=rv32imc -mabi=ilp32`, freestanding, function/data sections,
no LTO. Rust uses stable 1.90.0, `riscv32imc-unknown-none-elf`, opt-level 3,
one codegen unit, panic abort, no LTO. Full flags/commands are saved in each row.
GCC and LLVM are different compilers, so this study does not isolate the language
from compiler and specialization effects.

QEMU runs bare-metal `virt` with the `lowrisc-ibex` RV32IMC CPU, one hart, 4 MiB
of emulator RAM and no firmware. The 4 MiB mapping is an emulator convenience;
per-workload deployment budgets are separately enforced. MMIO UART is at
0x10000000 and the test finisher at 0x100000. Startup and the measurement harness
use Zicsr for counter access; inference code is compiled for RV32IMC only.
No RVV, packed SIMD, floating point or hand-written inference assembly is used.
The `virt` machine is not an ESP32-C3 peripheral or timing model.

## What is measured

`-icount shift=0,align=off,sleep=off` enables deterministic guest instruction
counting. QEMU's RISC-V fixed-counter implementation uses `icount_get_raw()` for
`instret` ([source](https://github.com/qemu/qemu/blob/v10.0.0/target/riscv/csr.c)).
Two CSR reads enclose the same out-of-line function-pointer call for every
implementation. An empty callback measures common counter/call/argument-setup
overhead (15 instructions here), which is subtracted. The public kernel's
prologue, checks, parameters, scratch filling, accumulation, requantization,
clamping and return are included. Startup, fixture creation, validation, UART,
watermark scanning and shutdown are excluded. Raw and adjusted counts are saved.

Calibration loops execute exactly 2,000 or 4,000 loop instructions and return
2,001 / 4,001 including the CSR boundary instruction. Three fresh QEMU processes
per measurement produced identical counts and checksums. An independent QEMU
plugin counts each **executed** instruction via execution callbacks between the
same two PCs; it is not a count of translated instructions, translation blocks,
host operations or elapsed time. All {len(checks)} saved plugin cross-checks match
the CSR regions exactly. Counts assume normal kernel execution without faults
or asynchronous interrupts. QEMU host elapsed time is recorded only as search
cost and never used to select a kernel.

## Numerical contract and fixtures

Batch one NHWC activations; OHWI standard filters and HWC depthwise weights;
depthwise multiplier one; dilation one; stride/padding from TOML. Signed INT8
weights have zero point zero. Bias and accumulation are INT32. Default input
offset is -3, output offset +5, activation range [-111,103]. The exact per-channel
multiplier and shift arrays appear in each JSON row and `data.h`; workloads can
also supply explicit `multipliers` and `shifts` arrays. Default multiplier is
1073741824 + (channel mod 7) × 11234567; default shift is the specified
`quant_shift` minus channel mod 3. The default two-rounding ESP-NN requantization
path is used throughout, including negative rounding and saturation. No fast
non-bit-exact requantization option is enabled.

Xorshift32 seed 20261018 creates reproducible full-range signed inputs and
weights; biases use independent subsequent values. The Python INT64 oracle
implements operators independently. Every output byte, not only a checksum,
is compared in QEMU. Fixtures are checked to contain unsaturated outputs.
A host differential test covers 3,390 requantization cases against upstream C,
Python and Rust, including signs, ties, INT32 extremes, saturation and positive
shifts without overflow. Additional QEMU regression data includes {len(reg)}
operator executions (three new seeds × eight workloads × three implementations),
with three deterministic process repeats per execution. A real deliberately
incorrect kernel was compiled, executed and rejected; evidence is retained.
These tests are extensive samples, not an exhaustive proof for arbitrary tensors.

## Complete comparison

{table}

![Instruction counts](figures/instructions.svg)

![Instruction-count ratios](figures/instruction-improvement.svg)

Generic Rust is worse than C on every measured workload. Its dynamic dimensions,
index arithmetic, channel-loop control and remaining slice bounds checks are
visible in the disassembly. This result is retained rather than replacing the
baseline with a specialized Rust implementation.

## Generator and automatic search

The host Rust generator emits different scalar loop bodies. Direct variants
combine output-channel blocks 1/2/4/8, reduction unroll 1/4/8/16, and looped or
unrolled spatial taps. Odd channel counts exercise scalar tails. Additional
3×3 variants split interior and border pixels or use a one-output-pixel INT16
partial-im2col column. The column includes input offset and zero padding and is
filled inside the timed region. It never caches input-dependent work outside
the measurement. Im2col is not generated for depthwise or 1×1 operators.
Multipliers, shifts, inputs and weights remain runtime data in the generated
kernels. The extended search includes two-/four-pixel tiles, channel-first depthwise
loops and exact requantization restricted to nonnegative multipliers and
nonpositive shifts. Parameter checks stay inside the timed call. It does not
specialize individual multipliers or implement a new search algorithm. Bias, requantization and clamping
are fused in every variant; they are not claimed as an independently measured
new optimization. RV32M multiply and RV32C compression are compiler generated.

The final grid measured {summary['candidates']} candidates, with
{summary['failed_candidates']} incorrect/build-failed candidates and
{summary['regressing_candidates']} correct candidates slower in instruction count
than C. All losing configurations remain in `results/candidates.json` and the
per-workload tuning files. Some distinct source schedules compile to equivalent
code, especially when a reduction is smaller than the unroll factor. Grid search
is deterministic; ties prefer smaller flash. Failed correctness/build tests are
excluded from selection. Configuration, source, compiler versions/binary hashes, flags, QEMU
configuration, harness, oracle and upstream files enter the SHA-256 cache key.

Objectives are minimum instructions, minimum flash, minimum instructions under
the TOML flash budget, or performance/flash Pareto selection. All enforce the
specified SRAM/flash budgets. SRAM selection includes the full 64 KiB reserved
stack plus .data/.bss, so observed stack watermarks are not treated as a formal
bound. Im2col scratch lives in that stack and is not counted twice. Flash budgets
use the complete executable's loadable text/rodata/data, including test fixtures
and the oracle, which is conservative for deployment.

The latest saved search (which may use the cache) took {summary['search_seconds']:.2f} host seconds summed
across sequential workload searches. Summed candidate compilation time is
{summary['candidate_compile_seconds']:.2f} seconds; cache hits retain original
compile measurements, so this sum is not a fresh-run wall-clock estimate.
Host timings are machine-specific orchestration costs.

A labeled search-cost snapshot is preserved in `results/search-cost.json`;
timings can include cache hits. The earlier experiment is retained separately
in `results/phase-before-muriscv/`.

## Ablation and trade-offs

The following are actual one-factor candidates from the grid, not estimated
contributions. Columns start from constant-dimension b1/u1/looped direct Rust.
The interior split also requires spatial unrolling. Missing cells were not
applicable/generated. Transformations interact, so differences are not additive.

{abtable}

![Ablation](figures/ablation.svg)

![Code-size tradeoffs](figures/tradeoffs.svg)

Output blocking reuses each input load and its offset across several output
channels; unrolling amortizes induction and branch instructions. Too much of
both increases register pressure. Generated 3×3 kernels reuse one or several INT16
columns across output channels; its fill and padded zero multiplies
are included. The selected fully connected schedule and its code/memory costs appear in
the table; aggressive unrolling can be harmful on that register budget. Depthwise cannot share
one input value across output channels in the same way, and ESP-NN already
blocks four channels, leaving a smaller gain. Interior splitting duplicated
large bodies and did not improve the final selection.

## Code size and memory

{mem}

.text is the complete linked benchmark text, including harness and any linked
upstream fallback/core routines, rather than source-file size. Flash additionally
includes immutable fixtures, weights, quantization arrays and expected outputs;
these raw sizes and ELF section listings are retained. Static RAM is .data/.bss
and includes the output tensor. Inputs/weights are immutable flash data in this
harness; real applications commonly put activations in SRAM and must budget that
separately. There is no kernel heap allocation and no hidden persistent packing.

Stack is an observed memory-pattern watermark including a 256-byte guard and
harness frames, not a formal worst-case stack proof. Untouched frame space or
values matching the watermark pattern can affect it. The full stack reservation
is always reported and used for resource feasibility. Im2col scratch requirements
are explicit in generated source comments and JSON, and are included in the
observed stack figure. The preserved `conv_small` b4/u16 im2col candidate demonstrates that low scratch
size alone does not prevent a large compiler-generated frame from aggressive
unrolling; it is no longer the selected kernel.

## Assembly evidence

Every measured candidate retains its ELF, disassembly, section sizes, link map
and ISA attributes under its content hash in `results/build/` (recreated by the
benchmark commands). `results/assembly-analysis.csv` locates selected and
regressing examples and records static disassembly facts, explicitly distinct
from dynamic instruction counts. On fully connected, b8/u16 performs
{next(r['instructions'] for r in tuning['fully_connected'] if r['variant']=='b8_u16_s0'):,}
instructions versus {best['fully_connected']['instructions']:,} for b8/u1;
its larger frame and additional stack load/store sites are consistent with
register spilling. The loop bodies use signed `lb`, scalar `mul`/`add`, pointer
updates and branches; requantization also uses the scalar RV32M high-multiply
operations generated by the compiler. Compressed 16-bit encodings reduce bytes,
but each still counts as one instruction. Static counts of spills/compressed
instructions alone do not establish hardware latency savings.

## Limits and future ESP32-C3 validation

Instruction counts give a reproducible surrogate; they do not model C3 pipeline
latencies, flash/cache misses, memory bandwidth, branch costs, power or interrupts.
An extra instruction may be cheap and fewer loads may matter more on hardware.
The suite contains eight isolated layers, not complete neural networks or model
accuracy/latency evaluations. Shape specialization, known input/output offsets,
and GCC versus LLVM confound a pure language comparison. Per-channel multipliers
stay dynamic, but some other quantization/shape metadata is specialized. Library
C is compiled as reusable functions, while generated Rust has workload constants;
that is the intended generator comparison, not proof that generic Rust beats C.
Selected kernels may increase flash and stack, as the tables show. Resource
constraints cover this harness; deployment placement must be reassessed.

Future hardware work should use ESP-IDF/esp-hal on a real C3, pin clock/cache
settings, measure `mcycle`/`minstret` over identical kernels with interrupts
controlled, report medians/distributions and empty-region overhead, and compare
flash versus SRAM placement. Validate every output again and report actual cycle
ratios separately from these QEMU instruction ratios. No hardware backend or
hardware performance result is claimed in this release.

## Reproduction and artifacts

Run `bash scripts/reproduce.sh` after setup; see [README](../README.md).
The Rust CLI uses a Python build/QEMU/fixture backend, with all search and source
generation in Rust. `cargo run -p autotuner -- compare` regenerates this report,
JSON/CSV tables and SVG/PNG/PDF plots exclusively from saved measurements.
`cargo run -p autotuner -- export` emits independently reusable no_std files
with safe slice entry points and documented C ABI wrappers. No placeholder
performance values or hardcoded expected speedups are used.
'''
(DOCS/'report.md').write_text(report)
print(table)
print(f'Primary geometric mean: {gm(primary):.6f}x; all workloads: {gm(names):.6f}x; candidates: {summary["candidates"]}')
