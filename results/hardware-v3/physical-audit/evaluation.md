# Independently triggered board timings: v3

## Findings

All three raw capture sets pass the local auditor in physical mode, using the locally generated tensor oracle rather than executable code from the returned ZIP. There are 2,256 intermediate tensor checks, 480 whole-model timing observations, nine delay calibrations and zero reported mismatches. Host sidecar hashes, every measurement request/response, randomized phase and schedule, and output fingerprints match. The image/model build ID is 1833fe83590e5171.

The person-detector result repeats across hardware resets: generated Rust takes 259.23 ms versus 376.13 ms for the fastest tested C configuration (the documented corrected scalar muRISCV-NN). This is 1.451×, or 31.08% less inference time. It is a hardware timing result, not a QEMU instruction-count result.

The second model exposes a regression: CIFAR-10 ResNet takes 682.28 ms in generated Rust versus 620.78 ms for stock scalar muRISCV-NN. Rust takes 9.91% more time, with a speedup ratio of 0.9099×. The QEMU instruction advantage did not predict this hardware ranking. Both models pass bit-exact output checks on every tested intermediate tensor.

| Model | Implementation | Median boot mean inference time | Range across three boots |
|---|---|---:|---:|
| Person detector | ESP-NN | 754.938 ms | 754.937209–754.939250 ms |
| Person detector | muRISCV-NN scalar | 377.111 ms | 377.108862–377.111722 ms |
| Person detector | muRISCV-NN corrected | 376.133 ms | 376.132581–376.136606 ms |
| Person detector | Generated Rust | 259.229 ms | 259.227747–259.229975 ms |
| CIFAR-10 ResNet | ESP-NN | 957.170 ms | 957.167806–957.170572 ms |
| CIFAR-10 ResNet | muRISCV-NN scalar | 620.776 ms | 620.774997–620.776184 ms |
| CIFAR-10 ResNet | muRISCV-NN corrected | 625.909 ms | 625.908294–625.912009 ms |
| CIFAR-10 ResNet | Generated Rust | 682.280 ms | 682.278506–682.281287 ms |

The equal-weight geometric mean across both models is **1.1490×**, below the original 1.2× target. Neither the losing model nor unfavorable C comparison is excluded.

## Repeatability evidence

The three captures have distinct host UUIDs, challenges, device identities, measurement schedules, requested phases and timing-sequence hashes. They observe 159,158,158 distinct tick phases respectively. Device/host elapsed-clock ratios are 1.0000362,1.0001059,0.9998492 (within 0.02%). The auditor also checks that the CPU counter agrees with the independent device microsecond timer and that randomized calibration delays match requested time. Earlier duplicate-sequence uncertainty is resolved for these v3 captures by the richer request/response evidence.

| Model | Boot 1 C/Rust | Boot 2 C/Rust | Boot 3 C/Rust |
|---|---:|---:|---:|
| Person detector | 1.450968× | 1.450989× | 1.450961× |
| CIFAR-10 ResNet | 0.909854× | 0.909857× | 0.909854× |

The local timeline records one post-flash reset and two esptool RTS hard resets, with USB power unchanged. There was no physical power-cycle capture. These are fresh reset sessions on one board, not a cold-power or cross-board study. The small observed ranges are reported without invented confidence intervals. Protocol consistency is not cryptographic hardware attestation.

## Implication for optimization

The cache-aware pointwise schedule is retained as a hardware-validated person-detector selection. It does not establish a general Rust runtime win. The new model has predominantly 3×3 convolutions with residual additions and a different weight/cache working set. Its current schedule was selected by QEMU guest instructions. That is a plausible source of the hardware ranking mismatch, but there is no per-layer hardware timing in this v3 capture to attribute the loss to a specific layer or transformation. V3_CHECK records are correctness checks, not timed layer profiles. The next optimization should use host-triggered per-layer timing and compare spatial tiling, im2col cost and weight reuse while retaining the current correct sources and all negative results.

## Scope and reproducibility

ESP32-C3 SuperMini rev0.4, 4 MB XMC flash, CPU160 MHz, flash DIO40 MHz, USB-powered, RTOS interrupts enabled. Four fixed fixtures per model; warm inference; no sensors or accuracy evaluation. The model outputs include the reference classifier mistakes documented in the model manifests. Backends are specifically built scalar ESP-NN, scalar muRISCV-NN, its separately documented corrected optional requantization configuration, and generated no_std Rust learned kernels. Common C graph dispatch/padding/pooling/reshape/softmax/residual-add code is shared. No comparison to CubeCL, MicroFlow, RVV/DSP acceleration or all available competitor configurations is implied.

This capture does not measure standalone ESP-IDF flash or isolated stack per backend; the image contains multiple implementations and reference tensors. The stack watermarks are cumulative. Earlier QEMU-only deployment size numbers remain separately labeled and must not be presented as board deployment size.

Reproduce: run hardware/esp32c3-v3/audit.py on the three original capture prefixes with hardware/esp32c3-v3/main/generated/oracle.json, without --emulator, then `python3 scripts/report_hardware_v3.py`. Original returned files are preserved in ../physical-return/. The local audit summary and file hashes accompany this report.
