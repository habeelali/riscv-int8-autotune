# Two-model timing audit

Valid captures: 2. Emulation only: True. Independent repeatability evidence: False.

Validation checks raw file hashes, the exact host/device challenges, command sequence and random phases, all 1880 intermediate fingerprints against the LiteRT oracle, final inference fingerprints, delay calibration, device cycle/wall counters, and host/device elapsed-clock agreement. Distinct nonces alone are insufficient. Failed captures are retained.

| Capture | Model | Best tested C / Rust |
|---|---|
| emulator-1792407188309451836 | person | rust | held_out | 1.1549× |
| emulator-1792407188309451836 | person | rust | all | 1.0691× |
| emulator-1792407188309451836 | person | tile8_b2p4 | held_out | 1.1116× |
| emulator-1792407188309451836 | person | tile8_b2p4 | all | 1.0842× |
| emulator-1792407188309451836 | person | tile16_b2p4 | held_out | 1.0125× |
| emulator-1792407188309451836 | person | tile16_b2p4 | all | 1.0122× |
| emulator-1792407188309451836 | person | tile16_b2p4_sram | held_out | 1.0791× |
| emulator-1792407188309451836 | person | tile16_b2p4_sram | all | 1.0477× |
| emulator-1792407188309451836 | person | tile16_b4p2 | held_out | 1.1346× |
| emulator-1792407188309451836 | person | tile16_b4p2 | all | 1.0809× |
| emulator-1792407188309451836 | person | selected_rust | held_out | 1.0636× |
| emulator-1792407188309451836 | person | selected_rust | all | 0.9831× |
| emulator-1792407188309451836 | person | selected_hybrid | held_out | 1.1901× |
| emulator-1792407188309451836 | person | selected_hybrid | all | 1.1545× |
| emulator-1792407290359336621 | person | rust | held_out | 0.9664× |
| emulator-1792407290359336621 | person | rust | all | 1.0037× |
| emulator-1792407290359336621 | person | tile8_b2p4 | held_out | 1.0090× |
| emulator-1792407290359336621 | person | tile8_b2p4 | all | 1.0227× |
| emulator-1792407290359336621 | person | tile16_b2p4 | held_out | 1.0287× |
| emulator-1792407290359336621 | person | tile16_b2p4 | all | 1.0580× |
| emulator-1792407290359336621 | person | tile16_b2p4_sram | held_out | 0.9723× |
| emulator-1792407290359336621 | person | tile16_b2p4_sram | all | 1.0039× |
| emulator-1792407290359336621 | person | tile16_b4p2 | held_out | 0.9724× |
| emulator-1792407290359336621 | person | tile16_b4p2 | all | 0.9983× |
| emulator-1792407290359336621 | person | selected_rust | held_out | 1.0329× |
| emulator-1792407290359336621 | person | selected_rust | all | 1.0412× |
| emulator-1792407290359336621 | person | selected_hybrid | held_out | 1.1472× |
| emulator-1792407290359336621 | person | selected_hybrid | all | 1.1692× |
| emulator-1792407188309451836 | cifar10_resnet | rust | held_out | 1.0725× |
| emulator-1792407188309451836 | cifar10_resnet | rust | all | 1.0981× |
| emulator-1792407188309451836 | cifar10_resnet | tile8_b2p4 | held_out | 0.4976× |
| emulator-1792407188309451836 | cifar10_resnet | tile8_b2p4 | all | 0.5071× |
| emulator-1792407188309451836 | cifar10_resnet | tile16_b2p4 | held_out | 1.1134× |
| emulator-1792407188309451836 | cifar10_resnet | tile16_b2p4 | all | 1.0624× |
| emulator-1792407188309451836 | cifar10_resnet | tile16_b2p4_sram | held_out | 0.9707× |
| emulator-1792407188309451836 | cifar10_resnet | tile16_b2p4_sram | all | 1.0313× |
| emulator-1792407188309451836 | cifar10_resnet | tile16_b4p2 | held_out | 0.8072× |
| emulator-1792407188309451836 | cifar10_resnet | tile16_b4p2 | all | 0.8209× |
| emulator-1792407188309451836 | cifar10_resnet | selected_rust | held_out | 1.1506× |
| emulator-1792407188309451836 | cifar10_resnet | selected_rust | all | 1.1357× |
| emulator-1792407188309451836 | cifar10_resnet | selected_hybrid | held_out | 1.1411× |
| emulator-1792407188309451836 | cifar10_resnet | selected_hybrid | all | 1.1347× |
| emulator-1792407290359336621 | cifar10_resnet | rust | held_out | 1.0757× |
| emulator-1792407290359336621 | cifar10_resnet | rust | all | 1.0645× |
| emulator-1792407290359336621 | cifar10_resnet | tile8_b2p4 | held_out | 0.5047× |
| emulator-1792407290359336621 | cifar10_resnet | tile8_b2p4 | all | 0.5091× |
| emulator-1792407290359336621 | cifar10_resnet | tile16_b2p4 | held_out | 1.0567× |
| emulator-1792407290359336621 | cifar10_resnet | tile16_b2p4 | all | 1.0663× |
| emulator-1792407290359336621 | cifar10_resnet | tile16_b2p4_sram | held_out | 1.1031× |
| emulator-1792407290359336621 | cifar10_resnet | tile16_b2p4_sram | all | 1.1152× |
| emulator-1792407290359336621 | cifar10_resnet | tile16_b4p2 | held_out | 0.8479× |
| emulator-1792407290359336621 | cifar10_resnet | tile16_b4p2 | all | 0.8327× |
| emulator-1792407290359336621 | cifar10_resnet | selected_rust | held_out | 1.1215× |
| emulator-1792407290359336621 | cifar10_resnet | selected_rust | all | 1.1078× |
| emulator-1792407290359336621 | cifar10_resnet | selected_hybrid | held_out | 1.1536× |
| emulator-1792407290359336621 | cifar10_resnet | selected_hybrid | all | 1.1311× |

Hardware speedups must not be inferred from emulator output. When hardware captures pass, compare each boot separately and report the range across boots; there is no automatic claim of a confidence interval from three captures.
