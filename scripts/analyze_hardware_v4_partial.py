#!/usr/bin/env python3
"""Audit incomplete V4 evidence; never treat training profiles as held-out wins."""
import collections,csv,hashlib,json,re,statistics as st
from pathlib import Path
R=Path(__file__).resolve().parents[1];D=R/'results/hardware-v4';RAW=D/'physical-return';OUT=D/'physical-audit';OUT.mkdir(exist_ok=True)
golden=json.loads((R/'hardware/esp32c3-v4/main/generated/oracle.json').read_text());manifest=json.loads((RAW/'manifest.json').read_text())
assert manifest['build_id']==golden['build_id']=='7492ae5875a8e422'
assert manifest['image_sha256']=='17011e9a9681a67c78659ca4e0f70490597d9b2f271c2b72870b7ddbe4ff275f'
rows=[];captures=[]
for name in ['boot-1-v4','boot-1-v4-retry','boot-1-v4-retry2']:
 p=RAW/'captures'/name;meta=json.loads(Path(str(p)+'.metadata.json').read_text());plan=json.loads(Path(str(p)+'.plan.json').read_text());paths={k:Path(str(p)+s) for k,s in [('raw','.raw.log'),('events','.events.jsonl'),('plan','.plan.json')]}
 for k,path in paths.items():assert hashlib.sha256(path.read_bytes()).hexdigest()==meta['hashes'][k]
 events=[json.loads(x) for x in paths['events'].read_text().splitlines()];assert [e['monotonic_ns'] for e in events]==sorted(e['monotonic_ns'] for e in events)
 assert [e['text'] for e in events if e['type']=='line']==[l.strip() for l in paths['raw'].read_text().splitlines()]
 token=meta['challenge'];assert token==plan['challenge'];records=[];sends={};receives={}
 for e in events:
  if e['type']=='send':
   parts=e['command'].split();assert parts[1]==token
   if parts[0] in ['CAL','PROFILE','RUN']:assert int(parts[2]) not in sends;sends[int(parts[2])]=e
  else:
   m=re.search(r'(V4_[A-Z_]+) (\{.*\})',e['text'])
   if m:
    v=json.loads(m[2]);v['type']=m[1]
    if 'challenge' in v:assert v['challenge']==token
    if 'seq' in v:assert v['seq'] not in receives;receives[v['seq']]=e
    records.append(v)
 session=next(v for v in records if v['type']=='V4_SESSION');assert session['build_id']==golden['build_id'] and session['cpu_hz']==160000000
 checks=[v for v in records if v['type']=='V4_CHECK'];expected={(m,b,s,l) for m,n in enumerate([31,16]) for b in range(8) for s in range(4) for l in range(n)}
 assert len(checks)==1504 and {(v['model'],v['backend'],v['sample'],v['layer']) for v in checks}==expected
 for v in checks:
  e=golden['models'][v['model']]['tensors'][f"{v['sample']}:{v['layer']}"];assert v['mismatches']==0 and v['output_hash']==v['expected_hash']==e['fnv1a32'] and v['bytes']==e['bytes']
 cals=[v for v in records if v['type']=='V4_CAL'];assert len(cals)==3
 for v in cals:
  q=next(q for q in plan['calibrations'] if q['seq']==v['seq']);assert v['requested_us']==q['requested_us']
  wall=v['device_end_us']-v['device_start_us'];assert abs(wall-q['requested_us'])<100+.1*q['requested_us'];assert abs(v['cycles_raw']-160*wall)<max(8000,.02*160*wall)
 profiles=[v for v in records if v['type']=='V4_PROFILE'];byplan={v['seq']:v for v in plan['profiles']}
 for v in profiles:
  q=byplan[v['seq']];assert sends[v['seq']]['command']==f'PROFILE {token} {q["seq"]} {q["model"]} {q["backend"]} {q["sample"]} {q["phase_us"]} {q["pause_ticks"]}'
  for k in ['seq','model','backend','sample']:assert v[k]==q[k]
  assert v['mismatches']==0 and v['requested_phase_us']==q['phase_us'] and v['pause_ticks']==q['pause_ticks']
  model=golden['models'][v['model']];assert v['output_hash']==model['tensors'][f"{v['sample']}:{model['operators']-1}"]['fnv1a32']
  wall=v['device_end_us']-v['device_start_us'];assert wall>0 and abs(v['cycles_raw']-160*wall)<max(8000,.02*160*wall)
  host=(receives[v['seq']]['monotonic_ns']-sends[v['seq']]['monotonic_ns'])/1e6;assert host+1>=wall/1000
  last=v['device_start_us'];assert len(v['layers'])==model['operators']
  for cycles,start,end in v['layers']:
   assert last<=start<=end<=v['device_end_us'] and cycles>0;assert abs(cycles-160*(end-start))<max(8000,.02*160*(end-start));last=end
  rows.append(dict(capture=name,**v,profile_ms=v['cycles_raw']/160000))
 sent=[e['command'] for e in events if e['type']=='send'];last=sent[-1].split();assert last[0]=='PROFILE' and list(map(int,last[3:6]))[0:2]==[1,6]
 assert not meta['completed'] and not any(v['type'] in ['V4_RUN','V4_SELECTED','V4_DONE'] for v in records)
 host_span=(receives[profiles[-1]['seq']]['monotonic_ns']-receives[cals[0]['seq']]['monotonic_ns'])/1e9
 device_span=(profiles[-1]['device_end_us']-cals[0]['device_end_us'])/1e6
 assert .98<device_span/host_span<1.02
 captures.append(dict(device_host_clock_ratio=device_span/host_span,name=name,build_id=session['build_id'],checks=len(checks),profiles=len(profiles),last_command=sent[-1],status='Incomplete; timeout on ResNet whole-weight staging; cause undetermined'))
summary=[]
for m,b,s in sorted({(r['model'],r['backend'],r['sample']) for r in rows}):
 vals=[r['profile_ms'] for r in rows if (r['model'],r['backend'],r['sample'])==(m,b,s)]
 summary.append(dict(model=m,backend=b,sample=s,count=len(vals),median_profile_ms=st.median(vals),minimum_profile_ms=min(vals),maximum_profile_ms=max(vals)))
result=dict(status='Failed complete protocol; partial training evidence only',independent_repeatability_evidence=False,held_out_runs=0,frozen_selections=0,captures=captures,training_profiles=summary,limitation='Uneven training-only samples/repeats. No held-out speedup, complete-inference comparison, confidence interval, or promotion.')
(OUT/'partial-evidence.json').write_text(json.dumps(result,indent=2)+'\n')
with (OUT/'training-profiles.csv').open('w',newline='') as f:
 w=csv.DictWriter(f,[k for k in rows[0] if k!='layers']);w.writeheader();w.writerows({k:v for k,v in row.items() if k!='layers'} for row in rows)
(OUT/'returned-file-hashes.json').write_text(json.dumps({str(p.relative_to(RAW)):hashlib.sha256(p.read_bytes()).hexdigest() for p in RAW.rglob('*') if p.is_file()},indent=2)+'\n')
print(json.dumps(dict(captures=len(captures),fixed_tensor_checks=sum(c['checks'] for c in captures),verified_partial_profiles=len(rows),held_out_runs=0),indent=2))
