# V5: repeatable physical complete-model inference gains

The local V5 auditor independently accepted all three returned raw captures against the local build oracle, without emulator exemptions. All 2,256 intermediate tensor checks and 480 timed complete-model inferences passed, with zero mismatches. Exact command plans, fresh challenges, raw/event hashes, clock calibration and host/device elapsed-time agreement passed. Returned scripts were not executed.

| Model | Fastest tested C | Fixed Rust | C/Rust | Rust range across resets |
|---|---:|---:|---:|---:|
| Person detector | 376.315 ms | 259.228 ms | 1.4517× | 259.227519–259.229681 ms |
| CIFAR-10 ResNet | 620.384 ms | 354.690 ms | 1.7491× | 354.689475–354.691372 ms |

Equal-model geometric mean: **1.5935×** against the fastest tested C baseline per input. Each boot uses four input medians (five timed observations per input/backend); times above average the input medians equally across the three boots. Ratios geometrically average per-input ratios, then boots and models. This is physical elapsed inference timing at 160 MHz, supported by cycle/wall-clock checks, not QEMU instruction counts. No confidence interval is inferred from three boots.

Fastest C is corrected muRISCV-NN for person detection and stock scalar muRISCV-NN for ResNet. ESP-NN is also measured in the same image; it is slower than those controls.

The largest Rust boot-to-boot range of the averaged input medians is 0.00083%. This describes these observations only, not individual-run jitter or a population uncertainty estimate.

## Scope and unresolved failure

All three sessions used the same firmware, native USB and one ESP32-C3 SuperMini, with documented esptool hard resets. They were not physical power cycles. Schedules were fixed before capture; V5 has no hardware search or staging backend. These are previously used validation fixtures, not a new independent accuracy set.

This is the first independently repeated two-model physical win in this project. It supersedes V3’s ResNet regression for this fixed V5 implementation. It does not promote the faster, staging-dependent V4.1 mixed selection, which failed on a later reset.

The intermittent V5 simulation stalls are still real unresolved failures. Successful board captures show this image completed these three hardware sessions; they neither identify a root cause nor justify declaring QEMU responsible. Earlier V4/V4.1 failures and all incomplete simulation attempts remain preserved. Further debugging must capture a genuinely stalled PC/stack before attributing the fault.

The diagnostic firmware includes all C/Rust backends, fixtures and golden tensors. Its 4 MiB merged flash file includes partition padding; neither that size nor whole-ELF size is an isolated kernel flash measurement. No new flash-saving claim is made.

[Timing graph](model-comparison.svg) · [per-boot comparison](model-comparison.csv) · [machine-readable results](comparison.json) · [strict raw audit](summary.json)
