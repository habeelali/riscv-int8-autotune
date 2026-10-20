Flash and run the supplied two-model ESP32-C3 v3 diagnostic. The user authorizes flashing this board. Use the exact provided binary, collect DIRECT serial observations, and return the original capture files unmodified. Do not rebuild firmware or replace missing data with generated/reconstructed records.

1. Install esptool==4.12.0 and pyserial==3.5. Verify firmware.sha256. Record OS/tool versions, COM/serial port, board, power and USB arrangement. Confirm ESP32-C3 and at least 4 MB physical flash with esptool; preserve identification output. Do not change eFuses/security.
2. Flash at 0x0, preserving all output:

   python -m esptool --chip esp32c3 --port PORT --baud 460800 --before default_reset --after hard_reset write_flash --flash_mode dio --flash_freq 40m --flash_size 4MB 0x0 esp32c3-two-models-v3.bin

3. Run the provided collector; passive serial monitoring is NOT sufficient because the PC must trigger measurements:

   python collect.py --port PORT --output captures/boot-1 --timeout 2400

   The firmware waits for a challenge. The collector chooses a fresh random token, sends CHECK, three CAL commands with different requested delays, and 160 RUN commands in randomized model/backend/input order with randomized timing phases. It logs host UTC and monotonic timestamps for each sent command and received serial line. It must reach V3_DONE. Allow up to 40 minutes if needed; typical runs should be much shorter. Do not use esptool or another serial monitor while collecting.
4. For boot 2 and boot 3, reset the board BEFORE starting each new collector. For example:

   python -m esptool --chip esp32c3 --port PORT --after hard_reset chip_id
   python collect.py --port PORT --output captures/boot-2 --timeout 2400

   Repeat using boot-3. A manual RESET or USB power cycle is also suitable; record what was physically done. Include at least one power-cycle capture if practical, keeping clocks/power arrangement otherwise unchanged. The collector can attach after startup because HELLO repeats READY. If it reports begin_requires_fresh_boot or waits for READY, reset the board and retry with a NEW output prefix. It refuses to overwrite files. Keep all failures and retry logs. Do not overlap captures or hold COM6 open in another process.
5. Each complete capture contains 752 V3_CHECK records (all tensors, two models), three V3_CAL records and 160 V3_RUN records, then V3_DONE with zero mismatches. Each output prefix produces .raw.log, .events.jsonl, .metadata.json and .plan.json; keep ALL FOUR original files. Do not edit metadata/timestamps, copy old logs into a new boot, or synthesize variation. Distinct tokens/nonces alone do not prove independent timing.
6. Audit successful captures together:

   python audit.py captures/boot-1 captures/boot-2 captures/boot-3 --output audit-results

   The default oracle.json is included in the bundle. Do NOT use --emulator for physical data. If audit fails, preserve its output and all original files and explain the actual failure. Never adjust the data to make the audit pass. The audit checks request/response sequences, golden tensor hashes, phase variation, delay calibration, cycle/wall consistency, and host/device elapsed-clock agreement.
7. Return hardware-results-v3.zip containing all raw/event/metadata/plan files, audit output, firmware checksum and flashing/identification transcripts, plus board.txt and a short factual reset/capture timeline. Include failed attempts. Do not return only averages or screenshots.

Backends (same IDs for both models): 0 ESP-NN scalar optimized C, 1 muRISCV-NN scalar, 2 muRISCV-NN scalar with the documented optional rounding correction, 3 generated scalar Rust. Models: 0 pretrained INT8 person detector, 1 MLPerf Tiny pretrained INT8 CIFAR-10 ResNet. No sensors, camera, Wi-Fi or other wiring is required. No hardware speedup for the second model is presumed.
