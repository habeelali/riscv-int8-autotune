# Second-model execution: pretrained CIFAR-10 ResNet

Pinned unmodified MLPerf Tiny model: 3c002613d1b2475eb51dd78dfb85a546c8ae658dee71cf6ade43b022fe205415. Four real test-image JPEG fixtures are decoded and quantized as pixel−128, following the upstream input scale. Preview compression may change original CIFAR pixels. LiteRT BUILTIN_REF, no delegates, preserves all intermediate tensors.

C was executed and measured before generic Rust; generic Rust was executed before schedule generation. All three C configurations, generic Rust, and all three generated schedules match every intermediate tensor on all four images, in two repeated RV32IMC QEMU runs each. No candidate was dropped for losing performance.

| Implementation | Mean raw guest instructions | .text bytes | Static RAM bytes |
|---|---:|---:|---:|
| c | 91,172,961.25 | 20,716 | 112,160 |
| muriscv | 54,717,397.75 | 28,430 | 112,160 |
| muriscv-fixed-asm | 54,045,386.75 | 28,706 | 112,160 |
| generic | 413,071,028.50 | 32,674 | 112,160 |
| generated-0 | 45,374,096.50 | 41,530 | 112,160 |
| generated-1 | 45,566,162.00 | 55,058 | 112,160 |
| generated-2 | 50,786,075.25 | 36,040 | 112,160 |

Raw guest instructions isolate the complete graph, including common residual add, padding, pooling, reshape and softmax, plus counter-region overhead. They are not cycles. The selected generated schedule is 1.191× versus the corrected scalar muRISCV-NN configuration, below 1.2×. Hardware timing remains unmeasured.

The model contains nine CONV_2D, three ADD, one AVERAGE_POOL_2D, one RESHAPE, one FULLY_CONNECTED and one SOFTMAX. A first-fit liveness allocator reserves separate live residual tensors and reaches 49,152 activation bytes; the two-model firmware reuses the existing 73,728 activation bytes and 38,416 padding bytes. Target kernels use stack im2col buffers up to 2,304 bytes in the selected schedule. The task has 16,384 bytes stack; actual high-water values are collected by firmware.

All source model, fixture hashes and labels are in models/cifar10-resnet/source.json. The reference predicts the first three fixture labels correctly and misses the fourth; this is not an accuracy estimate. Original weights, graph, quantization, and model bytes are unchanged. Schedule options are retained in generated-*/manifest.json, constants in parameters.h and reusable generated sources in source.rs.

Commands: `python3 scripts/resnet_model.py`, `python3 scripts/benchmark_resnet.py`, `python3 scripts/benchmark_resnet.py --generic`, `python3 scripts/tune_resnet.py`.
