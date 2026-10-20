"""Unmodified muRISCV-NN scalar and portable-algorithm build descriptors."""
import functools,hashlib,json,re,time
from pathlib import Path
from runner import ROOT,CC,CFLAGS,run,fingerprints
UP=ROOT/'baselines/muriscv-nn'
KINDS=('muriscv','muriscv-portable','muriscv-asm','muriscv-portable-asm','muriscv-fixed-asm','muriscv-portable-fixed-asm')
def defines(kind):
    out=[]
    if 'portable' in kind:out+=['-DUSE_PORTABLE_VEXT']
    if kind.endswith('-asm'):out+=['-DMURISCV_NN_USE_REQUANTIZE_INLINE_ASSEMBLY']
    return out

def include_dir(kind):
    if 'fixed' not in kind:return UP/'Include'
    original=(UP/'Include/muriscv_nn_support_functions.h').read_text()
    old='    return hi;\n\n#elif __riscv_xlen == 64'
    new='    /* Local correction: q = lo>>30; required rounding is q-(q>>1). */\n    return hi - (int32_t)(lo >> 1);\n\n#elif __riscv_xlen == 64'
    assert original.count(old)==1
    patched=original.replace(old,new)
    dest=ROOT/'results/upstream-overlays'/hashlib.sha256(patched.encode()).hexdigest();dest.mkdir(parents=True,exist_ok=True)
    (dest/'muriscv_nn_support_functions.h').write_text(patched)
    (dest/'patch.json').write_text(json.dumps(dict(upstream='f69494442858980be78180868d96c3524dbf26f9',description='Correct RV32 high-multiply rounding: q - (q >> 1) rather than q, q=low_product>>30',original_fragment=old,replacement=new),indent=2)+'\n')
    return dest

def scratch_size(w,kind):
    ic,k=w['input_channels'],w['kernel'];n=ic*k*k
    if w['operator']=='fc':return 0
    if w['operator']=='depthwise':
        return 4*int(re.search(r'#define CH_IN_BLOCK_MVE \((\d+)\)',(UP/'Include/muriscv_nn_support_functions.h').read_text())[1])*k*k if 'portable' in kind else 0
    if k==1 and w['padding']==0 and ic%4==0 and w['stride']==1:return 0
    return 4*((n+3)//4*4)

def wrapper(w,kind):
    h,wd,ic,oc,k,st,pad=(w[x] for x in ('height','width','input_channels','output_channels','kernel','stride','padding'))
    oh=(h+2*pad-k)//st+1;ow=(wd+2*pad-k)//st+1
    io,oo,lo,hi=(w[x] for x in ('input_offset','output_offset','activation_min','activation_max'))
    size=scratch_size(w,kind)
    head=f'''#include "muriscv_nn_functions.h"
// Scratch: {size} bytes (local stack, included in watermark).
void kernel(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {{
uint32_t scratch[{max(1,(size+3)//4)}];
muriscv_nn_context ctx={{ {'scratch' if size else '0'}, {size} }};
muriscv_nn_dims id={{1,{h},{wd},{ic}}}, fd={{{oc},{k},{k},{ic}}}, bd={{1,1,1,{oc}}}, od={{1,{oh},{ow},{oc}}};
'''
    if w['operator']=='fc':
        head+=f'''fd.n={ic}; fd.c={oc};
muriscv_nn_fc_params p={{{io},0,{oo},{{{lo},{hi}}}}};
muriscv_nn_quant_params q={{(int32_t*)m,(int32_t*)s,1}};
int status=muriscv_nn_fully_connected_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
'''
    else:
        op='depthwise_conv' if w['operator']=='depthwise' else 'convolve'
        param='muriscv_nn_dw_conv_params' if w['operator']=='depthwise' else 'muriscv_nn_conv_params'
        head+=f'''{param} p={{{io},{oo},{'1,' if w['operator']=='depthwise' else ''}{{{st},{st}}},{{{pad},{pad}}},{{1,1}},{{{lo},{hi}}}}};
muriscv_nn_per_channel_quant_params q={{(int32_t*)m,(int32_t*)s}};
int status=muriscv_nn_{op}_wrapper_s8(&ctx,&p,&q,&id,i,&fd,w,&bd,b,&od,o);
'''
    # Nonzero status must fail execution, even if the expected tensor happened to be zero.
    head+='if(status!=MURISCV_NN_SUCCESS){ *(volatile uint32_t*)0x100000=((uint32_t)(-status+1)<<16)|0x3333; for(;;){} }\n}\n'
    return head

@functools.cache
def archive(kind):
    files=sorted(p for folder in ('ConvolutionFunctions','FullyConnectedFunctions','NNSupportFunctions') for p in (UP/'Source'/folder).glob('*.c'))
    assert files, 'muRISCV-NN sources are missing; initialize the submodule'
    include=include_dir(kind)
    headers=sorted((UP/'Include').rglob('*.h'))
    hh=hashlib.sha256(json.dumps([kind,defines(kind),CFLAGS,fingerprints()],sort_keys=True).encode())
    for p in [Path(__file__),*files,*headers,*sorted((ROOT/'baselines/c/include').glob('*.h'))]:hh.update(p.read_bytes())
    key=hh.hexdigest();dest=ROOT/'results/upstream-build'/key;dest.mkdir(parents=True,exist_ok=True)
    lib=dest/'libmuriscv.a'
    if not lib.exists():
        started=time.monotonic();commands=[]
        for j,f in enumerate(files):
            cmd=[CC,*CFLAGS,*defines(kind),'-I'+str(include),'-I'+str(UP/'Include'),'-I'+str(ROOT/'baselines/c/include'),'-c',str(f),'-o',str(dest/f'{j}.o')]
            commands.append(cmd);run(cmd)
        cmd=['riscv64-unknown-elf-ar','rcs',str(lib),*[str(dest/f'{j}.o') for j in range(len(files))]]
        commands.append(cmd);run(cmd)
        (dest/'commands.json').write_text(json.dumps(commands,indent=2)+'\n')
        (dest/'build.json').write_text(json.dumps(dict(compile_seconds=time.monotonic()-started,source_files=len(files)),indent=2)+'\n')
    return lib,key
