# Two-model timing audit

Valid captures: 1. Emulation only: True. Independent repeatability evidence: False.

Validation checks raw file hashes, the exact host/device challenges, command sequence and random phases, all 752 intermediate fingerprints against the LiteRT oracle, final inference fingerprints, delay calibration, device cycle/wall counters, and host/device elapsed-clock agreement. Distinct nonces alone are insufficient. Failed captures are retained.

| Capture | Model | Best tested C / Rust |
|---|---|
| emulator-1792404230454733880 | person | 1.3393× |
| emulator-1792404230454733880 | cifar10_resnet | 1.0518× |

Hardware speedups must not be inferred from emulator output. When hardware captures pass, compare each boot separately and report the range across boots; there is no automatic claim of a confidence interval from three captures.
