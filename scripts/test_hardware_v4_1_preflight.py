#!/usr/bin/env python3
"""Reject real older emulator firmware before any CHECK or profiling command."""
import json,os,shutil,socket,subprocess,tempfile,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];H=ROOT/'hardware/esp32c3-v4_1';D=ROOT/'results/hardware-v4_1';D.mkdir(exist_ok=True)
q='/home/agent/.espressif/tools/qemu-riscv32/esp_develop_9.0.0_20240606/qemu/bin/qemu-system-riscv32';python='/home/agent/.espressif/python_env/idf5.4_py3.12_env/bin/python';results=[]
with tempfile.TemporaryDirectory() as td:
 t=Path(td)
 for name,image,expected in [('old_protocol','/workspace/esp32c3-two-models-v3.bin','Wrong firmware protocol'),('old_build','/workspace/esp32c3-generalize-v4.bin','Device firmware build differs')]:
  shutil.copyfile(image,t/'flash.bin');shutil.copyfile('/workspace/esp32c3-qemu-efuse.bin',t/'efuse.bin')
  with socket.socket() as s:s.bind(('127.0.0.1',0));port=s.getsockname()[1]
  cmd=[q,'-M','esp32c3','-display','none','-monitor','none','-chardev',f'socket,id=serial0,host=127.0.0.1,port={port},server=on,wait=on','-serial','chardev:serial0','-drive',f'file={t}/flash.bin,if=mtd,format=raw','-drive',f'file={t}/efuse.bin,if=none,format=raw,id=efuse','-global','driver=nvram.esp32c3.efuse,property=drive,value=efuse','-global','driver=timer.esp32c3.timg,property=wdt_disable,value=true']
  proc=subprocess.Popen(cmd,env=dict(os.environ,LD_LIBRARY_PATH='/workspace/tools/usr/lib/x86_64-linux-gnu'),stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
  try:
   time.sleep(.3);prefix=t/name
   r=subprocess.run([python,H/'collect.py','--port',f'socket://127.0.0.1:{port}','--output',prefix,'--timeout','30'],capture_output=True,text=True,timeout=35)
   m=json.loads(Path(str(prefix)+'.metadata.json').read_text());assert r.returncode==2 and expected in m['error'],(r.stdout,r.stderr,m)
   commands=[json.loads(l)['command'] for l in Path(str(prefix)+'.events.jsonl').read_text().splitlines() if json.loads(l)['type']=='send'];assert len(commands)==1 and commands[0].startswith('HELLO ')
   results.append(dict(case=name,passed=True,error=m['error'],commands_sent=['HELLO']))
  finally:
   proc.terminate();proc.wait(timeout=5)
(D/'preflight-tests.json').write_text(json.dumps(dict(emulation_only=True,tests=results),indent=2)+'\n');print(json.dumps(results,indent=2))
