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
from selection import select,validate_choices

p=argparse.ArgumentParser(description=__doc__)
p.add_argument('--port',required=True)
p.add_argument('--output',required=True,type=Path,help='New output prefix, e.g. captures/boot-1')
p.add_argument('--reset',action='store_true',help='Request RTS reset; manual reset often works better for native USB')
p.add_argument('--timeout',type=int,default=3600)
p.add_argument('--selection',type=Path,help='Frozen selection JSON from boot 1; do not retune later validation boots')
args=p.parse_args()
args.output.parent.mkdir(parents=True,exist_ok=True)
paths={k:Path(str(args.output)+s) for k,s in [('raw','.raw.log'),('events','.events.jsonl'),('meta','.metadata.json'),('plan','.plan.json')]}
if any(x.exists() for x in paths.values()):raise SystemExit('Use a new output prefix; existing captures will not be overwritten.')
clock=lambda:dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),monotonic_ns=time.monotonic_ns())
challenge=secrets.token_hex(16)
rng=secrets.SystemRandom()
schedule=list(itertools.product(range(2),range(10),range(4),range(5)));rng.shuffle(schedule)
occurrences={};plan=[]
for seq,(m,b,s,_) in enumerate(schedule,100):
    key=(m,b,s);round=occurrences.get(key,0);occurrences[key]=round+1
    plan.append(dict(seq=seq,model=m,backend=b,sample=s,round=round,phase_us=secrets.randbelow(9001),pause_ticks=1+secrets.randbelow(3)))
profile_schedule=list(itertools.product(range(2),range(8),range(2),range(3)));rng.shuffle(profile_schedule)
profiles=[dict(seq=j,model=m,backend=b,sample=s,phase_us=secrets.randbelow(9001),pause_ticks=1+secrets.randbelow(3)) for j,(m,b,s,_) in enumerate(profile_schedule,4)]
calibrations=[dict(seq=i+1,requested_us=base+secrets.randbelow(300)) for i,base in enumerate([300,2300,6100])]
metadata=dict(schema=4,host_capture_uuid=uuid.uuid4().hex,challenge=challenge,port=args.port,started=clock(),completed=False,reset_requested=args.reset)
paths['plan'].write_text(json.dumps(dict(challenge=challenge,calibrations=calibrations,profiles=profiles,runs=plan),indent=2)+'\n')
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
        data=port.read(4096)
        if not data:continue
        raw.write(data);raw.flush();buffer+=data
        while b'\n' in buffer:
            line,buffer=buffer.split(b'\n',1);text=line.decode('utf-8',errors='replace').strip()
            event('line',text=text)
            match=re.search(r'(V4_[A-Z_]+) (\{.*\})',text)
            if not match:continue
            name=match[1];record=json.loads(match[2])
            if name=='V4_ERROR':raise RuntimeError('Device error: '+text)
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
    ready=wait_for('V4_READY',90);metadata['ready']=ready
    send('BEGIN '+challenge);session=wait_for('V4_SESSION');metadata['session']=session
    send('CHECK '+challenge);check=wait_for('V4_CHECK_DONE',600)
    if check['checks']!=1504 or check['mismatches']!=0:raise RuntimeError('Intermediate correctness failure')
    for c in calibrations:
        send(f'CAL {challenge} {c["seq"]} {c["requested_us"]}')
        got=wait_for('V4_CAL',30)
        if got['seq']!=c['seq'] or got['requested_us']!=c['requested_us']:raise RuntimeError('Calibration request mismatch')
    measured=[]
    for n,r in enumerate(profiles):
        send(f'PROFILE {challenge} {r["seq"]} {r["model"]} {r["backend"]} {r["sample"]} {r["phase_us"]} {r["pause_ticks"]}')
        got=wait_for('V4_PROFILE',120)
        for k in ['seq','model','backend','sample']:
            if got[k]!=r[k]:raise RuntimeError('Profile mismatch: '+k)
        if got['mismatches']!=0:raise RuntimeError('Incorrect profile')
        measured.append(got)
        if (n+1)%16==0:print(f'{n+1}/96 profiles captured',flush=True)
    selection=json.loads(args.selection.read_text()) if args.selection else select(measured)
    if not validate_choices(selection['choices']):raise RuntimeError('Invalid frozen selection')
    if args.selection and selection['build_id']!=session['build_id']:raise RuntimeError('Selection firmware mismatch')
    selection.update(build_id=session['build_id'])
    metadata['selection_mode']='frozen' if args.selection else 'trained'
    metadata['selection']=selection
    selection_path=Path(str(args.output)+'.selection.json')
    if selection_path.exists():raise RuntimeError('Selection output exists')
    selection_path.write_text(json.dumps(selection,indent=2)+'\n')
    send('SELECT '+challenge+' '+' '.join(map(str,selection['choices'])))
    got=wait_for('V4_SELECTED')
    if got['choices']!=selection['choices']:raise RuntimeError('Selection response mismatch')
    send('VERIFY '+challenge)
    got=wait_for('V4_VERIFY_DONE',600)
    if got['checks']!=1880 or got['mismatches']!=0:raise RuntimeError('Selected graph correctness failure')
    for n,r in enumerate(plan):
        # Host jitter is recorded separately; neither jitter nor warmup is in the measured kernel interval.
        time.sleep(secrets.randbelow(21)/1000)
        send(f'RUN {challenge} {r["seq"]} {r["model"]} {r["backend"]} {r["sample"]} {r["phase_us"]} {r["pause_ticks"]}')
        got=wait_for('V4_RUN',120)
        for k in ['seq','model','backend','sample','round']:
            if got[k]!=r[k]:raise RuntimeError('Run request mismatch: '+k)
        if got['requested_phase_us']!=r['phase_us'] or got['pause_ticks']!=r['pause_ticks'] or got['mismatches']!=0:raise RuntimeError('Incorrect run')
        if (n+1)%16==0:print(f'{n+1}/400 runs captured',flush=True)
    send('END '+challenge);done=wait_for('V4_DONE');metadata['device_result']=done
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
