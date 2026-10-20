#!/usr/bin/env python3
"""Differential tests of generated code: zero points, borders, strides and tails."""
import copy,json,tempfile
from pathlib import Path
import numpy as np
from runner import ROOT,run
from reference import vectors

def test():
    cases=[]
    base=json.loads((ROOT/'results/model/cifar10-resnet/manifest.json').read_text())['layers'][0]['workload']
    for h,ic,oc,k,stride,pad,offset in [(5,3,6,3,1,1,128),(7,5,8,3,2,1,-127),(8,2,4,3,3,2,17),(3,3,2,1,2,0,0),(1,9,10,1,1,0,128),(5,1,4,3,1,0,-7)]:
        w=dict(base,height=h,width=h,input_channels=ic,output_channels=oc,kernel=k,stride=stride,padding=pad,input_offset=offset,operator='fc' if h==1 else 'conv',name='edge_case',multipliers=[1073741824]*oc,shifts=[-1-(j%4) for j in range(oc)],activation_min=-113,activation_max=119,output_offset=-9)
        cases.append(w)
    count=0
    with tempfile.TemporaryDirectory() as tmp:
        d=Path(tmp)
        for w in cases:
            inputs,weights,bias,mult,shift,expected=vectors(w)
            wt=np.array(weights,dtype=np.int64).reshape(w['output_channels'],-1)
            folded=np.array(bias)+w['input_offset']*wt.sum(axis=1)
            assert max(np.abs(folded)+128*np.abs(wt).sum(axis=1))<=1<<30
            for tile,pixels,stage in [(4,4,False),(8,2,True),(16,4,True),(8,1,False)]:
                wf=dict(w,input_offset=0)
                layer=dict(index=0,workload=wf,config=dict(block=2,pixels=pixels,unroll=1,spatial_unroll=False,im2col=True,interior=False,channel_outer=False,fast_quant=True),cache_tile=tile,original_input_zero=-w['input_offset'],stage_weights=stage,packed=True,bounded_quant=True,prevalidated=True)
                (d/'m.json').write_text(json.dumps(dict(layers=[layer])))
                run(['cargo','run','--quiet','--locked','-p','kernel-generator','--example','algorithm_model','--',d/'m.json',d/'s.rs'],cwd=ROOT)
                src=(d/'s.rs').read_text().split('#[panic_handler]')[0].replace('#![no_std]','')
                packed=wt.reshape(w['output_channels']//2,2,-1).transpose(0,2,1).reshape(-1).tolist()
                src+='fn main(){\n'
                for name,typ,data in [('i','i8',inputs),('w','i8',packed),('b','i32',folded.tolist()),('m','i32',mult),('s','i32',shift),('expected','i8',expected)]:
                    src+=f'let {name}:[{typ};{len(data)}]=['+','.join(map(str,data))+'];\n'
                src+=f'let mut o=[0i8;{len(expected)}];unsafe{{layer_0::layer_0(i.as_ptr(),w.as_ptr(),b.as_ptr(),m.as_ptr(),s.as_ptr(),o.as_mut_ptr());}}assert_eq!(o,expected);}}\n'
                (d/'test.rs').write_text(src);run(['rustc','--edition=2021','-O',d/'test.rs','-o',d/'test']);run([d/'test']);count+=1
    out=ROOT/'results/hardware-v4';out.mkdir(exist_ok=True)
    (out/'generator-differential-tests.json').write_text(json.dumps(dict(passed=True,generated_programs=count,cases=cases,scope='Actual generated scalar Rust executed on host; differential against independent Python integer reference'),indent=2)+'\n')
    print(count,'generated programs passed')
if __name__=='__main__':test()
