#!/usr/bin/env python3
"""Execute the public CLI on custom workloads and independently exercise exports."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile

from reference import vectors
ROOT=Path(__file__).resolve().parents[1]
BIN=Path(os.environ.get('RISCV_INT8_BIN',ROOT/'target/debug/riscv-int8')).resolve()

def run(args,env,ok=True):
    p=subprocess.run([str(BIN),*map(str,args)],cwd='/tmp',env=env,text=True,capture_output=True,timeout=240)
    assert (p.returncode==0)==ok,(args,p.stdout,p.stderr)
    return p

def main():
    env=dict(os.environ,RISCV_INT8_ROOT=str(ROOT))
    protected=[ROOT/'results/baseline.json',ROOT/'results/generic.json',ROOT/'results/best-conv3x3.json',ROOT/'results/hardware-v5/physical-audit/summary.json']
    before={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in protected}
    results=[]
    with tempfile.TemporaryDirectory(prefix='riscv-int8-e2e-') as td:
        d=Path(td)
        assert json.loads(run(['doctor','--json'],env).stdout)['ok']
        for operator in ['conv','pointwise','depthwise','fc']:
            name='custom_'+operator;spec=d/f'{name}.toml'
            run(['init',spec,'--operator',operator],env)
            run(['init',spec,'--operator',operator],env,False)
            text=spec.read_text()
            # Odd channels, non-square inputs and non-default offsets test
            # specialization independently of the published benchmark shapes.
            import re
            updates=dict(height=1 if operator=='fc' else 5,width=1 if operator=='fc' else 7,input_channels=13 if operator=='fc' else 3,output_channels=3 if operator=='depthwise' else 5,input_offset=17,output_offset=-9)
            for key,value in updates.items():text=re.sub(rf'^{key} = .*$',f'{key} = {value}',text,flags=re.M)
            spec.write_text(text)
            output=d/(name+' job')
            run(['build',spec,'--out',output,'--max-candidates','4'],env)
            summary=json.loads((output/'summary.json').read_text());assert summary['selected']['correct']
            assert summary['evaluated_candidates']==4 and summary['feasible_candidates']>0
            assert len(summary['selected']['samples'])==3
            source=output/'export/kernel.rs'
            source_hash=hashlib.sha256(source.read_bytes()).hexdigest()
            run(['build',spec,'--out',output,'--max-candidates','4'],env,False)
            run(['build',spec,'--out',output,'--max-candidates','3','--resume'],env,False)
            run(['build',spec,'--out',output,'--max-candidates','4','--resume'],env)
            assert hashlib.sha256(source.read_bytes()).hexdigest()==source_hash
            w=summary['inspection']['spec'];checks=0
            for seed in [42,719]:
                params=dict(w,seed=seed)
                inp,weights,bias,mult,shift,expected=vectors(params)
                test='#[path='+json.dumps(str(source))+'] mod exported;\nfn main(){\n'
                for label,typ,values in [('input','i8',inp),('weights','i8',weights),('bias','i32',bias),('mult','i32',mult),('shift','i32',shift),('expected','i8',expected)]:
                    test+=f'let {label}: [{typ};{len(values)}] = '+str(values)+';\n'
                test+=f'let mut output=[0i8;{len(expected)}]; exported::run(&input,&weights,&bias,&mult,&shift,&mut output); assert_eq!(output,expected);}}'
                path=d/'check.rs';path.write_text(test)
                subprocess.run(['rustc','--edition=2021','-O',path,'-o',d/'check'],check=True,capture_output=True)
                subprocess.run([d/'check'],check=True);checks+=1
            results.append(dict(operator=operator,candidates=4,extra_export_fixtures=checks,passed=True))
        # A geometrically valid specification with an infeasible stack budget
        # must preserve candidate evidence and fail instead of exporting a winner.
        spec=d/'infeasible.toml'
        run(['init',spec,'--operator','fc'],env)
        spec.write_text(spec.read_text().replace('sram_budget = 196608','sram_budget = 1'))
        output=d/'infeasible'
        run(['build',spec,'--out',output,'--max-candidates','1'],env,False)
        assert json.loads((output/'job.json').read_text())['status']=='failed'
        assert (output/'candidates.json').exists() and not (output/'export').exists()
        # Invalid shapes fail before creating a tuning job.
        invalid=d/'invalid.toml';invalid.write_text(spec.read_text().replace('kernel = 1','kernel = 2'))
        run(['build',invalid,'--out',d/'invalid'],env,False)
        assert not (d/'invalid').exists()
    assert before=={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in protected}
    report=dict(passed=True,workloads=results,resource_rejection=True,invalid_shape_rejection=True,overwrite_protection=True,resume_identity_check=True,unrelated_working_directory=True,historical_results_unchanged=True,scope='New CLI builds run real C/Rust RV32IMC programs in QEMU; eight additional host export fixtures compare against independent Python integer outputs. No hardware timing inferred.')
    path=ROOT/'results/tool-validation.json';path.write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps(report,indent=2))

if __name__=='__main__':main()
