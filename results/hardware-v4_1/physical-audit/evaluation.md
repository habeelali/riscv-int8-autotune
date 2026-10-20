# V4.1 physical results: held-out gains in one session, repeatability unresolved

Local code independently audited the returned raw captures against the local oracle. Boot 1 is valid: 1,880 intermediate tensor comparisons, 96 training profiles, 400 timed whole-model RUNs, three delay calibrations, zero mismatches, and consistent host/device clocks. Boot 2 is incomplete and explicitly rejected. Uploaded executable/bytecode files were not executed.

## Held-out complete-model timing from boot 1

Layer selection used only inputs 0/1; inputs 2/3 below were held out. Each value is the mean of the two input medians, each median from five RUN observations. Warmup, validation, printing and random waits are outside the timed interval. Ratios are geometric means of the two per-input ratios.

| Model | Fastest tested C | Selected Rust | Held-out C/Rust |
|---|---:|---:|---:|
| Person detector | 376.024 ms | 258.810 ms | 1.4529× |
| CIFAR-10 ResNet | 621.289 ms | 343.765 ms | 1.8073× |

The equal-model geometric mean is **1.6204× in this one completed session**. This clears 1.2× for that session; it does not satisfy the independent-reset repeatability requirement. No confidence interval or reliable cross-reset speedup is claimed.

The selected Rust-only and hybrid maps are identical in this capture. All selected learned kernels remain Rust; common C graph operators still run in both. The fixed tile-16 Rust backend also beats C on held-out ResNet inputs (1.7614×), so the successful model result is not solely a post-selection layer-profile inference. This is the first completed physical held-out ResNet win, rather than just promising training profiles.

## Failure on the next reset

Boot 2 reused the frozen first-boot selection, then stopped during the profiling phase at seq 12, ResNet backend 6, sample 1. Last progress is `warmed`, with cumulative minimum free stack 4,652 bytes. Boot 3 was not attempted, following the stop-after-first-failure instruction. Both sessions were USB-powered; the second used an RTS hard reset, not a physical power cycle.

Backend 6 is now the per-block SRAM staging kernel, not V4 whole-weight staging. Thus removing whole-weight staging did NOT eliminate the hardware failure. A hypothesis specific to writing whole weights into the shared padding buffer is insufficient to explain all observed failures. Successful warmup does not prove that the subsequent timed kernel completed. The last marker precedes a task delay, phase delay, timer reads, timed execution and result printing; the logs cannot distinguish a stall in those later stages or the transport. The stack watermark is not a proof excluding an interrupt or transient stack problem. Do not declare a root cause from the backend number alone.

## Interpretation and next review

Keep the numerical improvements as single-session evidence and preserve the failure. Do not promote the selected model as independently repeatable, remove backend 6 without documenting it, or silently infer that fixed-tile and selected maps share the same reliability. Wait for the ongoing independent research review before another speculative firmware iteration. Supply the updated evidence addendum so the review covers both the real held-out gains and the repeated block-staging stall.

The previous Rust ResNet backend measures about 653.6 ms averaged over all four inputs in this image, versus 682.3 ms in V3. Code layout/instrumentation differs between images; causal performance comparisons use the controls in the same V4.1 image. Stock tested C weights remain in flash, so improvements do not isolate the Rust language. Diagnostic size and cumulative stack watermarks do not establish compact deployment flash or isolated per-backend peak stack.

[Held-out timing graph](held-out-comparison.png), [all backend timings](model-comparison.csv), [machine-readable interpretation](comparison.json), and [strict audit](summary.json).
