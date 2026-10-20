#!/usr/bin/env python3
"""Direct, challenge-bound serial measurement with host timestamps and random phases."""
import argparse
import datetime
import hashlib
import itertools
import json
from pathlib import Path
import re
import secrets
import sys
import time
import uuid
import serial

p=argparse.ArgumentParser(description=__doc__)
p.add_argument('--port',required=True)
p.add_argument('--output',required=True,type=Path,help='New output prefix, e.g. captures/boot-1')
p.add_argument('--reset',action='store_true',help='Request RTS reset; manual reset often works better for native USB')
p.add_argument('--timeout',type=int,default=2400)
args=p.parse_args()
args.output.parent.mkdir(parents=True,exist_ok=True)
paths={k:Path(str(args.output)+s) for k,s in [('raw','.raw.log'),('events','.events.jsonl'),('meta','.metadata.json'),('plan','.plan.json')]}
if any(x.exists() for x in paths.values()):raise SystemExit('Use a new output prefix; existing captures will not be overwritten.')
clock=lambda:dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),monotonic_ns=time.monotonic_ns())
challenge=secrets.token_hex(16)
rng=secrets.SystemRandom()
replay=json.loads(Path('/workspace/riscv-int8-autotune/results/hardware-v5/emulator-1792432295820874232.plan.json').read_text());plan=replay['runs'];calibrations=replay['calibrations']
metadata=dict(schema=5,host_capture_uuid=uuid.uuid4().hex,challenge=challenge,port=args.port,started=clock(),completed=False,reset_requested=args.reset)
paths['plan'].write_text(json.dumps(dict(challenge=challenge,calibrations=calibrations,runs=plan),indent=2)+'\n')
raw=paths['raw'].open('xb');events=paths['events'].open('x');port=None;buffer=b'';deadline=time.monotonic()+args.timeout

def event(kind,**kw):
    events.write(json.dumps(dict(type=kind,**clock(),**kw),separators=(',',':'))+'\n');events.flush()

def send(command):
    data=(command+'\n').encode('ascii');event('send',command=command)
    port.write(data);port.flush()

def wait_for(kind,timeout=120):
    global buffer
    until=min(deadline,time.monotonic()+timeout)
    while time.monotonic()<until:
        if b'\n' not in buffer:
            data=port.read(4096)
            if not data:continue
            raw.write(data);raw.flush();buffer+=data
        while b'\n' in buffer:
            line,buffer=buffer.split(b'\n',1);text=line.decode('utf-8',errors='replace').strip()
            event('line',text=text)
            match=re.search(r'(V[0-9]+_[A-Z_]+) (\{.*\})',text)
            if not match:continue
            name=match[1];record=json.loads(match[2])
            if not name.startswith('V5_'):raise RuntimeError('Wrong firmware protocol: '+name)
            if name=='V5_ERROR':raise RuntimeError('Device error: '+text)
            if 'challenge' in record and record['challenge']!=challenge:raise RuntimeError('Wrong session challenge')
            if name==kind:return record
    raise TimeoutError('Timed out waiting for '+kind)

try:
    print('Capture',metadata['host_capture_uuid'],'challenge',challenge,flush=True)
    # Native USB can disappear briefly after reset. Retry opening before a session starts.
    while port is None and time.monotonic()<deadline:
        try:
            port=serial.serial_for_url(args.port,baudrate=115200,timeout=.05,do_not_open=True)
            port.dtr=False;port.rts=False;port.open()
        except (OSError,serial.SerialException):
            if port is not None:
                try:port.close()
                except Exception:pass
            port=None;time.sleep(.5)
    if port is None:raise TimeoutError('Serial port unavailable')
    if args.reset:
        port.rts=True;time.sleep(.15);port.rts=False;time.sleep(1)
    send('HELLO '+challenge)
    ready=wait_for('V5_READY',90);metadata['ready']=ready
    oracle_path=Path('/workspace/riscv-int8-autotune/hardware/esp32c3-v5/main/generated/oracle.json')
    if not oracle_path.exists():oracle_path=Path(__file__).parent/'main/generated/oracle.json'
    if ready['build_id']!=json.loads(oracle_path.read_text())['build_id']:raise RuntimeError('Firmware build mismatch; use V5 image and scripts together')
    send('BEGIN '+challenge);session=wait_for('V5_SESSION');metadata['session']=session
    send('CHECK '+challenge);check=wait_for('V5_CHECK_DONE',600)
    if check['checks']!=752 or check['mismatches']!=0:raise RuntimeError('Intermediate correctness failure')
    for c in calibrations:
        send(f'CAL {challenge} {c["seq"]} {c["requested_us"]}')
        got=wait_for('V5_CAL',30)
        if got['seq']!=c['seq'] or got['requested_us']!=c['requested_us']:raise RuntimeError('Calibration request mismatch')
    for n,r in enumerate(plan):
        # Host jitter is recorded separately; neither jitter nor warmup is in the measured kernel interval.
        time.sleep(secrets.randbelow(21)/1000)
        send(f'RUN {challenge} {r["seq"]} {r["model"]} {r["backend"]} {r["sample"]} {r["phase_us"]} {r["pause_ticks"]}')
        got=wait_for('V5_RUN',120)
        for k in ['seq','model','backend','sample','round']:
            if got[k]!=r[k]:raise RuntimeError('Run request mismatch: '+k)
        if got['requested_phase_us']!=r['phase_us'] or got['pause_ticks']!=r['pause_ticks'] or got['mismatches']!=0:raise RuntimeError('Incorrect run')
        if (n+1)%16==0:print(f'{n+1}/160 runs captured',flush=True)
    send('END '+challenge);done=wait_for('V5_DONE');metadata['device_result']=done
    if not done['correct'] or done['mismatches']!=0:raise RuntimeError('Device reports failure')
    metadata['completed']=True
    print('Complete:',paths['raw'],flush=True)
except BaseException as e:
    metadata['error']=str(e);print('Capture failed; keep all files:',e,file=sys.stderr)
finally:
    if port is not None:port.close()
    raw.close();events.close();metadata['finished']=clock()
    metadata['hashes']={k:hashlib.sha256(paths[k].read_bytes()).hexdigest() for k in ['raw','events','plan']}
    paths['meta'].write_text(json.dumps(metadata,indent=2)+'\n')
raise SystemExit(0 if metadata['completed'] else 2)
