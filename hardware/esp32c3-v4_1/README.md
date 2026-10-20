# Recovery from the physical V4 timeout

The V4 physical captures passed 4,512 fixed-backend tensor checks but all three
sessions stalled on the first ResNet whole-weight staging profile (backend 6).
No frozen selection or held-out timing was produced. The underlying hardware
stall is not diagnosed. V4.1 quarantines that graph-level candidate and replaces
it with the previously QEMU/host-tested per-block SRAM staging kernel. No whole
weight tensor is copied into the shared padding workspace in this recovery image.
The other tile kernels and validated person kernels are preserved.

V4.1 also checks the device build against this bundle before benchmarking and
reports accepted/warmed progress outside profile timing to locate any further
stall. Progress printing may change profile cache state; final whole-model runs
still use the unchanged warmup-and-measure protocol without progress printing.
A completed physical recovery run is still required; QEMU cannot prove the
physical stall is resolved. Stop and return evidence on the FIRST new failure.

# ESP32-C3 hardware-directed convolution tuning (V4.1)

This is a **diagnostic/tuning firmware**, not a production inference image. It
contains two unchanged pretrained models, golden intermediate tensors, three C
baselines, the physically measured incumbent Rust kernels, four new cache-tiled
Rust schedules, and two configurable composed backends. The person Rust kernels
and parameters are unchanged from V3. New hardware performance is not yet known.

Flash `esp32c3-generalize-v4_1.bin` at **0x0** on an ESP32-C3 with **at least 4 MB**
flash. It replaces the existing diagnostic. CPU 160 MHz, DIO flash 40 MHz. Use
`AGENT_PROMPT.md` for the full capture procedure. No Wi-Fi or external inputs are
needed. Python requirements are in `requirements.txt` in the release bundle.

## What the firmware does

1. Waits for a fresh host challenge. It never prints pre-recorded timings.
2. Checks all intermediate tensors of eight fixed backends on four inputs.
3. Measures three delay calibrations and 96 host-triggered, randomized profiling
   inferences: two models × eight backends × two **training inputs** × three repeats.
   Per-layer timing includes graph padding, kernel work, and dispatch; logging and
   correctness comparison happen outside those intervals. Profiling has additional
   timer overhead and is only a selection signal, not final whole-model latency.
4. Host `selection.py` chooses each learned layer. An alternative must improve both
   training-input median costs by at least 2%. One map is Rust-only; the other may
   use any tested C or Rust kernel. Shared graph operators remain common C. The
   person Rust map remains its previously validated implementation. This is a
   bounded empirical search, not a claim that one schedule fits every convolution.
5. The host freezes both maps and sends them to the device. All intermediate
   tensors of both composed graphs are checked on **all four inputs**.
6. Measures 400 complete-model inferences: two models × ten backends × four inputs
   × five repeats. Inputs 2 and 3 were excluded from selection. Warmups, randomized
   waits, output verification and serial logging are outside the timed interval.
7. On reset sessions two and three, `--selection` reuses the exact first-session
   maps. The audit requires their provenance from an included training capture.

The 2% margin is a deterministic guard, not statistical significance. Layer wins
are never treated as whole-model wins: the audit reports held-out full-model
latency against both the incumbent Rust and fastest tested C. No export is
silently promoted. Three resets on one board do not prove cross-board performance,
accuracy, power-cycle repeatability, or generality to all networks.

## Memory and code trade-offs

The new kernel generator creates signed-byte patch tiles (8 or 16 positions),
output-channel blocks (2 or 4), and register microtiles (4 or 2 positions).
Padding uses the original zero point, so offline bias folding is exact at borders.
One candidate copies only the current packed output-channel weight block to
local stack SRAM within the Rust kernel. Its copy cost is timed. Tensor storage
remains **112,144 bytes**. New explicit scratch reaches **10,368 stack bytes**;
the diagnostic task reserves **16,384 bytes**, unchanged from V3. Additional
profile and selection arrays also consume RAM. Stack low-water marks are recorded
on every final run. These costs must be considered before production deployment.
The image holds multiple duplicate packed parameter sets and all correctness
fixtures, so its size is not the size of an exported model.

Selection currently targets latency with a fixed bounded candidate pool; it does
not solve a global flash-constrained layer-mixture optimization. The frozen Rust
selection can be regenerated and independently measured using the repository's
`scripts/export_hardware_v4.py`. That export reports actual linked text and static
RAM and retains the old export; it is not automatic promotion.

## Captures

Each capture produces `.raw.log`, `.events.jsonl`, `.metadata.json`, `.plan.json`,
and `.selection.json`. Preserve all five, including failed captures. The auditor
checks file hashes, exact ordered commands, training-only selection, all 1,880
intermediate checks, final output fingerprints, timestamps, requested random
phases, cycle/wall-clock agreement, and independent reset identities.

```sh
python collect.py --port COM6 --output captures/boot-1
# Reset the device before each subsequent capture.
python collect.py --port COM6 --output captures/boot-2 --selection captures/boot-1.selection.json
python collect.py --port COM6 --output captures/boot-3 --selection captures/boot-1.selection.json
python audit.py captures/boot-1 captures/boot-2 captures/boot-3 --output audit-results
```

Expect roughly 10–20 minutes per capture depending on the serial connection.
`--timeout` defaults to 3,600 seconds. Use a new output prefix after a failed
attempt; never edit raw files or restart the same session without resetting.
`--emulator` on the auditor is solely for QEMU functional checks and prevents all
hardware-repeatability claims.
