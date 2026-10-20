#!/usr/bin/env python3
"""Actual RV32IMC checks of upstream requantization, including optional assembly."""
import hashlib,json,random,re
from pathlib import Path
from runner import ROOT,CC,CFLAGS,QEMU,QFLAGS,run,fingerprints,versions
from reference import requantize
from muriscv_backend import KINDS,defines,include_dir
rng=random.Random(20261018)
cases=[(x,m,s) for x in (-2147483648,-1001,-5,-2,-1,0,1,2,3,5,1001,2147483647) for m in (0,1,1<<30,2147483647) for s in (-27,-9,-1,0)]
cases += [(rng.randrange(-10000000,10000000),rng.randrange(1,2147483648),rng.randrange(-27,1)) for _ in range(2048)]
expected=[requantize(*c) for c in cases]
rows=[]
for kind in KINDS:
    inc=include_dir(kind)
    dest=ROOT/'results/upstream-quant'/kind;dest.mkdir(parents=True,exist_ok=True)
    src='#include "muriscv_nn_support_functions.h"\n'
    for name,values in [('qx',[c[0] for c in cases]),('qm',[c[1] for c in cases]),('qs',[c[2] for c in cases]),('qe',expected)]:
        src+=f'static const int32_t {name}[]={{'+','.join(map(str,values))+'};\n'
    src+='''static void ch(char c){*(volatile unsigned char*)0x10000000=c;}
static void num(uint32_t v){char b[12];int n=0;do{b[n++]='0'+v%10;v/=10;}while(v);while(n)ch(b[--n]);}
int main(void){uint32_t bad=0;for(unsigned i=0;i<sizeof(qx)/sizeof(qx[0]);i++){
 int32_t actual=muriscv_nn_requantize(qx[i],qm[i],qs[i]);
 if(actual!=qe[i]){if(bad<8 || (qx[i]==2 && qm[i]==1073741824 && qs[i]==0)){ch('X');ch(' ');num(i);ch(' ');num((uint32_t)actual);ch(' ');num((uint32_t)qe[i]);ch('\\n');}bad++;}}
ch('Q');ch(' ');num(bad);ch('\\n');return 0;}
'''
    (dest/'probe.c').write_text(src)
    commands=[]
    for name,file in [('probe',dest/'probe.c'),('start',ROOT/'qemu/start.S')]:
        cmd=[CC,*CFLAGS,*defines(kind),'-I'+str(inc),'-I'+str(ROOT/'baselines/muriscv-nn/Include'),'-I'+str(ROOT/'baselines/c/include'),'-c',file,'-o',dest/(name+'.o')];commands.append(list(map(str,cmd)));run(cmd)
    cmd=[CC,*CFLAGS,'-nostdlib','-Wl,--gc-sections','-T',ROOT/'qemu/link.ld',dest/'start.o',dest/'probe.o','-lgcc','-o',dest/'probe.elf'];commands.append(list(map(str,cmd)));run(cmd)
    cmd=[QEMU,*QFLAGS,'-kernel',dest/'probe.elf'];commands.append(list(map(str,cmd)));out=run(cmd,timeout=120);(dest/'qemu.txt').write_text(out)
    mismatch=int(re.search(r'^Q (\d+)',out,re.M)[1]);examples=[]
    for i,a,e in re.findall(r'^X (\d+) (\d+) (\d+)',out,re.M):
        index=int(i);a=int(a);a=a-2**32 if a>=2**31 else a
        examples.append(dict(input=cases[index],actual=a,expected=expected[index]))
    row=dict(kind=kind,cases=len(cases),mismatches=mismatch,qualified=mismatch==0,examples=examples,toolchains=versions(),toolchain_hashes=fingerprints(),commands=commands,artifact=str(dest.relative_to(ROOT)))
    rows.append(row);print(kind,mismatch,'mismatches',flush=True)
(ROOT/'results/upstream-quantization.json').write_text(json.dumps(rows,indent=2)+'\n')
