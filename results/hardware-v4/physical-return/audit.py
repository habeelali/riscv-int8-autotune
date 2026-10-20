#!/usr/bin/env python3
"""Audit raw challenge/response, reference hashes, host clocks and timing repetition."""
import argparse
import collections
import csv
import hashlib
import json
import math
from pathlib import Path
import re
import statistics as st
from selection import select,validate_choices,BACKENDS

p=argparse.ArgumentParser(description=__doc__)
p.add_argument('prefixes',nargs='+',type=Path,help='Capture prefixes passed to collect.py')
p.add_argument('--oracle',type=Path,default=Path(__file__).with_name('oracle.json'))
p.add_argument('--output',type=Path,default=Path('audit-results'))
p.add_argument('--emulator',action='store_true',help='Functional testing only; disables real-clock acceptance and hardware claims')
args=p.parse_args();args.output.mkdir(parents=True,exist_ok=True)
golden=json.loads(args.oracle.read_text());models=golden['models'];build=golden['build_id']
backends=BACKENDS
captures=[];valid=[];all_rows=[];failures=[]

def read_json(path):return json.loads(path.read_text())
def require(condition,message):
    if not condition:raise ValueError(message)

for prefix in args.prefixes:
    try:
        capture_rows=[]
        paths={k:Path(str(prefix)+s) for k,s in [('raw','.raw.log'),('events','.events.jsonl'),('meta','.metadata.json'),('plan','.plan.json')]}
        meta=read_json(paths['meta']);plan=read_json(paths['plan']);token=meta['challenge']
        require(plan['challenge']==token,'Plan/session challenge mismatch')
        require(len(plan['runs'])==400 and {(r['model'],r['backend'],r['sample'],r['round']) for r in plan['runs']}=={(m,b,s,n) for m in range(2) for b in range(10) for s in range(4) for n in range(5)},'Unbalanced run plan')
        for k in ['raw','events','plan']:require(meta['hashes'][k]==hashlib.sha256(paths[k].read_bytes()).hexdigest(),'Changed '+k+' file')
        require(meta['completed'],'Capture incomplete')
        events=[json.loads(x) for x in paths['events'].read_text().splitlines()]
        mono=[e['monotonic_ns'] for e in events];require(mono==sorted(mono),'Host event clock went backwards')
        raw_lines=[x.strip() for x in paths['raw'].read_text(errors='replace').splitlines()]
        event_lines=[x['text'] for x in events if x['type']=='line'];require(raw_lines==event_lines,'Event/raw line mismatch')
        rec=[];recv={};sends={}
        for event in events:
            if event['type']=='send':
                parts=event['command'].split()
                require(parts[1]==token,'Wrong command token')
                if parts[0]in ['CAL','RUN','PROFILE']:
                    seq=int(parts[2]);require(seq not in sends,'Duplicate sent sequence');sends[seq]=event
            elif event['type']=='line':
                m=re.search(r'(V4_[A-Z_]+) (\{.*\})',event['text'])
                if m:
                    r=dict(type=m[1],**json.loads(m[2]));rec.append(r)
                    if 'challenge'in r:require(r['challenge']==token,'Wrong device challenge')
                    if r['type']in ['V4_RUN','V4_CAL','V4_PROFILE']:
                        require(r['seq']not in recv,'Duplicate response sequence');recv[r['seq']]=event
        require(not any(r['type']=='V4_ERROR' for r in rec),'Device error')
        session=[r for r in rec if r['type']=='V4_SESSION'];end=[r for r in rec if r['type']=='V4_DONE']
        profiles=[r for r in rec if r['type']=='V4_PROFILE']
        checks=[r for r in rec if r['type']=='V4_CHECK'];runs=[r for r in rec if r['type']=='V4_RUN'];cals=[r for r in rec if r['type']=='V4_CAL']
        require(len(session)==len(end)==1 and session[0]['build_id']==end[0]['build_id']==build,'Firmware/session mismatch')
        require(session[0]['cpu_hz']==160000000,'Wrong CPU clock')
        require(end[0]['correct'] and end[0]['mismatches']==0 and end[0]['checks']==1880 and end[0]['runs']==400 and end[0]['calibrations']==3,'Incorrect or incomplete device summary')
        keys={(r['model'],r['backend'],r['sample'],r['layer']) for r in checks}
        expected={(m,b,s,l) for m in range(2) for b in range(10) for s in range(4) for l in range(models[m]['operators'])}
        require(len(checks)==1880 and keys==expected,'Missing/duplicate tensor checks')
        for r in checks:
            e=models[r['model']]['tensors'][f"{r['sample']}:{r['layer']}"]
            require(r['mismatches']==0 and r['output_hash']==r['expected_hash']==e['fnv1a32'] and r['bytes']==e['bytes'],'Reference tensor fingerprint mismatch')
        require(len(runs)==400 and len(cals)==3 and len(profiles)==96 and set(recv)==set(sends)==set(range(1,500)),'Measurement sequence incomplete')
        byseq={r['seq']:r for r in runs}
        for row in plan['runs']:
            r=byseq[row['seq']]
            require(sends[row['seq']]['command']==f'RUN {token} {row["seq"]} {row["model"]} {row["backend"]} {row["sample"]} {row["phase_us"]} {row["pause_ticks"]}','Recorded command differs from plan')
            for k in ['model','backend','sample','round']:require(r[k]==row[k],'Wrong run '+k)
            require(r['requested_phase_us']==row['phase_us'] and r['pause_ticks']==row['pause_ticks'],'Wrong phase request')
            require(r['mismatches']==0,'Incorrect inference')
            final=models[r['model']]['operators']-1
            require(r['output_hash']==models[r['model']]['tensors'][f"{r['sample']}:{final}"]['fnv1a32'],'Wrong final output')
        require(len(plan['profiles'])==96 and collections.Counter((r['model'],r['backend'],r['sample']) for r in plan['profiles'])==collections.Counter({(m,b,t):3 for m in range(2) for b in range(8) for t in range(2)}),'Unbalanced profiling plan')
        require([r['seq'] for r in plan['profiles']]==list(range(4,100)) and [r['seq'] for r in plan['runs']]==list(range(100,500)),'Wrong phase sequence')
        profile_byseq={r['seq']:r for r in profiles}
        for row in plan['profiles']:
            r=profile_byseq[row['seq']]
            require(sends[row['seq']]['command']==f'PROFILE {token} {row["seq"]} {row["model"]} {row["backend"]} {row["sample"]} {row["phase_us"]} {row["pause_ticks"]}','Profile command differs from plan')
            for k in ['model','backend','sample']:require(r[k]==row[k],'Profile mismatch '+k)
            require(r['mismatches']==0 and r['requested_phase_us']==row['phase_us'] and r['pause_ticks']==row['pause_ticks'],'Incorrect profile/phase')
            model=models[r['model']];require(r['output_hash']==model['tensors'][f"{r['sample']}:{model['operators']-1}"]['fnv1a32'],'Incorrect profile output')
            require(len(r['layers'])==model['operators'],'Missing layer profile')
            last_end=r['device_start_us']
            for cycles,start,end in r['layers']:
                require(cycles>0 and last_end<=start<=end<=r['device_end_us'],'Invalid layer interval')
                if not args.emulator:require(abs(cycles-160*(end-start))<max(8000,.02*160*(end-start)),'Layer clock mismatch')
                last_end=end
        selection=meta['selection'];require(selection['build_id']==build and validate_choices(selection['choices']),'Invalid selection/build')
        selected=[r for r in rec if r['type']=='V4_SELECTED'];require(len(selected)==1 and selected[0]['choices']==selection['choices'],'Selected graph differs')
        if meta['selection_mode']=='trained':require(select(profiles)['choices']==selection['choices'],'Selection not reproduced from training profiles')
        else:require(meta['selection_mode']=='frozen','Unknown selection mode')
        commands=[e['command'] for e in events if e['type']=='send']
        required_commands=['HELLO '+token,'BEGIN '+token,'CHECK '+token]
        required_commands += [f'CAL {token} {c["seq"]} {c["requested_us"]}' for c in plan['calibrations']]
        required_commands += [sends[r['seq']]['command'] for r in plan['profiles']]
        required_commands += ['SELECT '+token+' '+' '.join(map(str,selection['choices'])),'VERIFY '+token]
        required_commands += [sends[r['seq']]['command'] for r in plan['runs']]+['END '+token]
        require(commands==required_commands,'Unexpected/missing/out-of-order host commands')
        for c in plan['calibrations']:
            got=next(r for r in cals if r['seq']==c['seq']);require(got['requested_us']==c['requested_us'],'Wrong calibration delay')
        require(len({r['requested_phase_us'] for r in runs})>=100,'Insufficient requested phase variation')
        require(len({r['observed_tick_phase_us'] for r in runs})>=20,'Insufficient measured tick-phase variation')
        clock_errors=[]
        for r in [*cals,*profiles,*runs]:
            wall=r['device_end_us']-r['device_start_us'];require(wall>0,'Nonpositive device interval')
            seq=r['seq'];host=(recv[seq]['monotonic_ns']-sends[seq]['monotonic_ns'])/1000
            require(host>0,'Response before request')
            if not args.emulator:
                require(host+1000>=wall,'Device interval exceeds host round trip')
                require(abs(r['cycles_raw']-160*wall)<max(8000,.02*160*wall),'Cycle/wall clock inconsistency')
                if r['type']=='V4_CAL':require(abs(wall-r['requested_us'])<100+.1*r['requested_us'],'Delay calibration failure')
            if r['type']=='V4_RUN':
                capture_rows.append(dict(capture=str(prefix),**r,host_round_trip_ms=host/1000,device_wall_ms=wall/1000,inference_ms=r['cycles_raw']/160000))
        first,last=cals[0],runs[-1]
        host_span=(recv[last['seq']]['monotonic_ns']-recv[first['seq']]['monotonic_ns'])/1e9
        device_span=(last['device_end_us']-first['device_end_us'])/1e6
        clock_ratio=device_span/host_span
        if not args.emulator:require(.98<clock_ratio<1.02,'Device/host elapsed-clock drift exceeds 2%')
        stats=[]
        for m in range(2):
            for b in range(10):
                values=[st.median(r['cycles_raw'] for r in runs if r['model']==m and r['backend']==b and r['sample']==s) for s in range(4)]
                stats.append(dict(model=m,backend=b,mean_input_median_cycles=st.mean(values),mean_input_median_ms=st.mean(values)/160000,input_median_cycles=values))
        fingerprint=hashlib.sha256(json.dumps(sorted([(r['model'],r['backend'],r['sample'],r['round'],r['cycles_raw']) for r in runs])).encode()).hexdigest()
        info=dict(selection_mode=meta['selection_mode'],selection=selection,profiles=profiles,prefix=str(prefix),valid=True,challenge=token,host_uuid=meta['host_capture_uuid'],boot_counter=session[0]['boot_counter'],nonce_hi=session[0]['nonce_hi'],nonce_lo=session[0]['nonce_lo'],statistics=stats,phase_count=len({r['observed_tick_phase_us'] for r in runs}),timing_sequence_sha256=fingerprint,device_host_clock_ratio=clock_ratio)
        captures.append(info);valid.append(info);all_rows.extend(capture_rows)
    except (ValueError,KeyError,OSError,StopIteration) as e:
        info=dict(prefix=str(prefix),valid=False,error=str(e));captures.append(info);failures.append(info)

trained=[c['selection']['choices'] for c in valid if c['selection_mode']=='trained']
for c in list(valid):
    if c['selection_mode']=='frozen' and c['selection']['choices'] not in trained:
        c['valid']=False;c['error']='Frozen selection has no matching audited training capture';failures.append(c);valid.remove(c)
all_rows=[r for r in all_rows if any(c['prefix']==r['capture'] for c in valid)]
identities={(c['boot_counter'],c['nonce_hi'],c['nonce_lo']) for c in valid}
unique_sequences={c['timing_sequence_sha256'] for c in valid}
independent=(not args.emulator and len(valid)>=3 and len(identities)==len(valid) and len({c['challenge'] for c in valid})==len(valid) and len({c['host_uuid'] for c in valid})==len(valid) and len(unique_sequences)==len(valid) and len({tuple(c['selection']['choices']) for c in valid})==1)
comparison=[]
for m in range(2):
    for capture in valid:
        group={s['backend']:s for s in capture['statistics'] if s['model']==m}
        for b in range(3,10):
            for label,samples in [('held_out',[2,3]),('all',[0,1,2,3])]:
                ratios=[min(group[c]['input_median_cycles'][s] for c in range(3))/group[b]['input_median_cycles'][s] for s in samples]
                regression=[group[3]['input_median_cycles'][s]/group[b]['input_median_cycles'][s] for s in samples]
                comparison.append(dict(capture=capture['prefix'],model=models[m]['name'],backend=backends[b],split=label,geomean_best_c_over_rust=math.exp(st.mean(math.log(x) for x in ratios)),geomean_incumbent_over_candidate=math.exp(st.mean(math.log(x) for x in regression))))
summary=dict(schema=4,emulation_only=args.emulator,build_id=build,captures=captures,valid_captures=len(valid),failed_captures=len(failures),distinct_device_identities=len(identities),distinct_timing_sequences=len(unique_sequences),independent_repeatability_evidence=independent,comparisons=comparison,
    limitation='Three direct challenge-bound captures with randomized phases support repeatability, not cryptographic device attestation or cross-board generalization. No fabricated variance or confidence intervals.')
(args.output/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
if all_rows:
    with (args.output/'runs.csv').open('w',newline='') as f:
        w=csv.DictWriter(f,list(all_rows[0]));w.writeheader();w.writerows(all_rows)
lines=['# Two-model timing audit','',f'Valid captures: {len(valid)}. Emulation only: {args.emulator}. Independent repeatability evidence: {independent}.','',
'Validation checks raw file hashes, the exact host/device challenges, command sequence and random phases, all 1880 intermediate fingerprints against the LiteRT oracle, final inference fingerprints, delay calibration, device cycle/wall counters, and host/device elapsed-clock agreement. Distinct nonces alone are insufficient. Failed captures are retained.', '',
'| Capture | Model | Best tested C / Rust |','|---|---|']
for r in comparison:lines.append(f"| {Path(r['capture']).name} | {r['model']} | {r['backend']} | {r['split']} | {r['geomean_best_c_over_rust']:.4f}× |")
lines.extend(['','Hardware speedups must not be inferred from emulator output. When hardware captures pass, compare each boot separately and report the range across boots; there is no automatic claim of a confidence interval from three captures.'])
(args.output/'report.md').write_text('\n'.join(lines)+'\n')
print(json.dumps(dict(valid=len(valid),failed=len(failures),independent_repeatability_evidence=independent,output=str(args.output)),indent=2))
for f in failures:print(f['prefix'],f['error'])
raise SystemExit(0 if valid and not failures else 2)
