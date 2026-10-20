# Hardware-guided cache candidates (v2)

The previous physical-board result is a correctness pass and a performance miss: 73.11M candidate cycles versus 58.51M for the fastest tested muRISCV-NN configuration. The previous Rust implementation, sources, parameters and saved selection remain unchanged.

## Diagnosis and change

Layer profiles in the first complete returned capture show candidate/C ratios of 1.92–2.19 on repeated 6×6×128→128 pointwise layers and 1.56–1.59 on the last wider layers. Most smaller learned layers favor Rust. Layer profiling includes serial/delay gaps, so those ratios diagnose rather than sum to full-model timings.

The previous pointwise body traverses pixel tiles before output-channel blocks. The new body traverses channel blocks first and reuses them over all pixel tiles. Only layers 14,16,18,20,22,24,26 are changed; both variants use scalar RV32IMC loads and multiplies. Parameters are offline-repacked into the existing blocked layout, with unchanged parameter byte counts. The channel-first working set is block×input_channels (256–1,024 bytes here), rather than sweeping 16–64 KB of weights for every pixel tile. This is consistent with a flash-cache bottleneck; cache causation and hardware speedup are not established without board measurements.

Assembly evidence: `cache4-layer14-assembly.txt` and `cache2-layer14-assembly.txt`. The cache4 inner body loads four adjacent weight bytes, multiplies against two input pixels, and branches over reduction. The outer channel block surrounds the spatial traversal. No handwritten assembly or vector extension is introduced.

## Executed validation

Full model on QEMU virt: four fixture inputs, all intermediate tensors match the independent LiteRT oracle. Standalone production-style images: a fifth all-zero image matches LiteRT; three repeated executions per implementation agree. The actual merged ESP32-C3 v2 image also boots in Espressif QEMU: all 1,116 layer checks and 252 timed observations pass. Its emulator timings are deliberately not reported as board performance.

| Implementation | QEMU mean guest instructions | Benchmark .text | Standalone binary bytes | Standalone static RAM |
|---|---:|---:|---:|---:|
| mixture-fast-validated | 29,599,020.75 | 44,790 | 282,452 | 121,360 |
| hardware-cache-b4-p2 | 29,580,455.75 | 44,906 | 282,664 | 121,360 |
| hardware-cache-b2-p4 | 29,645,169.50 | 44,172 | 281,880 | 121,360 |

QEMU-measured maximum stack remains 708 bytes. Tensor RAM remains 112,144 bytes; standalone static RAM 121,360 includes an extra 9,216-byte test input. The cache4 variant slightly increases code; cache2 reduces code but slightly increases guest instructions. Neither is silently substituted for the previous selected kernel. Selection now requires physical cycle data.

Rust formatting, clippy with warnings denied, and all workspace tests pass. The updated log parser was exercised on the old hardware capture and the complete new ESP32-C3 emulator capture.

## New diagnostic

Nine backends retain all seven earlier configurations and add rust_cache4 and rust_cache2. All use the same new graph driver and buffers. Backend tables now reside in read-only flash. The combined image is approximately 2.3 MB of application payload and requires 4 MB flash because it contains all backends, packed-parameter sets and full tensor oracles; it is not a single-kernel deployment size. The previous user board was identified as 4 MB.

The original returned captures were nearly identical. V2 adds HW_CAPTURE boot counter and fresh RNG values, extends the startup capture window to eight seconds, and records host UUID/time/hash sidecars. A physical-board result for v2 is still pending. Build IDs identify bundled kernel/parameter provenance; the complete merged image SHA-256 identifies exact firmware bytes.

Reproduce candidates: `python3 scripts/tune_hardware_cache.py`. Build: `python3 hardware/esp32c3/prepare.py --cache-candidates`, then ESP-IDF v5.4.2 `idf.py -C hardware/esp32c3 build`. Package with `hardware/esp32c3/package.py` only after emulator verification of the exact image. See the included AGENT_PROMPT.md for the physical test.
