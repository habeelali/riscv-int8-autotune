# Preliminary novelty audit

Full-model evidence is now available in the [pretrained person-detector evaluation](model-evaluation.md).
It strengthens the empirical evaluation; it does not establish new algorithms or
a first-ever Rust inference claim.

Follow-up: the [additional muRISCV-NN evaluation](competitor-evaluation.md) now
contains an executed comparison and additional generated schedules. The audit
below records the earlier search and does not assert priority for those changes.

Search date: 2026-10-18. This is a targeted prior-art review, not a systematic
literature review or proof of priority. Search results were checked against
primary papers, official project documentation, and selected source files.
No external project was benchmarked during this review. Upstream README claims
are not treated as independently reproduced results.

## Assessment

The broad claims "first Rust TinyML engine", "first generated Rust INT8 kernels",
"first Rust kernel autotuner", and "first Rust ML on RISC-V" are not defensible.
Existing projects cover those areas.

The narrower **candidate systems contribution** is an offline Rust source
autotuner for scalar RV32IMC INT8 operators that combines generated computation
loops, tested integer equivalence to ESP-NN, flash/SRAM-constrained selection,
and deterministic QEMU executed-instruction regions independently checked by
an execution-callback plugin. This search did not identify an exact equivalent
of that complete workflow. Absence from the search does not establish that this
is the first implementation, nor make the component optimizations new.

The present project establishes an engineering artifact and empirical result.
It does not establish a new scheduling algorithm or demonstrate that Rust as a
language is faster than C.

## Closest work

| Primary source | Established overlap | Distinction supported by the inspected material |
|---|---|---|
| [MicroFlow paper](https://arxiv.org/abs/2409.19432), first submitted 2024; [repository](https://github.com/matteocarnelos/microflow-rs) | Rust TinyML, compiler-based deployment, quantized operators, compile-time dimensions, performance and memory evaluation | The inspected implementation generates model code and uses const-generic operator implementations. I did not identify an offline RV32IMC schedule grid evaluated against ESP-NN through the CSR/plugin workflow. This is a limited inspection finding. |
| [embedded-nn](https://github.com/leftger/embedded-nn) | Pure Rust no_std quantized kernels, Rust/C code generation, memory planning and model-level Pareto tooling | Its inspected Rust emitter calls runtime operators. The inspected `embedded-nn-train/src/pareto.rs` explores model architectures and contains `estimated_cycles` expressions; that is different from compiling and instruction-counting alternative implementations of one fixed layer. This finding applies to that file/commit, not every feature of the repository. |
| [CubeCL autotuning documentation](https://burn.dev/books/cubecl/core-features/autotune.html), [repository](https://github.com/tracel-ai/cubecl) | Rust compute kernel generation, measured kernel/configuration selection, compile-time configuration and caching | Rust autotuning already exists. Our focus is offline scalar MCU source export and the specific bare-metal measurement/quantization contract, rather than the documented general compute/runtime workflow. This does not claim CubeCL can never target such hardware. |
| [edge-infer](https://github.com/idan-ben-ami/edge-infer) | Generates no_std Rust inference code, const-generic specialization, QEMU validation and stack measurement | The inspected README explicitly describes INT8 weights being dequantized for floating-point accumulation and identifies full-integer inference as future work. It is not the same integer-only INT8/INT32 kernel contract. Its current generator is Python; Rust is the output language. |
| [Ariel-ML preprint](https://arxiv.org/abs/2512.09800) | Embedded Rust platform, RISC-V support, automated inference optimization and evaluation | Its described approach integrates IREE and a C runtime for multi-core execution. Rust orchestration/platform support should not be confused with all kernels being generated as Rust source. Our current experiment is single-core scalar kernel scheduling. |
| [RISCVML official 2026 poster](https://cfp.riscv-europe.org/eu-summit-2026/talk/UMWQJ8/) | Rust embedded ML on ESP32-C3/C6/P4 | The abstract describes education and an ESP-DL/FFI capstone, not this measured scalar kernel autotuning workflow. It rules out presenting Rust + ESP32-C3 + ML itself as new. |
| [muRISCV-NN](https://github.com/tum-ei-eda/muriscv-nn) | Optimized RISC-V quantized kernels, integer compatibility, simulator evaluation and framework integration | Already important RISC-V prior art. Its documented V/P acceleration differs from this scalar RV32IMC-only experiment. This is not a claim that it has no scalar code. |
| [AutoTVM paper](https://arxiv.org/abs/1805.08166), [Ansor paper](https://arxiv.org/abs/2006.06762) | Automated tensor implementation search, schedule transformations and cost models | Grid search, blocking, unrolling and automated specialization are established methods. Emitting Rust does not make these transformations algorithmically novel. |

## Source-level checks and immutable references

* MicroFlow commit `6d193da21b8cc973db9ee94faec622cc0af2f71f`:
  [const-generic convolution](https://github.com/matteocarnelos/microflow-rs/blob/6d193da21b8cc973db9ee94faec622cc0af2f71f/src/ops/conv_2d.rs).
  It accumulates products in INT32 and uses floating-point constants/rounding in
  its inspected output epilogue. Thus neither const-generic quantized Rust
  operators nor compile-time preprocessing is new here. This observation is not
  a performance or correctness criticism of its different numerical contract.
* embedded-nn commit `69ea4187a298431fcaa313c3ab2a2ec1c473c4c7`:
  [per-channel convolution](https://github.com/leftger/embedded-nn/blob/69ea4187a298431fcaa313c3ab2a2ec1c473c4c7/crates/embedded-nn/src/convolution.rs),
  [Rust emitter](https://github.com/leftger/embedded-nn/blob/69ea4187a298431fcaa313c3ab2a2ec1c473c4c7/crates/embedded-nn-codegen/src/emit_rust.rs),
  [model Pareto routine](https://github.com/leftger/embedded-nn/blob/69ea4187a298431fcaa313c3ab2a2ec1c473c4c7/crates/embedded-nn-train/src/pareto.rs).
  These source files demonstrate real overlap with Rust integer operators and
  code generation; broad first-in-Rust claims would ignore this prior work.
* edge-infer commit `5eac90cc33ef06130571688aae012dd797d024ac`:
  [implementation notes and roadmap](https://github.com/idan-ben-ami/edge-infer/blob/5eac90cc33ef06130571688aae012dd797d024ac/README.md).

Code inspection covered these files and targeted text searches, not exhaustive
program analysis. No "missing feature" statement should be generalized to
uninspected branches, other implementations, unpublished work or future releases.

## Claim wording that the current evidence supports

> We present an open-source offline autotuning workflow that generates reusable
> no_std Rust INT8 kernels for scalar RV32IMC processors. The workflow checks
> numerical outputs, measures isolated QEMU executed-instruction regions with an
> independent counter cross-check, and selects implementations under executable
> flash and reserved-SRAM constraints. In the evaluated four-layer suite it
> achieves a 1.678× geometric-mean instruction-count improvement over ESP-NN's
> C3-appropriate scalar dispatch, with explicit code-size and stack trade-offs.

This is a descriptive contribution statement. It deliberately contains no
unproven priority claim. The C3 fully connected dispatch is ANSI C; the three
primary convolution workloads alone achieve 1.488×. No real hardware timing
improvement has been established.

## Stronger research questions, not existing results

A promising focus is **the interaction between safe Rust indexing, source
specialization and register pressure on scalar RV32IMC**. Our current disassembly
and losing candidates motivate this question but do not isolate its causes.

A controlled evaluation should add:

1. Shape-specialized C implementing the same schedules, compiled with Clang/LLVM
   as well as GCC. Keep quantization, packing/setup, buffer layout and resource
   limits identical. This separates source specialization and compiler effects
   from a language effect.
2. Matched safe-slice and carefully checked unsafe Rust variants, with the same
   schedule, to quantify actual safety-check costs. The current safe compute loops
   have a small unsafe FFI boundary; they are not formally verified.
3. A larger held-out shape/model suite and real ESP32-C3 measurements to test
   whether QEMU-selected schedules transfer to hardware. Evaluate whole-model
   latency, memory and code size separately from isolated layer counts.
4. If proposing a new spill-aware selector or cost model, compare it against
   exhaustive search and existing tuning methods on held-out workloads. Adding a
   heuristic alone does not establish novelty; it must yield a new, supported
   result such as lower search cost at equal quality or better hardware selection.

These experiments could support a more substantial empirical or methodological
contribution. Their outcomes and publishability are not guaranteed, and none is
claimed as already implemented in this audit.

## Search scope

Targeted combinations included Rust / no_std / INT8 / TinyML / RV32IMC /
scalar RISC-V / ESP-NN / QEMU / instruction count / code generation / autotuning,
plus named-project searches. Sources included GitHub repositories, project
manuals, arXiv papers and the official RISC-V Summit poster abstract. Negative
search results are weak evidence: indexing gaps, alternate terminology and
unpublished implementations remain possible. A formal publication should expand
this into a documented systematic literature and citation review.
