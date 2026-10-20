"""Offline final packaging, including the completed supervisor console log."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import zipfile

ROOT = Path(__file__).resolve().parent
OUT = ROOT / "hardware-results"
TARGET = ROOT.parent / "hardware-results-v5.zip"


def sha(data):
    return hashlib.sha256(data).hexdigest()


state = json.loads((OUT / "procedure-status.json").read_text())
assert state.get("finished_utc") and (state.get("completed") or state.get("first_failure")), "Do not package an active procedure"
shutil.copy2(ROOT / "procedure-v5.console.log", OUT / "procedure-v5.console.log")
shutil.copy2(__file__, OUT / Path(__file__).name)
with zipfile.ZipFile(ROOT.parent / "esp32c3-final-benchmark-v5.zip") as original:
    matches = {name: sha(original.read(name)) == sha((ROOT / name).read_bytes()) for name in original.namelist() if not name.endswith("/")}
assert all(matches.values()), "Supplied bundle was modified"
(OUT / "bundle-preservation.json").write_text(json.dumps(dict(all_original_files_unchanged=True, files=matches), indent=2) + "\n")
captures = json.loads((OUT / "capture-summary.json").read_text())
lines = ["# ESP32-C3 V5 physical hardware results", "", "Procedure completed: " + str(state["completed"]), "", "Image flashed once, unchanged, at 0x0. ESP32-C3 and physical 4 MB flash were identified before flashing. Esptool's data hash verification succeeded. Python runtime was reused from an existing environment; only this bundle's V5 collector, auditor and image were used.", "", "No --emulator argument was used. Native USB Serial/JTAG connection; board model and power assumptions are in board-details.json. Reset times and methods are recorded in reset-timeline.json. These resets are not physical power cycles.", "", "## Captures"]
for c in captures:
    records = c["records"]
    lines.append(f"- {c['prefix']}: completed={c['completed']}; CHECK={records.get('V5_CHECK', 0)}, CAL={records.get('V5_CAL', 0)}, RUN={records.get('V5_RUN', 0)}, DONE={records.get('V5_DONE', 0)}; error={c.get('error')}")
if state.get("first_failure"):
    lines += ["", "Stopped after the first failure; no subsequent reset/capture/retry was attempted.", state["first_failure"]["error"]]
audit_path = OUT / "audit-results" / "summary.json"
if audit_path.exists():
    audit = json.loads(audit_path.read_text())
    lines += ["", f"Hardware audit: valid={audit['valid_captures']}; failed={audit['failed_captures']}; independent_repeatability_evidence={audit['independent_repeatability_evidence']}; emulation_only={audit['emulation_only']}."]
else:
    lines += ["", "Three-capture audit was not run because the procedure stopped early."]
lines += ["", "## Files", "", "captures/ contains every original raw log, event log, metadata and plan. Step console logs include complete identification, flashing, reset, collector and auditor output, with exit codes in procedure-status.json. capture-summary.json retains each capture's last sent command. bundle-reference/ contains the supplied scripts, oracle, manifest, checksums, provenance and other original non-binary references. Producer-provided emulator records there are reference material, not board results. The original ELF and image are omitted as requested; their original hashes are retained.", "", "The supervisor console includes its initial archive hash. This final archive was rebuilt offline to include that completed console, so the authoritative final archive hash is hardware-results-v5.zip.sha256. inventory.sha256.json hashes every other file in this archive."]
(OUT / "README.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
inventory = {p.relative_to(OUT).as_posix(): dict(bytes=p.stat().st_size, sha256=sha(p.read_bytes())) for p in sorted(OUT.rglob("*")) if p.is_file() and p.name != "inventory.sha256.json"}
(OUT / "inventory.sha256.json").write_text(json.dumps(inventory, indent=2) + "\n")
temporary = ROOT.parent / "hardware-results-v5.finalizing.zip"
with zipfile.ZipFile(temporary, "x", compression=zipfile.ZIP_DEFLATED) as z:
    for p in sorted(OUT.rglob("*")):
        if p.is_file():
            z.write(p, p.relative_to(OUT).as_posix())
with zipfile.ZipFile(temporary) as z:
    assert z.testzip() is None, "Archive CRC verification failed"
    for name, info in inventory.items():
        assert sha(z.read(name)) == info["sha256"], name
os.replace(temporary, TARGET)
verification = dict(archive=str(TARGET), bytes=TARGET.stat().st_size, sha256=sha(TARGET.read_bytes()), entries=len(inventory)+1, crc_and_all_file_sha256_verified=True, procedure_completed=state["completed"])
TARGET.with_suffix(".zip.sha256").write_text(verification["sha256"] + "  " + TARGET.name + "\n")
TARGET.with_suffix(".verification.json").write_text(json.dumps(verification, indent=2) + "\n")
print(json.dumps(verification, indent=2))
