# Research summary

This project is a software tool and empirical systems study, not a claim that Rust
alone improves neural-network performance. Its reusable output is specialized
scalar RV32IMC `no_std` code. The host CLI generates and benchmarks kernels; the
ESP32-C3 firmware validates complete models on physical hardware.

## Supported contribution

The implemented workflow combines workload specialization, exact integer
validation, calibrated QEMU executed-instruction measurements, bounded search with
resource constraints, reusable source export and auditable physical model timing.
The published artifact includes unfavorable generic Rust results, losing schedules,
code-size costs and failed physical/simulated sessions.

The practical CLI turns this workflow into a repeatable input/output interface:
TOML workload → C and generic baselines → measured candidates → dependency-free
kernel crate, JSON/CSV, Pareto frontier and offline report. It preserves historical
results rather than replacing them with a new user's search.

## Established techniques used

- Channel blocking and spatial register tiling reuse inputs, weights and accumulators.
- Partial im2col and signed-INT8 patch tiles trade scratch work for simpler inner loops.
- Offline weight packing improves traversal for the fixed model export family.
- Bias folding removes repeated zero-point compensation only under the proved padding contract.
- Restricted requantization specializes exact arithmetic after its bounds are validated.
- Sharing identical generated function bodies reduces flash while preserving instruction counts.

None of these component techniques is asserted to be new. The project does not
establish a new asymptotic convolution algorithm or a first-ever Rust ML claim.
The [novelty audit](novelty-audit.md) and [related-work review](related-work.md)
compare the scope with MicroFlow, CubeCL, RISC-V libraries, TVM and other systems.
A targeted literature review does not establish absence of prior art.

## What was measured

| Experiment | Platform/metric | Supported conclusion |
|---|---|---|
| Operator search | Generic RV32IMC QEMU, executed instructions | Tuned schedules improve the aggregate versus tested C; generic Rust loses |
| Person-model flash deduplication | Archived QEMU/model build | 24.6% less generated code, identical checked instruction counts |
| Cache-tile candidates | Generic QEMU, instructions and linked text | Tile schedules expose instruction/code/scratch trade-offs; staging can regress |
| V3 physical sessions | One ESP32-C3 | Person win, ResNet loss |
| V4/V4.1 physical returns | Same board | Staging experiments can stall; one V4.1 session wins but fails repeatability |
| V5 fixed physical sessions | Same board, three hard resets | Repeatable 1.452× person and 1.749× ResNet speedups versus fastest tested C |
| New CLI validation | Actual QEMU plus additional host reference checks | Four custom operator workloads export working reusable code; failures/resume are exercised |

V5's equal-model geometric mean is **1.593×**. It uses fixed non-staging kernels;
it does not promote the faster staging-dependent V4.1 mixed map. Both compared
graphs retain common C operators. The exact model/fixture provenance and arithmetic
conventions are retained with the results.

## Threats to validity

One board and three hard resets do not establish power-cycle, cross-board or
long-duration stability. Four known inputs per model are functional fixtures,
not a dataset accuracy evaluation. Whole-model timings are implementation and
memory-layout comparisons; they do not isolate a language effect. QEMU executed
instructions are not board cycles, cache timing or energy. Benchmark linked flash
includes data/oracles and is not always isolated kernel deployment size.

ESP32-C3 QEMU stalls are undiagnosed. Successful physical V5 captures do not prove
the emulator caused them. V4/V4.1 hardware stalls are preserved. Exported kernels
have an INT32/no-aliasing arithmetic contract; changing trained parameters requires
application-specific validation. The practical CLI's simple runtime-weight grid
is not the same pipeline as the offline-packed two-model hardware export.

No direct comparable runtime experiment was performed against MicroFlow, CubeCL
or other untested systems, so this artifact does not claim to beat them. The
executed comparisons are against the specific pinned ESP-NN/muRISCV-NN variants
reported in the measurement files.

## Next experiments

Capture a real stalled PC/stack and resolve the shared runtime uncertainty.
Independently rerun on power cycles and additional boards, evaluate more models
and input sets, and report deployment-only size and energy with explicit
measurement methods. Validate any event-counter reprogramming separately because
ESP32-C3 SDK cycle timing uses the same custom performance counter. Broader
hardware-aware search should follow reliable measurement, not replace it.
