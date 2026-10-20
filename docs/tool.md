# Use riscv-int8 as a tool

`riscv-int8` is a local command-line tool: workload TOML in, measured and reusable
scalar `no_std` kernel crate out. It needs the source checkout and Python-backed
measurement scripts at runtime. Installing the executable alone does not bundle
the toolchain or Python scripts. Linux x86-64 is the validated host.

## Install and check

```bash
bash scripts/setup.sh
source scripts/env.sh
cargo install --locked --path crates/autotuner
riscv-int8 --help
riscv-int8 doctor
```

Run inside the checkout, or set `RISCV_INT8_ROOT=/absolute/path/to/riscv-int8-autotune`.
The tool detects the checkout from the current directory, then uses the build-time
checkout as a fallback. Set that variable after moving an installed tool's source.
`doctor --json` provides machine-readable checks and actionable failures.

## Create a workload and build

```bash
riscv-int8 init my_conv.toml --operator conv
# Edit dimensions, offsets, per-channel quantization and resource budgets.
riscv-int8 inspect my_conv.toml
riscv-int8 build my_conv.toml --out artifacts/my_conv --max-candidates 24
```

Templates are available for `conv`, `pointwise`, `depthwise`, and `fc`. Geometry is
fixed at generation time. Activations are NHWC. Standard convolution weights are
OHWI; depthwise weights HWC; fully connected weights [output,input]. The current
simple workload interface supports batch one, dilation one, 1×1 or 3×3 kernels,
and depthwise multiplier one. Quantization and INT32 accumulation must satisfy
the exported contract. `inspect` validates the specification before execution.

`build` performs real work in this order:

1. Validate and measure optimized ESP-NN, then stock scalar and corrected optional
   RV32-assembly muRISCV-NN, using the same deterministic tensors and quantization.
2. Validate and measure generic Rust.
3. Generate different Rust loop bodies, cross-compile and run each in three fresh
   QEMU processes, reject incorrect candidates, and enforce flash/SRAM budgets.
4. Select an eligible candidate, save all records and a Pareto frontier, export a
   standalone dependency-free crate, cross-check its target compilation, and render
   an offline HTML report.

The default limit of 24 selects evenly distributed indices across the deterministic
grid. `--full-search` evaluates every grid candidate. A bounded search is not a
claim of global optimality; some selections can lose to C. Every report prints the
actual ratio, including values below one. `--objective min-flash` selects smallest
linked benchmark flash; `min-instructions`, `budget` and `pareto` select the fastest
feasible candidate (the latter also makes the frontier explicit). Every successful
job writes its complete frontier regardless of objective.

```bash
riscv-int8 build my_conv.toml --out artifacts/compact --objective min-flash
riscv-int8 build my_conv.toml --out artifacts/full --full-search
riscv-int8 build my_conv.toml --out artifacts/my_conv --max-candidates 24 --resume
```

Jobs use their own measurement directory and do not replace the historical research
selections. Shared upstream compilation caches may be reused. `--resume` requires
exactly the same workload bytes, candidate limit and objective. Cache keys include
source/configuration and compiler identities; changes trigger real remeasurement.
A failed job retains its status, errors and completed candidate records. Existing
nonempty output directories are protected unless `--resume` is explicit. Use a
separate output directory for each concurrent job; concurrent writes to the same
job are not supported.

## Use the result

```text
artifacts/my_conv/
├── workload.toml          # exact input specification
├── job.json               # identity, environment, completion or failure
├── baselines.json         # optimized C and generic Rust
├── candidates.json/.csv   # every evaluated candidate and rejection
├── pareto.json            # instructions/flash frontier under budgets
├── summary.json           # selected result, counts, flags and provenance
├── report.html            # readable offline report
├── instructions.svg       # generated from this job's measurements
├── measurements/build/    # cache, ELF, assembly, commands and QEMU output
└── export/
    ├── Cargo.toml
    ├── lib.rs
    ├── kernel.rs
    ├── kernel.h
    ├── manifest.json
    ├── LICENSE
    └── README.md
```

Add the exported directory as a Cargo path dependency; it requires no workspace
or external crates. Call its safe `run` function with caller-owned buffers. The
parent application supplies panic handling, startup and linking. `kernel.h`
provides a C ABI declaration with the same non-aliasing and buffer-length contract.

```toml
# In your firmware Cargo.toml:
[dependencies]
riscv-int8-my-conv = { path = "../artifacts/my_conv/export" }
```

```rust,ignore
riscv_int8_my_conv::run(input, weights, bias, multipliers, shifts, output);
```

Unlike model-specific cache-tiled exports, this simple workflow accepts ordinary
unpacked runtime weights and runtime per-channel quantization arrays. Its
specialized geometry and offsets must match the application. It does not import
arbitrary TFLite models or claim to automatically reproduce the V5 model timings.
The two model import/export pipelines remain separate research workflows.

## What the metrics mean

Counts are dynamically executed guest instructions with calibrated overhead, not
hardware cycles or latency. One deterministic tensor fixture per workload is
checked numerically; three QEMU executions validate counter repeatability, not
three different inputs. Additional application tensors must be tested.

Resource constraints use the linked benchmark flash, including immutable test data
and oracle, plus static RAM and the complete 64 KiB reserved stack. Local scratch
sits inside that stack; its size and measured watermark are reported separately.
This is conservative harness accounting, not the isolated deployed kernel size.

## Verify the tool itself

```bash
cargo build --locked -p autotuner
python3 scripts/test_tool_e2e.py
```

The end-to-end suite uses custom odd-channel and non-square workloads, actually
runs QEMU for all four operators, checks standalone exports against extra Python
reference fixtures, exercises resume/rejection behavior, and verifies the
historical result files remain unchanged.
