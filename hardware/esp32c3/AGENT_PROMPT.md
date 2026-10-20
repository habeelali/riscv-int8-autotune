Flash the supplied v2 diagnostic onto the user's ESP32-C3 SuperMini and return original measurements. The user authorizes flashing. Use the supplied binary unchanged, without rebuilding, modifying clocks, or inventing/substituting measurements.

1. Verify the SHA-256 in firmware.sha256. Install esptool==4.12.0 and pyserial==3.5. Identify the chip and detected physical flash; save the transcripts. This image requires ESP32-C3 with at least 4 MB physical flash. Do not alter eFuses/security settings.
2. Flash the merged image at offset 0x0, saving all output:

   python -m esptool --chip esp32c3 --port PORT --baud 460800 --before default_reset --after hard_reset write_flash --flash_mode dio --flash_freq 40m --flash_size 4MB 0x0 esp32c3-int8-cache-v2.bin

3. Capture directly from the serial port, using the included script and a NEW output path:

   python capture.py --port PORT --output boot-1.log --timeout 1200
   python summarize.py boot-1.log --output boot-1-results

   There is an eight-second startup delay. If native USB reset/reconnection fails, first start capture.py with --no-reset, then manually press RESET. Do not capture after inference has started. Wait for HW_DONE. Expect HW_CAPTURE, HW_BEGIN, HW_INFO, exactly 1116 HW_LAYER records, and 252 HW_RUN records. All mismatches should be zero. A failed/incomplete log must still be returned.
4. Repeat twice with fresh resets and new filenames. Keep all .capture.json sidecars. Compare HW_CAPTURE nonce_hi/nonce_lo and boot_counter between logs. Each capture must originate from a fresh board run. If identities repeat or logs have identical hashes, preserve them, report the issue, and make an additional capture after power cycling. Never copy an old log into a new capture filename.
5. Return hardware-results-v2.zip with ALL raw logs, capture metadata, parser JSON/CSV, identification and flashing output, and board.txt describing chip, physical flash, USB/power, port, host OS and tool versions. Include a brief timeline of actual resets and any serial reconnect/failure. Avoid screenshots or aggregate numbers in place of raw data.

Nine backends include all seven previous configurations plus rust_cache4 and rust_cache2. Compare their per-input median whole-model cycles against rust_candidate and the fastest C. Layer counts are diagnostic; whole-model timing is authoritative. Do not claim a win just because the firmware passes correctness. No sensor/camera/Wi-Fi/external wiring is needed.
