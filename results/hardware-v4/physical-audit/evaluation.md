# Physical V4 return: incomplete protocol, candidate quarantined

Local audit: **0 valid completed captures, 3 incomplete captures**. The image/build hashes match the supplied V4 firmware. Each capture passed all 1,504 fixed-backend intermediate tensor comparisons: 4,512 checks total, zero mismatches. Sixty-two completed training profiles also passed local raw/event hashes, exact host request/challenge binding, output fingerprint, and cycle/wall/host interval checks.

All three sessions stop on the first ResNet backend 6 PROFILE request. That backend stages whole-layer weights in the shared padding workspace. It passed per-layer correctness checks with inter-layer logging/yields, but never returned a complete profiled graph on the physical board. No crash text or per-layer progress was captured, so the underlying cause cannot be identified from these logs. QEMU did not reproduce the stall.

The two earlier attempted captures show old V3 firmware and are preserved separately. The three captures with the correct V4 build were the ones audited. No capture completed training; no selection file, held-out model timing, or independent timing-repeatability result exists.

## Descriptive training observations only

Uneven counts across only inputs 0/1; these are instrumented PROFILE intervals, not the final RUN protocol. Do not calculate a publishable speedup from this table.

| ResNet backend | Completed observations | Median profile ms | Observed range ms |
|---|---:|---:|---:|
| ESP-NN C | 7 | 950.251 | 950.244–950.277 |
| mu scalar C | 2 | 628.001 | 627.932–628.070 |
| mu corrected C | 2 | 619.459 | 619.451–619.467 |
| Previous Rust | 2 | 673.679 | 673.676–673.682 |
| Tile 8 Rust | 3 | 371.167 | 371.166–371.175 |
| Tile 16 Rust | 3 | 352.688 | 352.687–352.693 |
| Whole-weight SRAM (stalled) | 0 | — | — |
| Tile 16 / block 4 Rust | 7 | 382.281 | 382.280–382.287 |

[Observed training dots](partial-training-observations.png). Every dot is an actual completed returned profile; vertical offsets separate overlapping observations and are not timing variance.

## Recovery V4.1

The unstable whole-weight staging graph path is quarantined rather than labeled fixed. Backend 6 now uses the separately retained, previously tested per-block SRAM staging kernel. The other tile and person kernel sources are preserved. No whole-weight copy into the shared padding workspace remains in the recovery graph. Added PROFILE accepted/warmed progress records are outside the timed region and identify whether a future stall occurs before or after warmup. The collector checks the firmware build against its local oracle and rejects an old protocol immediately.

The recovery remains a hardware-unverified workaround until a physical capture completes. The local agent should stop on the first new failure, preserving all raw files and last progress metadata, rather than spend time on repeated timeouts. If training completes, freeze that first selection for two further resets and evaluate held-out full-model RUN intervals.

Previous V3 validated measurements remain the published hardware results: person detector wins; ResNet loses. V4 partial profiles are promising evidence for the cache-tiled family, not a replacement for held-out validation.
