Flash and benchmark the fixed V5 firmware on my local ESP32-C3. This is authorized.
Use this bundle unchanged. Do not use the old V3/V4/V4.1 collectors or image.

1. Verify esp32c3-final-benchmark-v5.bin SHA-256 against manifest.json and
   firmware.sha256. Confirm ESP32-C3 and physical flash >=4 MB. Install bundled
   requirements.txt in a Python environment if needed. Save chip/flash identity,
   Python/esptool/pyserial versions, verification output and all console logs.
2. Flash once at address 0x0, replacing the previous diagnostic:
   python -m esptool --chip esp32c3 --port PORT --baud 460800 --after hard_reset write_flash --flash_mode dio --flash_freq 40m --flash_size 4MB 0x0 esp32c3-final-benchmark-v5.bin
   Save complete flash output. Do not erase_flash or program any other device.
3. Find the re-enumerated native USB serial port (previously COM6). Close serial
   monitors and run:
   python collect.py --port PORT --output captures/boot-1 --timeout 2400
   The collector checks the V5 build before benchmarking. Expect 752 checks,
   three calibrations and 160 timed inference RUNs. No profiling or tuning phase.
4. After completion, perform an actual reset (reset button or esptool chip_id with
   --after hard_reset). Record UTC time/method; keep USB cable, power and settings.
   python collect.py --port PORT --output captures/boot-2 --timeout 2400
5. Reset again, record it, and run:
   python collect.py --port PORT --output captures/boot-3 --timeout 2400
   There is NO --selection file or argument for V5; all kernels are already fixed.
6. Audit in hardware mode, saving output and exit code:
   python audit.py captures/boot-1 captures/boot-2 captures/boot-3 --output audit-results
   Never pass --emulator. Never edit raw files or discard slow/failed observations.
7. Return hardware-results-v5.zip with all captures' four sidecars and console
   logs, audit-results, manifest.json, oracle.json, provenance.json,
   firmware.sha256, flash/checksum/identity output, script copies for provenance,
   dependency versions, and UTC reset timeline. The producer already has the ELF
   and binary; no need to send them back.

If a firmware/protocol mismatch or any timeout occurs, STOP after the first failure
and return all evidence; do not repeat the same failure several times. Preserve
last sent command and complete raw bytes. A reset is not a physical power cycle.
This image removes both staging variants and the PROFILE/SELECT paths implicated
in prior failures; the actual cause remains unknown. No prediction is a result.
