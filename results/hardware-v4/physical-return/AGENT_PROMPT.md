You are operating my local ESP32-C3 board. I authorize flashing this diagnostic
and collecting its output. Follow this procedure and return the evidence ZIP.
Do not change firmware, models, capture/audit code, or measurements.

1. Unzip the bundle into a fresh directory. Read manifest.json. Record Python,
   esptool and pyserial versions and the board/chip/flash identification. Install
   requirements.txt into a local Python environment if needed.
2. Verify the binary SHA-256 exactly matches manifest.json / firmware.sha256.
   Require ESP32-C3 with at least 4 MB flash. Stop on a different chip or mismatch.
3. Flash the merged image ONCE at address 0x0 using:
   python -m esptool --chip esp32c3 --port PORT --baud 460800 --after hard_reset write_flash --flash_mode dio --flash_freq 40m --flash_size 4MB 0x0 esp32c3-generalize-v4.bin
   Save the complete flash output. This intentionally replaces the prior test
   firmware. Do not use erase_flash or overwrite unrelated devices.
4. Find the re-enumerated serial port (previously COM6). Close serial monitors.
   After the post-flash reset, run:
   python collect.py --port PORT --output captures/boot-1 --timeout 3600
   This captures training profiles, freezes layer choices automatically, and runs
   the first full validation. It may take 10–20 minutes. Save console output too.
5. Perform a real hardware reset (esptool --after hard_reset chip_id worked in V3,
   or use the reset button). Record method and UTC time. Keep board, power supply,
   USB cable, frequency and ambient conditions unchanged. Run:
   python collect.py --port PORT --output captures/boot-2 --selection captures/boot-1.selection.json --timeout 3600
6. Reset again and run:
   python collect.py --port PORT --output captures/boot-3 --selection captures/boot-1.selection.json --timeout 3600
   Do NOT retune separately on boot 2 or 3. Do not pretend a reset is a power cycle.
7. Run exactly:
   python audit.py captures/boot-1 captures/boot-2 captures/boot-3 --output audit-results
   NEVER pass --emulator on physical hardware. Save stdout/stderr and exit code.
8. Package hardware-results-v4.zip containing every capture's five files
   (.raw.log, .events.jsonl, .metadata.json, .plan.json, .selection.json), console
   transcripts, audit-results, manifest.json, oracle.json, provenance.json,
   firmware.sha256, flash/checksum/board outputs, environment versions, and a UTC
   reset timeline. Include collector/auditor/selection script copies for provenance.
   The firmware ELF/binary need not be returned since the producer already has them.

If any check fails, preserve all failed outputs and report exactly what failed.
A retry needs a reset and a NEW capture prefix. Keep the first successful training
selection frozen for subsequent validation resets. Do not manufacture output,
filter slow observations, summarize away losses, edit logs, tune on held-out
inputs, or call an emulator result hardware. The producer needs raw per-layer
profiles plus all full-model observations, including regressions, to determine
whether the Rust-only and/or hybrid implementations actually generalize.
