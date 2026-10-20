# Two-model timing audit

Valid captures: 3. Emulation only: False. Independent repeatability evidence: True.

Validation checks raw file hashes, the exact host/device challenges, command sequence and random phases, all 752 intermediate fingerprints against the LiteRT oracle, final inference fingerprints, delay calibration, device cycle/wall counters, and host/device elapsed-clock agreement. Distinct nonces alone are insufficient. Failed captures are retained.

| Capture | Model | Best tested C / Rust |
|---|---|
| boot-1 | person | 1.4517× |
| boot-2 | person | 1.4517× |
| boot-3 | person | 1.4517× |
| boot-1 | cifar10_resnet | 1.7491× |
| boot-2 | cifar10_resnet | 1.7491× |
| boot-3 | cifar10_resnet | 1.7491× |

Hardware speedups must not be inferred from emulator output. When hardware captures pass, compare each boot separately and report the range across boots; there is no automatic claim of a confidence interval from three captures.
