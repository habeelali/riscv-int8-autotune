#!/usr/bin/env python3
"""Export and rebenchmark the Rust schedule from an audited physical V4 capture.

This never promotes an emulator-selected schedule or replaces validated exports.
"""
import argparse,copy,json,hashlib
from pathlib import Path
from runner import ROOT,run
from resnet_model import DEST
from tune_algorithm import parse_constants
from model_import import array_decl
from benchmark_resnet import benchmark
p=argparse.ArgumentParser(description=__doc__);p.add_argument('audit',type=Path);p.add_argument('--output',type=Path,required=True);p.add_argument('--emulator-functional-test',action='store_true');a=p.parse_args()
audit=json.loads(a.audit.read_text())
if a.emulator_functional_test:
    assert audit['emulation_only'] and audit['valid_captures']>0,'Functional-only export requires audited emulator data'
else:
    assert not audit['emulation_only'] and audit['independent_repeatability_evidence'],'Need three valid physical captures with frozen choices'
choices=audit['captures'][0]['selection']['choices'];out=a.output;out.mkdir(parents=True,exist_ok=False)
paths={3:DEST/'generated-0',4:DEST/'tile8_b2p4',5:DEST/'tile16_b2p4',6:DEST/'tile16_b2p4_sram',7:DEST/'tile16_b4p2'}
if audit.get('backends',[None]*8)[6]=='tile16_b2p4_block_sram':paths[6]=DEST/'tile16_b2p4_block_sram'
manifests={b:json.loads((path/'manifest.json').read_text()) for b,path in paths.items()}
constants={b:parse_constants((path/'parameters.h').read_text()) for b,path in paths.items()}
m=copy.deepcopy(manifests[3]);params={}
for j,l in enumerate(m['layers']):
    if 'workload' not in l:continue
    b=choices[31+j];m['layers'][j]=copy.deepcopy(manifests[b]['layers'][j])
    for key in ['weights','bias','mult','shift']:
        n=f'r_{key}_{j}';params[n]=constants[b][n]
(out/'manifest.json').write_text(json.dumps(m,indent=2)+'\n')
run(['cargo','run','--quiet','--locked','-p','kernel-generator','--example','algorithm_model','--',out/'manifest.json',out/'kernels.rs'],cwd=ROOT)
header='#include <stdint.h>\n'+''.join(array_decl(n,v,t) for n,(t,v) in params.items());(out/'parameters.h').write_text(header)
measurement_key='hardware-export-'+audit['build_id']+'-'+hashlib.sha256(json.dumps(choices).encode()).hexdigest()[:8]+('-emulator' if a.emulator_functional_test else '')
result=benchmark(m,measurement_key,source=(out/'kernels.rs').read_text(),constants=header)
# Exporting a correct candidate is not promotion. Report held-out physical ratios
# and retained incumbent so a loss cannot be hidden by layer-level selection.
result.update(audit_sha256=hashlib.sha256(a.audit.read_bytes()).hexdigest(),emulation_only=audit['emulation_only'],physical_comparisons=[] if audit['emulation_only'] else [r for r in audit['comparisons'] if r['backend']=='selected_rust'],person_export='generated/person-detection/hardware, unchanged',promotion='not automatic; inspect held-out whole-model timing and production size')
(out/'validation.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(dict(output=str(out),qemu_correct=result['correct'],text_bytes=result['text_bytes']),indent=2))
