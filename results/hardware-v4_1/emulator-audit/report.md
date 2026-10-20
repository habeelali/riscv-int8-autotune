# Two-model timing audit

Valid captures: 2. Emulation only: True. Independent repeatability evidence: False.

Validation checks raw file hashes, the exact host/device challenges, command sequence and random phases, all 1880 intermediate fingerprints against the LiteRT oracle, final inference fingerprints, delay calibration, device cycle/wall counters, and host/device elapsed-clock agreement. Distinct nonces alone are insufficient. Failed captures are retained.

| Capture | Model | Best tested C / Rust |
|---|---|
| emulator-1792409323669866368 | person | rust | held_out | 1.2336× |
| emulator-1792409323669866368 | person | rust | all | 1.2498× |
| emulator-1792409323669866368 | person | tile8_b2p4 | held_out | 1.3112× |
| emulator-1792409323669866368 | person | tile8_b2p4 | all | 1.2639× |
| emulator-1792409323669866368 | person | tile16_b2p4 | held_out | 1.2740× |
| emulator-1792409323669866368 | person | tile16_b2p4 | all | 1.2743× |
| emulator-1792409323669866368 | person | tile16_b2p4_block_sram | held_out | 1.2550× |
| emulator-1792409323669866368 | person | tile16_b2p4_block_sram | all | 1.2281× |
| emulator-1792409323669866368 | person | tile16_b4p2 | held_out | 1.2849× |
| emulator-1792409323669866368 | person | tile16_b4p2 | all | 1.2172× |
| emulator-1792409323669866368 | person | selected_rust | held_out | 1.3316× |
| emulator-1792409323669866368 | person | selected_rust | all | 1.2836× |
| emulator-1792409323669866368 | person | selected_hybrid | held_out | 1.2536× |
| emulator-1792409323669866368 | person | selected_hybrid | all | 1.2149× |
| emulator-1792409425073284600 | person | rust | held_out | 1.1157× |
| emulator-1792409425073284600 | person | rust | all | 1.1304× |
| emulator-1792409425073284600 | person | tile8_b2p4 | held_out | 1.1297× |
| emulator-1792409425073284600 | person | tile8_b2p4 | all | 1.2065× |
| emulator-1792409425073284600 | person | tile16_b2p4 | held_out | 1.2401× |
| emulator-1792409425073284600 | person | tile16_b2p4 | all | 1.2679× |
| emulator-1792409425073284600 | person | tile16_b2p4_block_sram | held_out | 1.1977× |
| emulator-1792409425073284600 | person | tile16_b2p4_block_sram | all | 1.2901× |
| emulator-1792409425073284600 | person | tile16_b4p2 | held_out | 1.1194× |
| emulator-1792409425073284600 | person | tile16_b4p2 | all | 1.1850× |
| emulator-1792409425073284600 | person | selected_rust | held_out | 1.1924× |
| emulator-1792409425073284600 | person | selected_rust | all | 1.2512× |
| emulator-1792409425073284600 | person | selected_hybrid | held_out | 1.1615× |
| emulator-1792409425073284600 | person | selected_hybrid | all | 1.2246× |
| emulator-1792409323669866368 | cifar10_resnet | rust | held_out | 1.1514× |
| emulator-1792409323669866368 | cifar10_resnet | rust | all | 1.2119× |
| emulator-1792409323669866368 | cifar10_resnet | tile8_b2p4 | held_out | 0.9769× |
| emulator-1792409323669866368 | cifar10_resnet | tile8_b2p4 | all | 1.0780× |
| emulator-1792409323669866368 | cifar10_resnet | tile16_b2p4 | held_out | 1.2247× |
| emulator-1792409323669866368 | cifar10_resnet | tile16_b2p4 | all | 1.2043× |
| emulator-1792409323669866368 | cifar10_resnet | tile16_b2p4_block_sram | held_out | 0.9916× |
| emulator-1792409323669866368 | cifar10_resnet | tile16_b2p4_block_sram | all | 1.0267× |
| emulator-1792409323669866368 | cifar10_resnet | tile16_b4p2 | held_out | 0.9264× |
| emulator-1792409323669866368 | cifar10_resnet | tile16_b4p2 | all | 0.9423× |
| emulator-1792409323669866368 | cifar10_resnet | selected_rust | held_out | 1.1761× |
| emulator-1792409323669866368 | cifar10_resnet | selected_rust | all | 1.2491× |
| emulator-1792409323669866368 | cifar10_resnet | selected_hybrid | held_out | 1.2206× |
| emulator-1792409323669866368 | cifar10_resnet | selected_hybrid | all | 1.2837× |
| emulator-1792409425073284600 | cifar10_resnet | rust | held_out | 1.1697× |
| emulator-1792409425073284600 | cifar10_resnet | rust | all | 1.1889× |
| emulator-1792409425073284600 | cifar10_resnet | tile8_b2p4 | held_out | 1.1648× |
| emulator-1792409425073284600 | cifar10_resnet | tile8_b2p4 | all | 1.1767× |
| emulator-1792409425073284600 | cifar10_resnet | tile16_b2p4 | held_out | 1.1759× |
| emulator-1792409425073284600 | cifar10_resnet | tile16_b2p4 | all | 1.1560× |
| emulator-1792409425073284600 | cifar10_resnet | tile16_b2p4_block_sram | held_out | 1.0168× |
| emulator-1792409425073284600 | cifar10_resnet | tile16_b2p4_block_sram | all | 0.9630× |
| emulator-1792409425073284600 | cifar10_resnet | tile16_b4p2 | held_out | 0.9622× |
| emulator-1792409425073284600 | cifar10_resnet | tile16_b4p2 | all | 0.9485× |
| emulator-1792409425073284600 | cifar10_resnet | selected_rust | held_out | 1.2411× |
| emulator-1792409425073284600 | cifar10_resnet | selected_rust | all | 1.1961× |
| emulator-1792409425073284600 | cifar10_resnet | selected_hybrid | held_out | 1.1476× |
| emulator-1792409425073284600 | cifar10_resnet | selected_hybrid | all | 1.1777× |

Hardware speedups must not be inferred from emulator output. When hardware captures pass, compare each boot separately and report the range across boots; there is no automatic claim of a confidence interval from three captures.
