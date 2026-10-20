"""Execute the supplied V5 procedure once; stop hardware work on first failure."""
import datetime
import hashlib
import importlib.metadata
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
import time
import traceback
import zipfile

from serial.tools import list_ports

ROOT = Path(__file__).resolve().parent
OUT = ROOT / "hardware-results"
ARCHIVE = ROOT.parent / "hardware-results-v5.zip"
PYTHON = sys.executable
state = {"schema": 5, "mode": "physical_hardware", "completed": False, "steps": [], "resets": []}


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def write(name, value):
    (OUT / name).write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def persist():
    write("procedure-status.json", state)
    write("reset-timeline.json", state["resets"])


def run(name, args, timeout):
    step = dict(name=name, command=[PYTHON, *args], started_utc=utc())
    state["steps"].append(step)
    persist()
    print("START", name, step["started_utc"], flush=True)
    with (OUT / (name + ".console.log")).open("wb") as log:
        try:
            result = subprocess.run([PYTHON, *args], cwd=ROOT, stdout=log, stderr=subprocess.STDOUT, timeout=timeout)
            step["exit_code"] = result.returncode
        except subprocess.TimeoutExpired:
            step["timed_out"] = True
            step["finished_utc"] = utc()
            persist()
            raise RuntimeError(name + " exceeded host supervisor timeout; no retry")
    step["finished_utc"] = utc()
    persist()
    print("FINISH", name, "exit", step["exit_code"], flush=True)
    if step["exit_code"] != 0:
        raise RuntimeError(name + " failed with exit code " + str(step["exit_code"]))
    return (OUT / (name + ".console.log")).read_text(encoding="utf-8", errors="replace")


def ports():
    return [dict(device=p.device, description=p.description, vid=p.vid, pid=p.pid, serial_number=p.serial_number, hwid=p.hwid) for p in list_ports.comports()]


def native_port():
    until = time.monotonic() + 60
    observations = []
    while time.monotonic() < until:
        found = ports()
        observations.append(dict(utc=utc(), ports=found))
        matching = [p for p in found if p["vid"] == 0x303A and p["pid"] == 0x1001 and (p["serial_number"] or "").upper() == "94:A9:90:96:72:D4"]
        if len(matching) == 1:
            state.setdefault("port_observations", []).extend(observations)
            persist()
            return matching[0]["device"]
        if len(matching) > 1:
            raise RuntimeError("Ambiguous native USB target; no hardware action")
        time.sleep(.5)
    state.setdefault("port_observations", []).extend(observations)
    persist()
    raise RuntimeError("Target native USB serial port did not enumerate; no retry of hardware procedure")


def reset_record(method, step, session):
    state["resets"].append(dict(session=session, method=method, requested_utc=step["started_utc"], completed_utc=step["finished_utc"], exit_code=step["exit_code"], physical_power_cycle=False))
    persist()


def capture_summary():
    result = []
    for meta_path in sorted((OUT / "captures").glob("*.metadata.json")):
        meta = json.loads(meta_path.read_text())
        prefix = meta_path.name.removesuffix(".metadata.json")
        events_path = meta_path.with_name(prefix + ".events.jsonl")
        counts = {}
        last_send = None
        for line in events_path.read_text().splitlines():
            event = json.loads(line)
            if event["type"] == "send":
                last_send = event
            if event["type"] == "line":
                match = re.search(r"(V[0-9]+_[A-Z_]+) (\{.*\})", event["text"])
                if match:
                    counts[match[1]] = counts.get(match[1], 0) + 1
        result.append(dict(prefix=prefix, completed=meta["completed"], error=meta.get("error"), records=counts, last_sent_command=last_send, metadata_hashes=meta["hashes"]))
    write("capture-summary.json", result)


def package():
    inventory = {p.relative_to(OUT).as_posix(): dict(bytes=p.stat().st_size, sha256=sha(p)) for p in sorted(OUT.rglob("*")) if p.is_file() and p.name != "inventory.sha256.json"}
    write("inventory.sha256.json", inventory)
    with zipfile.ZipFile(ARCHIVE, "x", compression=zipfile.ZIP_DEFLATED) as z:
        for p in sorted(OUT.rglob("*")):
            if p.is_file():
                z.write(p, p.relative_to(OUT).as_posix())
    with zipfile.ZipFile(ARCHIVE) as z:
        bad = z.testzip()
        if bad:
            raise RuntimeError("Archive CRC error: " + bad)
    ARCHIVE.with_suffix(".zip.sha256").write_text(sha(ARCHIVE) + "  " + ARCHIVE.name + "\n")
    print("ARCHIVE", str(ARCHIVE), "SHA256", sha(ARCHIVE), flush=True)


def main():
    if OUT.exists() or ARCHIVE.exists():
        raise SystemExit("Refusing to overwrite prior V5 evidence")
    OUT.mkdir()
    (OUT / "captures").mkdir()
    reference = OUT / "bundle-reference"
    reference.mkdir()
    original_names = zipfile.ZipFile(ROOT.parent / "esp32c3-final-benchmark-v5.zip").namelist()
    original_hashes = {}
    for name in original_names:
        p = ROOT / name
        if p.is_file():
            original_hashes[name] = sha(p)
            if p.suffix not in (".bin", ".elf"):
                target = reference / name
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(p, target)
    shutil.copy2(__file__, OUT / Path(__file__).name)
    write("original-bundle-files.sha256.json", original_hashes)
    write("board-details.json", dict(board_model="ESP32-C3 SuperMini (user identified)", usb_connection="Native USB Serial/JTAG, VID 303A PID 1001", expected_usb_serial="94:A9:90:96:72:D4", power="Assumed host USB bus power through the existing cable; no independent rail measurement or wiring inspection", cable_or_power_changed_by_agent=False, physical_power_cycle=False))
    state["started_utc"] = utc()
    try:
        versions = dict(python=sys.version, executable=PYTHON, esptool=importlib.metadata.version("esptool"), pyserial=importlib.metadata.version("pyserial"))
        write("dependency-versions.json", versions)
        if versions["esptool"] != "4.12.0" or versions["pyserial"] != "3.5":
            raise RuntimeError("Bundled dependency version requirement not satisfied")
        manifest = json.loads((ROOT / "manifest.json").read_text())
        image = ROOT / manifest["image"]
        checksum = dict(image=image.name, bytes=image.stat().st_size, calculated_sha256=sha(image), manifest_sha256=manifest["image_sha256"], firmware_sha256=(ROOT / "firmware.sha256").read_text().split()[0], bundle_zip_sha256=sha(ROOT.parent / "esp32c3-final-benchmark-v5.zip"))
        checksum["verified"] = checksum["calculated_sha256"] == checksum["manifest_sha256"] == checksum["firmware_sha256"] and checksum["bytes"] == manifest["image_bytes"]
        write("checksum-verification.json", checksum)
        if not checksum["verified"]:
            raise RuntimeError("Image checksum or size mismatch")
        port = native_port()
        identity = run("01-chip-flash-identification", ["-m", "esptool", "--chip", "esp32c3", "--port", port, "--after", "hard_reset", "flash_id"], 90)
        if "Chip is ESP32-C3" not in identity or not re.search(r"Detected flash size:\s*(4|8|16|32|64|128)MB", identity):
            raise RuntimeError("Physical chip/flash identification did not confirm ESP32-C3 with >=4 MB")
        reset_record("esptool flash_id --after hard_reset (identification)", state["steps"][-1], None)
        port = native_port()
        flash = run("02-flash-merged-at-0x0", ["-m", "esptool", "--chip", "esp32c3", "--port", port, "--baud", "460800", "--after", "hard_reset", "write_flash", "--flash_mode", "dio", "--flash_freq", "40m", "--flash_size", "4MB", "0x0", image.name], 300)
        if "Hash of data verified" not in flash:
            raise RuntimeError("Successful flashing digest verification not found in output")
        reset_record("esptool write_flash --after hard_reset", state["steps"][-1], 1)
        for boot in range(1, 4):
            if boot > 1:
                port = native_port()
                run(f"reset-before-boot-{boot}", ["-m", "esptool", "--chip", "esp32c3", "--port", port, "--after", "hard_reset", "chip_id"], 90)
                reset_record("esptool chip_id --after hard_reset", state["steps"][-1], boot)
            port = native_port()
            prefix = OUT / "captures" / f"boot-{boot}"
            run(f"collect-boot-{boot}", ["collect.py", "--port", port, "--output", str(prefix.relative_to(ROOT)), "--timeout", "2400"], 2500)
            metadata = json.loads(Path(str(prefix) + ".metadata.json").read_text())
            if not metadata["completed"]:
                raise RuntimeError("Collector metadata reports incomplete session")
        run("audit-hardware-three-boots", ["audit.py", *[str((OUT / "captures" / f"boot-{boot}").relative_to(ROOT)) for boot in range(1, 4)], "--output", str((OUT / "audit-results").relative_to(ROOT))], 120)
        state["completed"] = True
    except BaseException as exc:
        state["first_failure"] = dict(utc=utc(), error=str(exc), traceback=traceback.format_exc())
        print("STOP_AFTER_FIRST_FAILURE", str(exc), flush=True)
    finally:
        state["finished_utc"] = utc()
        persist()
        capture_summary()
        package()
    return 0 if state["completed"] else 2


if __name__ == "__main__":
    raise SystemExit(main())
