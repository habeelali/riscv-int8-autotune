# Related work and scope

A later [targeted novelty audit](novelty-audit.md) adds CubeCL, embedded-nn,
edge-infer, Ariel-ML and RISCVML, with source-level checks and narrower claim wording.
The original pre-optimization survey below is retained as the historical baseline.

Sources inspected before kernel specialization (2026-10-18). This is an engineering
experiment, not a claim of new scheduling or quantization algorithms.

| Work | Relevant established techniques | Relationship to this experiment |
|---|---|---|
| [ESP-NN](https://github.com/espressif/esp-nn), `f1bfc45cb0edfdbe8fd4871f3abc0adfc7fe1c47` | C3 generic scalar optimization: 4-way reduction unrolling in convolution; specialized pointwise path; four depthwise accumulators; per-channel integer quantization | Primary baseline: original, unmodified upstream sources compiled at `-O3`. Fully connected dispatches to ANSI C on C3; this limitation is explicit. No ESP32-S3 or P4 instructions are used. |
| [CMSIS-NN](https://github.com/ARM-software/CMSIS-NN), inspected `71cbe3d5a686c875c88a4d112633b13fae94b5a2` | Portable C plus Arm DSP/MVE implementations, im2col/matrix kernels, TFLM integer conventions | Important prior art. Arm intrinsics are not RV32IMC baselines. ESP-NN portable optimized sources compiled successfully, so no fallback was necessary. |
| [muRISCV-NN](https://github.com/tum-ei-eda/muriscv-nn), inspected `f69494442858980be78180868d96c3524dbf26f9` | CMSIS-NN-compatible scalar/vector kernels; RISC-V V and packed P acceleration; TFLM/microTVM integration | Direct RISC-V prior art. V/P performance results do not transfer to C3's scalar ISA. See [CF 2024 paper](https://doi.org/10.1145/3637543.3652878). |
| [microTVM v0.18](https://github.com/apache/tvm/blob/v0.18.0/docs/topic/microtvm/index.rst), [AutoTVM](https://arxiv.org/abs/1805.08166), [Ansor](https://arxiv.org/abs/2006.06762) | Code generation, schedule search, device measurement, memory-aware deployment | Autotuning, blocking, unrolling and generated constant dimensions are established. This project uses a much smaller deterministic search with auditable QEMU counts. |
| [MicroFlow](https://github.com/matteocarnelos/microflow-rs) | Compile-time Rust model processing, source generation, no_std runtime and quantization | Rust embedded inference and generated source already exist. Rust alone is no novelty. |
| [tract](https://github.com/sonos/tract) | Model optimization, quantized inference and backend-specific execution | Wider Rust inference ecosystem; no assertion that this project outperforms tract. |
| [MLonMCU](https://github.com/tum-ei-eda/mlonmcu) | RISC-V TinyML deployment, reproducible simulator/hardware evaluation | Existing measurement/deployment infrastructure; this project is a smaller standalone experiment rather than a replacement. |

The concrete contribution sought here is a reusable, bounded Rust source search
for four scalar INT8 operators, with bit-exact validation against a separate host
oracle, a pinned C3-appropriate optimized C baseline, calibrated deterministic
QEMU instruction regions, and preserved losing candidates. These are reproducible
engineering artifacts; a publication would need a broader literature review,
more models and real hardware before claiming a research advance.

## Quantization contract

Batch one, NHWC activations, OHWI standard filters, HW-channel depthwise filters,
channel multiplier one, dilation one, symmetric signed weights (zero point zero).
Input zero point is `-input_offset`; padding contributes zero *after* offset.
Bias is INT32 and per-output-channel multiplier/shift arrays are shared by all
implementations. Requantization uses ESP-NN's default two-rounding path (no
`SKIP_NUDGE`): saturating rounding doubling high multiply with signed nudge and
truncation toward zero, then rounding division by a power of two. Output offset
and activation clamp are applied last. Validated workloads avoid INT32 overflow.
