#!/usr/bin/env python3
"""Capture a real board's complete diagnostic run without requiring ESP-IDF."""
import atexit
import datetime
import hashlib
import uuid
import argparse
import json
from pathlib import Path
import sys
import time

import serial

parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--port',required=True)
parser.add_argument('--output',default='hardware-serial.log')
parser.add_argument('--timeout',type=int,default=900)
parser.add_argument('--no-reset',action='store_true')
args=parser.parse_args()
path=Path(args.output);path.parent.mkdir(parents=True,exist_ok=True)
if path.exists():raise SystemExit('Refusing to overwrite an existing capture: '+str(path))
metadata=dict(host_capture_uuid=str(uuid.uuid4()),started_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
              port=args.port,no_reset=args.no_reset,timeout_seconds=args.timeout,completed=False)
def save_metadata():
    if path.exists():
        metadata.update(log_bytes=path.stat().st_size,log_sha256=hashlib.sha256(path.read_bytes()).hexdigest())
    metadata['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat()
    path.with_suffix('.capture.json').write_text(json.dumps(metadata,indent=2)+'\n')
atexit.register(save_metadata)
print('Host capture UUID:',metadata['host_capture_uuid'],flush=True)
deadline=time.monotonic()+args.timeout
port=None;buffer=b'';records=0;started=False;reset_done=args.no_reset
with path.open('xb') as log:
    while time.monotonic()<deadline:
        try:
            if port is None:
                port=serial.Serial(port=None,baudrate=115200,timeout=1)
                port.dtr=False;port.rts=False;port.port=args.port;port.open()
                if not reset_done:
                    reset_done=True
                    port.rts=True;time.sleep(.15);port.rts=False
                    print('Reset requested; listening on',args.port,flush=True)
            data=port.read(4096)
            if not data:continue
            log.write(data);log.flush();buffer+=data
            while b'\n' in buffer:
                line,buffer=buffer.split(b'\n',1)
                text=line.decode('utf-8',errors='replace').strip()
                if 'HW_CAPTURE ' in text:print(text,flush=True)
                if 'HW_BEGIN ' in text:
                    started=True;print(text,flush=True)
                if 'HW_' in text:records+=1
                if records and records%100==0:print('Captured',records,'diagnostic records',flush=True)
                if 'HW_ERROR ' in text or 'Guru Meditation' in text or 'panic' in text.lower():
                    print('Device error recorded; preserving full log:',text,flush=True)
                if 'HW_DONE ' in text:
                    result=json.loads(text.split('HW_DONE ',1)[1]);metadata['completed']=True;metadata['device_result']=result;print(text,flush=True)
                    print('Complete serial log:',path.resolve(),flush=True)
                    port.close();sys.exit(0 if result.get('correct') else 2)
        except (serial.SerialException,OSError) as error:
            if port is not None:
                try:port.close()
                except Exception:pass
            port=None
            print('Waiting for serial port to reconnect:',error,flush=True)
            time.sleep(1)
print('Capture timed out; return the saved log even if it is incomplete.',file=sys.stderr)
if not started:print('No HW_BEGIN received. Check the port and press RESET; try --no-reset for native USB.',file=sys.stderr)
sys.exit(3)
