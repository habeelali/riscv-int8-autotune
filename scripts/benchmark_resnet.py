#!/usr/bin/env python3
"""Execute the second pretrained model against established C and Rust on RV32IMC."""
import hashlib,json,re,time
from pathlib import Path
import muriscv_backend as mb
from runner import ROOT,CC,CFLAGS,QEMU,QFLAGS,RFLAGS,run,versions
from benchmark_model import wrapper
from resnet_model import load,graph_source,DEST

KINDS=['muriscv','muriscv-fixed-asm','c']

def benchmark(m,kind,source=None,constants=None):
    dest=DEST/kind;dest.mkdir(exist_ok=True)
    for name in ['vectors.h','constants.h']:(dest/name).write_bytes((DEST/name).read_bytes())
    prefixes=['r_kernel'];(dest/'graph.c').write_text(graph_source(m,prefixes))
    header='#include <stdint.h>\n'
    if source is None:
        for l in m['layers']:
            if 'workload' not in l:continue
            src=mb.wrapper(l['workload'],kind) if kind.startswith('muriscv') else wrapper(l['workload'])
            header+=src.replace('void kernel(',f'void r_kernel_layer_{l["index"]}(')
        (dest/'wrappers.c').write_text(header)
    else:
        (dest/'kernels.rs').write_text(source.replace('fn layer_','fn r_kernel_layer_'))
        run(['rustc',*RFLAGS,dest/'kernels.rs','-o',dest/'kernels.a'])
        if constants:(dest/'constants.h').write_text(constants)
    driver='''#include <stdint.h>
int8_t activations[73728],padded[38416];
extern void resnet_execute(unsigned,unsigned,unsigned);
static void put(char c){*(volatile unsigned char*)0x10000000=c;}
static void num(uint32_t x){char s[12];unsigned n=0;do{s[n++]='0'+x%10;x/=10;}while(x);while(n)put(s[--n]);}
static void str(const char*s){while(*s)put(*s++);}
static unsigned wrong=0;
void record_check(unsigned model,unsigned b,unsigned sample,unsigned layer,const int8_t*out,const int8_t*expected,unsigned n){unsigned w=0;for(unsigned i=0;i<n;i++)w+=out[i]!=expected[i];wrong+=w;str("CHECK ");num(sample);put(' ');num(layer);put(' ');num(w);put(10);}
__attribute__((noinline)) static uint32_t measure(unsigned s){uint32_t a,b;asm volatile("rdinstret %0":"=r"(a)::"memory");resnet_execute(0,s,0);asm volatile("rdinstret %0":"=r"(b)::"memory");return b-a;}
int main(void){for(unsigned s=0;s<4;s++){resnet_execute(0,s,1);uint32_t n=measure(s);str("MODEL ");num(s);put(' ');num(n);put(' ');num(wrong);put(10);}return 0;}
'''
    (dest/'main.c').write_text(driver)
    flags=[*CFLAGS,'-march=rv32imc_zicsr','-I'+str(dest),'-I'+str(ROOT/'baselines/c/include'),'-I'+str(mb.include_dir(kind) if kind.startswith('muriscv') else mb.UP/'Include'),'-I'+str(mb.UP/'Include'),'-I'+str(ROOT/'baselines/esp-nn/include'),'-I'+str(ROOT/'baselines/esp-nn/src/common')]
    objects=[];commands=[]
    def execute(cmd,**kw):commands.append(list(map(str,cmd)));return run(cmd,**kw)
    sources=[dest/'main.c',dest/'graph.c',ROOT/'qemu/start.S',ROOT/'qemu/memory.c',mb.UP/'Source/BasicMathFunctions/muriscv_nn_elementwise_add_s8.c',*[mb.UP/'Source/SoftmaxFunctions'/n for n in ['muriscv_nn_softmax_s8.c','muriscv_nn_softmax_common_s8.c']]]
    if source is None:sources.append(dest/'wrappers.c')
    if kind=='c':sources += [ROOT/'baselines/esp-nn/src'/n for n in ['convolution/esp_nn_conv_opt.c','convolution/esp_nn_conv_ansi.c','fully_connected/esp_nn_fully_connected_ansi.c']]
    for j,src in enumerate(sources):
        out=dest/f'{j}.o';execute([CC,*flags,*(mb.defines(kind) if src.name=='wrappers.c' and kind.startswith('muriscv') else []),'-c',src,'-o',out]);objects.append(out)
    lib=[]
    if kind.startswith('muriscv'):lib=[mb.archive(kind)[0]]
    if source is not None:lib=[dest/'kernels.a']
    elf=dest/'model.elf';execute([CC,*flags,'-nostdlib','-Wl,--gc-sections','-Wl,--no-relax','-T',ROOT/'results/algorithm/link.ld',*objects,*lib,'-lgcc','-o',elf])
    outputs=[]
    for rep in range(2):
        text=execute([QEMU,*QFLAGS,'-kernel',elf],timeout=300);(dest/f'qemu-{rep}.log').write_text(text)
        checks=[tuple(map(int,v)) for v in re.findall(r'CHECK (\d+) (\d+) (\d+)',text)];measures=[tuple(map(int,v)) for v in re.findall(r'MODEL (\d+) (\d+) (\d+)',text)]
        assert len(checks)==64 and {a[:2] for a in checks}=={(s,l) for s in range(4) for l in range(16)}
        assert all(v[2]==0 for v in checks) and len(measures)==4 and all(v[2]==0 for v in measures),(kind,checks,measures)
        outputs.append(measures)
    assert outputs[0]==outputs[1]
    size=execute(['riscv64-unknown-elf-size','-A',elf]);(dest/'size.txt').write_text(size)
    sections={a:int(b) for a,b in re.findall(r'^(\.\S+)\s+(\d+)\s+\d+',size,re.M)}
    result=dict(kind=kind,correct=True,samples=[dict(sample=s,instructions_raw=n,mismatches=w) for s,n,w in outputs[0]],mean_instructions_raw=sum(n for s,n,w in outputs[0])/4,
                text_bytes=sections.get('.text',0),rodata_bytes=sections.get('.rodata',0),static_ram_bytes=sections.get('.data',0)+sections.get('.bss',0),versions=versions(),commands=commands,model_sha256=m['model_sha256'],elf_sha256=hashlib.sha256(elf.read_bytes()).hexdigest())
    (dest/'result.json').write_text(json.dumps(result,indent=2)+'\n');print(kind,result['mean_instructions_raw'],flush=True);return result

def generic_source(m):
    from benchmark_generic import source
    result='#![no_std]\n'
    panic="#[panic_handler]\nfn panic(_: &core::panic::PanicInfo<'_>) -> ! { loop { core::hint::spin_loop(); } }"
    for l in m['layers']:
        if 'workload'not in l:continue
        text=source(l['workload']);assert panic in text
        text=text.replace('#![no_std]','').replace(panic,'').replace('fn kernel(',f'fn layer_{l["index"]}(')
        result+=f'mod layer_{l["index"]} {{\n{text}\n}}\n'
    return result+'#[panic_handler]fn panic(_: &core::panic::PanicInfo)->!{loop{core::hint::spin_loop();}}\n'

if __name__=='__main__':
    import argparse
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--generic',action='store_true');args=parser.parse_args()
    m=load()
    if args.generic:
        assert (DEST/'baseline-milestone.json').exists(),'Execute C baselines first'
        s=generic_source(m);(DEST/'generic.rs').write_text(s);benchmark(m,'generic',source=s)
    else:
        results=[]
        for kind in KINDS:results.append(benchmark(m,kind));(DEST/'baseline-results.json').write_text(json.dumps(results,indent=2)+'\n')
        (DEST/'baseline-milestone.json').write_text(json.dumps(dict(correct=True,model_sha256=m['model_sha256'],baseline_results=results),indent=2)+'\n')
