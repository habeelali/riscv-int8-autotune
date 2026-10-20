#!/usr/bin/env python3
"""Negative controls on an actual V5 emulator capture; no performance fabrication."""
import hashlib,json,shutil,subprocess,sys,tempfile
from pathlib import Path
R=Path(__file__).resolve().parents[1];D=R/'results/hardware-v5';H=R/'hardware/esp32c3-v5';prefix=R/json.loads((D/'emulator-functional-check.json').read_text())['capture_prefix'];results=[]
with tempfile.TemporaryDirectory() as td:
 for case in ['control','changed_raw','altered_command','missing_run']:
  out=Path(td)/case;out.mkdir();p=out/'capture'
  for suffix in ['.raw.log','.events.jsonl','.metadata.json','.plan.json']:shutil.copyfile(str(prefix)+suffix,str(p)+suffix)
  mp=Path(str(p)+'.metadata.json');meta=json.loads(mp.read_text())
  if case=='changed_raw':
   with Path(str(p)+'.raw.log').open('ab') as f:f.write(b'changed\n')
  elif case in ['altered_command','missing_run']:
   ep=Path(str(p)+'.events.jsonl');events=[json.loads(l) for l in ep.read_text().splitlines()]
   if case=='altered_command':
    victim=next(e for e in events if e['type']=='send' and e['command'].startswith('RUN '));victim['command']+=' 99'
   else:
    victim=next(e for e in events if e['type']=='line' and e['text'].startswith('V5_RUN '));events.remove(victim);rp=Path(str(p)+'.raw.log');lines=rp.read_text().splitlines(keepends=True);idx=next(i for i,l in enumerate(lines) if l.strip()==victim['text']);del lines[idx];rp.write_text(''.join(lines));meta['hashes']['raw']=hashlib.sha256(rp.read_bytes()).hexdigest()
   ep.write_text(''.join(json.dumps(e)+'\n' for e in events));meta['hashes']['events']=hashlib.sha256(ep.read_bytes()).hexdigest()
  mp.write_text(json.dumps(meta));r=subprocess.run([sys.executable,H/'audit.py',p,'--oracle',H/'main/generated/oracle.json','--output',out/'audit','--emulator'],capture_output=True,text=True)
  assert (r.returncode==0)==(case=='control'),(case,r.stdout,r.stderr)
  results.append(dict(case=case,passed=True,exit_code=r.returncode))
(D/'negative-audit-tests.json').write_text(json.dumps(dict(scope='Deliberately corrupted fixtures, not measurements',tests=results),indent=2)+'\n');print('4 audit controls passed')
