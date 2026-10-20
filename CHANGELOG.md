# Changelog

## 0.1.0 — 2026-10-19

- Added the `riscv-int8` executable with workload templates, environment diagnostics,
  geometry inspection and an isolated measure/tune/export workflow.
- Added standalone no_std Cargo exports, C ABI headers, JSON/CSV measurements,
  Pareto results, resume protection and offline HTML reports.
- Preserved scalar RV32IMC reference, code generation, optimized C comparisons,
  QEMU counter calibration and historical search/flash experiments.
- Audited V5 physical results across three hard-reset sessions on one ESP32-C3:
  1.452× person detector and 1.749× ResNet versus fastest tested C controls;
  1.593× equal-model geometric mean, zero mismatches.
- Added data-derived publication figures and a LinkedIn media kit.
- Known issue: intermittent ESP32-C3 simulation stalls and older staging-firmware
  hardware stalls remain undiagnosed. No general reliability claim is made.
