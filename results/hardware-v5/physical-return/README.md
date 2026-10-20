# ESP32-C3 V5 physical hardware results

Procedure completed: True

Image flashed once, unchanged, at 0x0. ESP32-C3 and physical 4 MB flash were identified before flashing. Esptool's data hash verification succeeded. Python runtime was reused from an existing environment; only this bundle's V5 collector, auditor and image were used.

No --emulator argument was used. Native USB Serial/JTAG connection; board model and power assumptions are in board-details.json. Reset times and methods are recorded in reset-timeline.json. These resets are not physical power cycles.

## Captures
- boot-1: completed=True; CHECK=752, CAL=3, RUN=160, DONE=1; error=None
- boot-2: completed=True; CHECK=752, CAL=3, RUN=160, DONE=1; error=None
- boot-3: completed=True; CHECK=752, CAL=3, RUN=160, DONE=1; error=None

Hardware audit: valid=3; failed=0; independent_repeatability_evidence=True; emulation_only=False.

## Files

captures/ contains every original raw log, event log, metadata and plan. Step console logs include complete identification, flashing, reset, collector and auditor output, with exit codes in procedure-status.json. capture-summary.json retains each capture's last sent command. bundle-reference/ contains the supplied scripts, oracle, manifest, checksums, provenance and other original non-binary references. Producer-provided emulator records there are reference material, not board results. The original ELF and image are omitted as requested; their original hashes are retained.

The supervisor console includes its initial archive hash. This final archive was rebuilt offline to include that completed console, so the authoritative final archive hash is hardware-results-v5.zip.sha256. inventory.sha256.json hashes every other file in this archive.
