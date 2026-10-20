# Two-model timing audit

Valid captures: 1. Emulation only: False. Independent repeatability evidence: False.

Validation checks raw file hashes, the exact host/device challenges, command sequence and random phases, all 1880 intermediate fingerprints against the LiteRT oracle, final inference fingerprints, delay calibration, device cycle/wall counters, and host/device elapsed-clock agreement. Distinct nonces alone are insufficient. Failed captures are retained.

| Capture | Model | Best tested C / Rust |
|---|---|
| boot-1 | person | rust | held_out | 1.4529× |
| boot-1 | person | rust | all | 1.4529× |
| boot-1 | person | tile8_b2p4 | held_out | 1.4529× |
| boot-1 | person | tile8_b2p4 | all | 1.4529× |
| boot-1 | person | tile16_b2p4 | held_out | 1.4529× |
| boot-1 | person | tile16_b2p4 | all | 1.4529× |
| boot-1 | person | tile16_b2p4_block_sram | held_out | 1.4529× |
| boot-1 | person | tile16_b2p4_block_sram | all | 1.4529× |
| boot-1 | person | tile16_b4p2 | held_out | 1.4529× |
| boot-1 | person | tile16_b4p2 | all | 1.4529× |
| boot-1 | person | selected_rust | held_out | 1.4529× |
| boot-1 | person | selected_rust | all | 1.4529× |
| boot-1 | person | selected_hybrid | held_out | 1.4529× |
| boot-1 | person | selected_hybrid | all | 1.4529× |
| boot-1 | cifar10_resnet | rust | held_out | 0.9506× |
| boot-1 | cifar10_resnet | rust | all | 0.9506× |
| boot-1 | cifar10_resnet | tile8_b2p4 | held_out | 1.6749× |
| boot-1 | cifar10_resnet | tile8_b2p4 | all | 1.6749× |
| boot-1 | cifar10_resnet | tile16_b2p4 | held_out | 1.7614× |
| boot-1 | cifar10_resnet | tile16_b2p4 | all | 1.7614× |
| boot-1 | cifar10_resnet | tile16_b2p4_block_sram | held_out | 1.7517× |
| boot-1 | cifar10_resnet | tile16_b2p4_block_sram | all | 1.7517× |
| boot-1 | cifar10_resnet | tile16_b4p2 | held_out | 1.6146× |
| boot-1 | cifar10_resnet | tile16_b4p2 | all | 1.6146× |
| boot-1 | cifar10_resnet | selected_rust | held_out | 1.8073× |
| boot-1 | cifar10_resnet | selected_rust | all | 1.8073× |
| boot-1 | cifar10_resnet | selected_hybrid | held_out | 1.8073× |
| boot-1 | cifar10_resnet | selected_hybrid | all | 1.8073× |

Hardware speedups must not be inferred from emulator output. When hardware captures pass, compare each boot separately and report the range across boots; there is no automatic claim of a confidence interval from three captures.
