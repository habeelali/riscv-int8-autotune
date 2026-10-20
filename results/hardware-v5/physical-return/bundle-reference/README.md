# V5 fixed-kernel final benchmark

Flash esp32c3-final-benchmark-v5.bin at **0x0** on ESP32-C3 with at least 4 MB flash.
CPU 160 MHz, DIO flash 40 MHz. Use only the image, oracle and scripts from this
bundle. It contains four backends: ESP-NN, stock scalar muRISCV-NN, the documented
corrected muRISCV-NN, and fixed no_std Rust learned kernels with common C graph ops.

The person Rust kernel is the preserved cache-b2-p4 export. ResNet uses the fixed
**tile16_b2p4** implementation, which completed physical held-out RUNs in V4.1
(about 352.7 ms in that image, 1.761× fastest tested C). It does NOT use the mixed
343.8 ms selection, because that selection includes the staging experiment.
Those previous figures are historical, not promised V5 performance: image layout
and cache interactions can change timings.

Both whole-weight and block-weight staging candidates are absent. There is no
PROFILE, SELECT, hardware tuning, or staged backend in this binary. Schedules are
fixed at build time and identical across every reset. This bypasses the failing
experimental path; it does not establish the root cause of the earlier stalls or
prove that a future board run cannot fail. The physical run measures repeatability
of this narrower fixed-kernel image. Prior failures remain documented.

Each host capture checks **752 intermediate tensors**, calibrates three delays,
then collects **160 complete inference RUNs** (2 models × 4 backends × 4 input
fixtures × 5 repeats). The collector randomizes backend/input order, phases and
waits and saves host timestamps. Each RUN has untimed warmup; startup, printing,
validation and waits are outside its counter interval. Reference outputs are
checked on every RUN. No IRQ masking, CPU/flash overclock, or watchdog changes
have been added. The task stack remains 16 KiB; tile scratch is at most 9,216 B.
Golden fixture data and multiple backends make this a diagnostic benchmark image,
not a minimal production flash-size artifact.

There is no selection file and no --selection option. Each reset uses the same
compiled kernel. Fixtures 2/3 were originally held out during tuning; all four
are now repeated validation fixtures, not a new unseen accuracy dataset.

```sh
python collect.py --port COM6 --output captures/boot-1
# Physically reset / esptool hard-reset; re-open the correct port.
python collect.py --port COM6 --output captures/boot-2
# Reset again.
python collect.py --port COM6 --output captures/boot-3
python audit.py captures/boot-1 captures/boot-2 captures/boot-3 --output audit-results
```

Preserve all four sidecars per capture: .raw.log, .events.jsonl, .metadata.json and
.plan.json. No files are overwritten. Use a new prefix if a capture failed. Stop
and return evidence after the first unexpected timeout; do not add delays, remove
slow observations, edit firmware, or change measurement settings to get a pass.
Do not pass --emulator on hardware. One board / three resets is not a cross-board
or physical power-cycle claim. Physical timing of V5 remains pending until these
captures are returned. Follow AGENT_PROMPT.md for the exact local procedure.

## Validation limits

Two complete ESP32-C3 QEMU sessions passed all 752 tensor checks and 160 RUNs.
Four negative audit controls passed. Other emulator launches also stalled (one
at initial CHECK, one at RUN 158); the same timed-out RUN schedule later completed
with a monitor socket enabled. An icount-mode boot experiment did not produce boot
output and was terminated. These attempts are recorded in emulator-attempts.json.
They are not discarded or called passes. The underlying stall is not diagnosed.
This release is a benchmark candidate that removes staging/profiling paths, not
proof that all hangs are fixed. Physical V5 captures are needed before any final
repeatability or speedup claim. Stop and return evidence on the first failure.
