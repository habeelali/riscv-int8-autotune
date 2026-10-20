# Generated dense_demo kernel

A standalone MIT-licensed `no_std` crate with no dependencies or allocation.
Add this directory as a path dependency, then call `riscv_int8_dense_demo::run(input, weights, bias, multipliers, shifts, output)`.
The containing firmware supplies startup, a linker layout and panic handling.

```sh
cargo check --manifest-path Cargo.toml --target riscv32imc-unknown-none-elf
```

Geometry is fixed by workload.toml in the parent directory. Inputs and outputs use NHWC; standard convolution weights use OHWI, depthwise HWC and FC [out,in]. Weights are ordinary signed INT8, not packed by this workflow. Runtime weights and quantization arrays may differ from the benchmark fixture, provided this contract holds.

Buffer lengths: input 256, weights 32768, output 128; bias/multiplier/shift each 128. Signed INT8 data; INT32 bias and accumulation; per-channel nonnegative Q31 multipliers; shifts -30..0. Two-round requantization, fixed offsets and activation clamp from the workload. Accumulation and every intermediate arithmetic expression must fit INT32. Output must not alias the input or parameter buffers. Safe Rust checks lengths; C callers must satisfy lengths, alignment and non-aliasing themselves.

Explicit stack scratch: 0 bytes. Measured whole-harness stack watermark: 304 bytes; it is not a bound on application interrupts or total stack needs. No heap allocation occurs in the kernel. `kernel.h` declares the optional C ABI symbol. Do not link multiple exports with the same workload name into one binary.

The measured C/selected instruction ratio is 1.1229x. A value below 1 is slower than the fastest tested C. This is QEMU instruction count, not board latency. `manifest.json` retains configuration, exact compiler flags, fixture quantization and measurements. A deterministic test fixture validates the candidate; verify your application tensors as well.
