# ESP32-C3 v2 returned-log evaluation

## Observed result

The returned serial logs show the new cache2 Rust implementation at 257.99 ms versus 367.25 ms for the fastest tested C configuration, scalar muRISCV-NN. Across the four fixture inputs, the geometric-mean speedup is 1.4235×, a 29.75% reduction in inference time. The previous Rust candidate in the same v2 image takes 442.84 ms; cache2 reduces its time by 41.74%. The 1.2× target is exceeded in these returned measurements, subject to the capture qualification below.

| Implementation | Mean sample-median cycles | Equivalent inference time |
|---|---:|---:|
| Rust cache2 (selected) | 41,278,014.0 | 257.99 ms |
| Rust cache4 | 42,780,598.0 | 267.38 ms |
| muRISCV-NN scalar | 58,760,021.0 | 367.25 ms |
| muRISCV-NN corrected | 58,846,308.0 | 367.79 ms |
| muRISCV-NN portable | 59,352,775.0 | 370.95 ms |
| muRISCV-NN portable corrected | 59,707,969.0 | 373.17 ms |
| Previous Rust candidate | 70,854,283.0 | 442.84 ms |
| Earlier compact Rust | 85,541,432.0 | 534.63 ms |
| ESP-NN baseline | 120,471,083.5 | 752.94 ms |

Cycles are from the ESP32-C3 CPU counter, converted at the logged 160 MHz frequency. Logged esp_timer wall times agree. Both new variants beat the fastest C on every fixture; the cache4 geometric-mean speedup is 1.3735×. Comparisons use the same v2 image and graph harness. The old candidate and C timings changed slightly from v1 due to image/graph/code placement, so v2-to-v2 is the headline comparison.

## Correctness and capture audit

Independently parsed raw logs: three complete captures, each with exactly 1,116 unique backend/input/layer keys and 252 unique backend/input/round keys; all reported mismatches are zero and the firmware/model identifiers match. The failed automatic-reset attempt is empty and excluded. Host sidecar log hashes match raw bytes; supplied image identification matches the locally built image. All successful logs have distinct HW_CAPTURE boot identifiers and host UUIDs.

However, all 252 whole-model cycle counts match exactly across all three captures; 1,113 of 1,116 layer counts match boot 1 in boot 2 retry. Each backend/input also has identical cycle counts in all seven rounds. This unusual determinism is retained explicitly: nonce uniqueness is not proof of independent timing evidence. We use boot 1 for the table and do not pool captures, report confidence intervals, or claim 21 independent observations. A direct capture with per-observation host timestamps and varied tick phase would strengthen the evidence. This audit does not establish why the repetition occurs.

## What changed

Only large pointwise layers 14,16,18,20,22,24,26 switch to channel-first packed traversal. Each small weight block is reused across spatial tiles before advancing to the next channel block. cache2 uses two output channels and four pixels per tile; cache4 uses four channels and two pixels. This reduces the active weight working set compared with repeatedly sweeping entire 16–64 KB matrices. Arithmetic, quantization, activation and expected intermediate tensors are unchanged. These are established loop/data-layout techniques; no novelty is claimed merely for implementing them in Rust. Cache behavior is a supported hypothesis, not a hardware-counter attribution study.

The learned kernels remain scalar no_std Rust with no heap during inference. Common graph dispatch, padding, pooling, reshape and softmax are C, shared by all backends. Thus this is a Rust-kernel result for a mixed runtime, not an entirely Rust neural-network runtime.

## Memory and scope

Previously executed standalone QEMU deployment builds measure cache2 at 281,880 binary bytes versus 282,452 for the prior candidate (572 bytes smaller), with unchanged 121,360 bytes static RAM including the held-out input and 708-byte measured maximum stack. These are QEMU-linked deployment sizes, not standalone ESP-IDF board-image sizes. The 4 MB combined diagnostic contains nine backends, duplicate parameter layouts and reference tensors and must not be used as a single-model size claim.

One ESP32-C3 SuperMini rev0.4, 4 MB flash, 160 MHz CPU, DIO40 MHz, USB power, RTOS interrupts enabled. One pretrained INT8 96×96 person detector and four fixtures, including two mirrored images; not dataset accuracy or cross-model evidence. ESP-NN and muRISCV-NN here are the specifically built scalar configurations. This is not evidence of beating CubeCL, MicroFlow, accelerator/vector implementations, or every ESP-NN target-specific backend.

Reproduce parsing, audits, tables and plots: `python3 scripts/analyze_hardware_v2.py`. Raw returned files remain in `../physical-return/`.
