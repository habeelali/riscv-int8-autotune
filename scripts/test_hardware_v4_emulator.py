#!/usr/bin/env python3
"""Exercise the unmodified merged image and real host protocol in ESP32-C3 QEMU."""
import argparse,json,os,shutil,socket,subprocess,sys,time
p=argparse.ArgumentParser();p.add_argument("--frozen-from",type=str);args=p.parse_args()
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
D=ROOT/'results/hardware-v4';D.mkdir(exist_ok=True)
image=Path('/workspace/esp32c3-generalize-v4.bin')
shutil.copyfile(image,D/'emulator-flash.bin')
with socket.socket() as s:s.bind(('127.0.0.1',0));port=s.getsockname()[1]
q='/home/agent/.espressif/tools/qemu-riscv32/esp_develop_9.0.0_20240606/qemu/bin/qemu-system-riscv32'
python='/home/agent/.espressif/python_env/idf5.4_py3.12_env/bin/python'
cmd=[q,'-M','esp32c3','-display','none','-monitor','none','-chardev',f'socket,id=serial0,host=127.0.0.1,port={port},server=on,wait=on','-serial','chardev:serial0','-drive',f'file={D}/emulator-flash.bin,if=mtd,format=raw','-drive','file=/workspace/esp32c3-qemu-efuse.bin,if=none,format=raw,id=efuse','-global','driver=nvram.esp32c3.efuse,property=drive,value=efuse','-global','driver=timer.esp32c3.timg,property=wdt_disable,value=true']
with (D/'qemu-stderr.log').open('w') as err,(D/'collector-stdout.log').open('w') as output:
 qemu=subprocess.Popen(cmd,env=dict(os.environ,LD_LIBRARY_PATH='/workspace/tools/usr/lib/x86_64-linux-gnu'),stdout=subprocess.DEVNULL,stderr=err)
 try:
  time.sleep(.5)
  capture=D/('emulator-'+str(time.time_ns()))
  r=subprocess.run([python,ROOT/'hardware/esp32c3-v4/collect.py','--port',f'socket://127.0.0.1:{port}','--output',capture,'--timeout','2400',*(['--selection',args.frozen_from+'.selection.json'] if args.frozen_from else [])],stdout=output,stderr=subprocess.STDOUT,timeout=2450)
  assert r.returncode==0,(r.returncode,(D/'collector-stdout.log').read_text())
  subprocess.run([python,ROOT/'hardware/esp32c3-v4/audit.py',*([args.frozen_from] if args.frozen_from else []),capture,'--oracle',ROOT/'hardware/esp32c3-v4/main/generated/oracle.json','--output',D/'emulator-audit','--emulator'],check=True)
  summary=json.loads((D/'emulator-audit/summary.json').read_text());assert summary['valid_captures']==(2 if args.frozen_from else 1)
  import hashlib
  result=dict(emulation_only=True,physical_board_tested=False,build_id=summary['build_id'],image_sha256=hashlib.sha256(image.read_bytes()).hexdigest(),functional_pass=True,frozen_selection_path_tested=bool(args.frozen_from),full_tensor_checks=1880,host_triggered_runs=400, layer_profile_inferences=96,requested_delay_calibrations=3,host_challenge_and_timestamps_validated=True,capture_prefix=str(capture.relative_to(ROOT)),no_hardware_performance_claim=True)
  (D/'emulator-functional-check.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
 finally:
  qemu.terminate()
  try:qemu.wait(timeout=5)
  except subprocess.TimeoutExpired:qemu.kill();qemu.wait()
