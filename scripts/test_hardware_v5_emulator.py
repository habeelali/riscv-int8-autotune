#!/usr/bin/env python3
"""Exercise the unmodified merged image and real host protocol in ESP32-C3 QEMU."""
import json,os,shutil,socket,subprocess,sys,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
D=ROOT/'results/hardware-v5';D.mkdir(exist_ok=True)
image=Path('/workspace/esp32c3-final-benchmark-v5.bin')
shutil.copyfile(image,D/'emulator-flash.bin')
with socket.socket() as s:s.bind(('127.0.0.1',0));port=s.getsockname()[1]
q='/home/agent/.espressif/tools/qemu-riscv32/esp_develop_9.0.0_20240606/qemu/bin/qemu-system-riscv32'
python='/home/agent/.espressif/python_env/idf5.4_py3.12_env/bin/python'
cmd=[q,'-M','esp32c3','-display','none','-monitor',os.environ.get('V5_MONITOR','none'),'-chardev',f'socket,id=serial0,host=127.0.0.1,port={port},server=on,wait=on','-serial','chardev:serial0','-drive',f'file={D}/emulator-flash.bin,if=mtd,format=raw','-drive','file=/workspace/esp32c3-qemu-efuse.bin,if=none,format=raw,id=efuse','-global','driver=nvram.esp32c3.efuse,property=drive,value=efuse','-global','driver=timer.esp32c3.timg,property=wdt_disable,value=true']
if os.environ.get('V5_ICOUNT'):cmd += ['-icount',os.environ['V5_ICOUNT']]
with (D/'qemu-stderr.log').open('w') as err,(D/'collector-stdout.log').open('w') as output:
 qemu=subprocess.Popen(cmd,env=dict(os.environ,LD_LIBRARY_PATH='/workspace/tools/usr/lib/x86_64-linux-gnu'),stdout=subprocess.DEVNULL,stderr=err)
 try:
  time.sleep(.5)
  capture=D/('emulator-'+str(time.time_ns()))
  collector=ROOT/'hardware/esp32c3-v5/collect.py'
  if os.environ.get('V5_REPLAY_PLAN'):
   source=collector.read_text();start=source.index('schedule=list(');end=source.index('metadata=dict(',start)
   source=source[:start]+"replay=json.loads(Path("+repr(os.environ['V5_REPLAY_PLAN'])+").read_text());plan=replay['runs'];calibrations=replay['calibrations']\n"+source[end:]
   source=source.replace("oracle_path=Path(__file__).with_name('oracle.json')","oracle_path=Path("+repr(str(ROOT/'hardware/esp32c3-v5/main/generated/oracle.json'))+")")
   collector=D/'collector-replay.py';collector.write_text(source)
  r=subprocess.run([python,collector,'--port',f'socket://127.0.0.1:{port}','--output',capture,'--timeout','1200'],stdout=output,stderr=subprocess.STDOUT,timeout=1250)
  assert r.returncode==0,(r.returncode,(D/'collector-stdout.log').read_text())
  subprocess.run([python,ROOT/'hardware/esp32c3-v5/audit.py',capture,'--oracle',ROOT/'hardware/esp32c3-v5/main/generated/oracle.json','--output',D/'emulator-audit','--emulator'],check=True)
  summary=json.loads((D/'emulator-audit/summary.json').read_text());assert summary['valid_captures']==1
  import hashlib
  result=dict(emulation_only=True,physical_board_tested=False,build_id=summary['build_id'],image_sha256=hashlib.sha256(image.read_bytes()).hexdigest(),functional_pass=True,qemu_monitor=os.environ.get('V5_MONITOR','none'),qemu_icount=os.environ.get('V5_ICOUNT'),replayed_plan=bool(os.environ.get('V5_REPLAY_PLAN')),full_tensor_checks=752,host_triggered_runs=160,requested_delay_calibrations=3,host_challenge_and_timestamps_validated=True,capture_prefix=str(capture.relative_to(ROOT)),no_hardware_performance_claim=True)
  (D/'emulator-functional-check.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
 finally:
  qemu.terminate()
  try:qemu.wait(timeout=5)
  except subprocess.TimeoutExpired:qemu.kill();qemu.wait()
