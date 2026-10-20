#!/usr/bin/env python3
"""Negative audit tests using copies of a real emulator capture, never benchmarks."""
import hashlib,json,shutil,subprocess,sys,tempfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];D=ROOT/'results/hardware-v4';H=ROOT/'hardware/esp32c3-v4'
prefix=ROOT/json.loads((D/'emulator-functional-check.json').read_text())['capture_prefix']
# Choose a trained capture even after the frozen-path test updates the result.
if json.loads(Path(str(prefix)+'.metadata.json').read_text())['selection_mode']!='trained':
 summary=json.loads((D/'emulator-audit/summary.json').read_text());prefix=Path(next(c['prefix'] for c in summary['captures'] if c['selection_mode']=='trained'))
results=[]
with tempfile.TemporaryDirectory() as td:
 for case in ['control','changed_raw','changed_selected_map','missing_profile','frozen_without_training']:
  out=Path(td)/case;out.mkdir();p=out/'capture'
  for suffix in ['.raw.log','.events.jsonl','.metadata.json','.plan.json','.selection.json']:shutil.copyfile(str(prefix)+suffix,str(p)+suffix)
  meta_path=Path(str(p)+'.metadata.json');meta=json.loads(meta_path.read_text())
  if case=='changed_raw':
   with Path(str(p)+'.raw.log').open('ab') as f:f.write(b'changed\n')
  elif case=='changed_selected_map':meta['selection']['choices'][31]=7 if meta['selection']['choices'][31]!=7 else 3
  elif case=='missing_profile':
   raw=Path(str(p)+'.raw.log');lines=raw.read_text().splitlines(keepends=True);victim=next(x for x in lines if x.startswith('V4_PROFILE '));lines.remove(victim);raw.write_text(''.join(lines))
   events=Path(str(p)+'.events.jsonl');rows=[json.loads(x) for x in events.read_text().splitlines()];rows=[r for r in rows if r.get('text')!=victim.strip()];events.write_text(''.join(json.dumps(r)+'\n' for r in rows))
   for key,path in [('raw',raw),('events',events)]:meta['hashes'][key]=hashlib.sha256(path.read_bytes()).hexdigest()
  elif case=='frozen_without_training':meta['selection_mode']='frozen'
  meta_path.write_text(json.dumps(meta))
  r=subprocess.run([sys.executable,H/'audit.py',p,'--oracle',H/'main/generated/oracle.json','--output',out/'audit','--emulator'],text=True,capture_output=True)
  assert (r.returncode==0)==(case=='control'),(case,r.stdout,r.stderr)
  results.append(dict(case=case,passed=True,expected_acceptance=case=='control',audit_exit_code=r.returncode))
(D/'negative-audit-tests.json').write_text(json.dumps(dict(scope='Deliberately corrupted test fixtures, not performance measurements',tests=results),indent=2)+'\n')
print(len(results),'audit controls passed')
