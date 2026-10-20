#!/usr/bin/env python3
"""Build/run backend; host Rust CLI supplies generated candidate sources."""
import argparse,csv,functools,hashlib,json,os,re,shutil,subprocess,sys,time,tomllib
from pathlib import Path
from reference import vectors
ROOT=Path(__file__).resolve().parents[1]
RESULTS=ROOT/'results'
prefix=Path(os.environ.get('RISCV_TOOLS_PREFIX',str(ROOT/'.tools' if (ROOT/'.tools').exists() else Path('/workspace/tools'))))
os.environ['PATH']=str(Path.home()/'.cargo/bin')+':'+str(prefix/'usr/bin')+':'+os.environ['PATH']
if (prefix/'usr/lib/x86_64-linux-gnu').exists():
    os.environ['LD_LIBRARY_PATH']=str(prefix/'usr/lib/x86_64-linux-gnu')+':'+os.environ.get('LD_LIBRARY_PATH','')
CC=os.environ.get('RISCV_CC','riscv64-unknown-elf-gcc')
QEMU=os.environ.get('QEMU','qemu-system-riscv32')
CFLAGS=['-O3','-march=rv32imc','-mabi=ilp32','-ffreestanding','-fno-builtin','-ffunction-sections','-fdata-sections','-fno-unwind-tables','-fno-asynchronous-unwind-tables']
RFLAGS=['--edition=2021','--crate-type=staticlib','--target=riscv32imc-unknown-none-elf','-C','opt-level=3','-C','panic=abort','-C','codegen-units=1','-C','overflow-checks=off']
QFLAGS=['-M','virt','-cpu','lowrisc-ibex','-smp','1','-m','4M','-nographic','-monitor','none','-bios','none','-icount','shift=0,align=off,sleep=off']

def run(cmd,**kw):
    p=subprocess.run([str(x) for x in cmd],text=True,stdout=subprocess.PIPE,stderr=subprocess.PIPE,**kw)
    if p.returncode: raise RuntimeError(f'Command failed: {cmd}\n{p.stdout}\n{p.stderr}')
    return p.stdout

def load(path):
    w=tomllib.loads(Path(path).read_text())
    assert w['isa']=='rv32imc' and w['operator'] in ('conv','depthwise','fc')
    assert w['kernel'] in (1,3) and 0<w['stride']<=4 and 0<=w['padding']<=2
    assert all(0<w[k]<=1024 for k in ('height','width','input_channels','output_channels'))
    assert w['operator']!='depthwise' or w['input_channels']==w['output_channels']
    assert w['operator']!='fc' or (w['height']==w['width']==w['kernel']==1 and w['padding']==0)
    assert not(w['kernel']==1 and w['padding']) # ESP-NN's 1x1 path does not handle padding.
    assert -128<=w['activation_min']<=w['activation_max']<=127
    assert -128<=w['input_offset']<=128 and -128<=w['output_offset']<=127
    assert -25<=w['quant_shift']<=0
    for key in ('multipliers','shifts'):
        if key in w: assert len(w[key])==w['output_channels']
    if 'multipliers' in w: assert all(0<=m<2**31 for m in w['multipliers'])
    if 'shifts' in w: assert all(-30<=s<=0 for s in w['shifts'])
    return w

def data_header(w):
    vals=vectors(w)
    s=f'#define OUT_LEN {len(vals[-1])}\n'
    for name,typ,v in zip(('input','weights','bias','mult','shift','expected'),('int8_t','int8_t','int32_t','int32_t','int32_t','int8_t'),vals):
        s+=f'static const {typ} {name}[{len(v)}] = {{'+','.join(map(str,v))+'};\n'
    return s,vals

def wrapper(w):
    h,wd,ic,oc,k,st,pad=(w[x] for x in ('height','width','input_channels','output_channels','kernel','stride','padding'))
    oh=(h+2*pad-k)//st+1; ow=(wd+2*pad-k)//st+1
    head='#include <esp_nn_ansi_headers.h>\nvoid kernel(const int8_t*i,const int8_t*w,const int32_t*b,const int32_t*m,const int32_t*s,int8_t*o) {\n'
    io,oo,lo,hi=(w[x] for x in ('input_offset','output_offset','activation_min','activation_max'))
    if w['operator']=='fc':
        return head+f'esp_nn_fully_connected_per_ch_s8_ansi(i,{io},{ic},w,0,b,o,{oc},{oo},s,m,{lo},{hi});}}\n'
    op='depthwise_conv' if w['operator']=='depthwise' else 'conv'
    params='dw_conv_params_t' if w['operator']=='depthwise' else 'conv_params_t'
    return head+f'''data_dims_t id={{{wd},{h},{ic},1}}, fd={{{k},{k},{ic},1}}, od={{{ow},{oh},{oc},1}};
{params} p={{{io},{oo},{'1,' if w['operator']=='depthwise' else ''}{{{st},{st}}},{{{pad},{pad}}},{{1,1}},{{{lo},{hi}}}}};
quant_data_t q={{(int32_t*)s,(int32_t*)m}};
esp_nn_{op}_s8_opt(&id,i,&fd,w,b,&od,o,&p,&q);}}
'''

@functools.cache
def versions():
    return {k:run(v).splitlines()[0] for k,v in {'gcc':[CC,'--version'],'rustc':['rustc','--version'],'qemu':[QEMU,'--version']}.items()}

@functools.cache
def fingerprints():
    paths={'rustc':run(['rustup','which','rustc']).strip(),'gcc':shutil.which(CC),'gcc_cc1':run([CC,'-print-prog-name=cc1']).strip(),'qemu':shutil.which(QEMU)}
    return {k:hashlib.sha256(Path(v).read_bytes()).hexdigest() for k,v in paths.items()}

def benchmark(path,kind,source=None,force=False):
    w=load(path)
    if kind!='c' and not (RESULTS/'baseline-milestone.json').exists():
        raise RuntimeError('Baseline milestone is required before Rust benchmarks or optimization')
    version=versions()
    cfiles=[ROOT/'baselines/esp-nn/src'/s for s in ('convolution/esp_nn_conv_opt.c','convolution/esp_nn_conv_ansi.c','convolution/esp_nn_depthwise_conv_opt.c','fully_connected/esp_nn_fully_connected_ansi.c')]
    upstream=None
    if kind.startswith('muriscv'):
        import muriscv_backend as mb
        assert kind in mb.KINDS
        upstream,upstream_key=mb.archive(kind)
        src=mb.wrapper(w,kind)
    else: src=Path(source).read_text() if source else wrapper(w)
    hh=hashlib.sha256(json.dumps([w,kind,version,fingerprints(),CFLAGS,RFLAGS,QFLAGS],sort_keys=True).encode()+src.encode())
    for p in [Path(__file__),ROOT/'scripts/reference.py',ROOT/'baselines/esp-nn.lock.json',ROOT/'qemu/harness.c',ROOT/'qemu/memory.c',ROOT/'qemu/start.S',ROOT/'qemu/link.ld',*cfiles,*sorted((ROOT/'baselines/esp-nn/include').glob('*.h')),ROOT/'baselines/esp-nn/src/common/common_functions.h']:
        hh.update(p.read_bytes())
    if upstream: hh.update(upstream_key.encode())
    key=hh.hexdigest(); build=RESULTS/'build'/key; build.mkdir(parents=True,exist_ok=True)
    result=build/'result.json'
    if result.exists() and not force:
        cached=json.loads(result.read_text())
        if not cached['correct']: raise RuntimeError(f"Numerical mismatch (cached): {cached['mismatches']}; evidence: {result}")
        return cached
    start=time.monotonic(); header,vals=data_header(w); (build/'data.h').write_text(header)
    inc=['-I'+str(ROOT/'baselines/c/include'),'-I'+str(ROOT/'baselines/esp-nn/include'),'-I'+str(ROOT/'baselines/esp-nn/src/common'),'-I'+str(build)]
    commands=[]
    def execute(cmd,**kw):
        commands.append(list(map(str,cmd)));return run(cmd,**kw)
    objects=[]
    for name,cf in [('harness',ROOT/'qemu/harness.c'),('memory',ROOT/'qemu/memory.c'),('start',ROOT/'qemu/start.S')]:
        out=build/(name+'.o'); execute([CC,*CFLAGS,'-march=rv32imc_zicsr',*inc,'-c',cf,'-o',out]); objects.append(out)
    if upstream:
        inc+=['-I'+str(mb.include_dir(kind)),'-I'+str(mb.UP/'Include')]
        (build/'wrapper.c').write_text(src)
        obj=build/'wrapper.o';execute([CC,*CFLAGS,*mb.defines(kind),*inc,'-c',build/'wrapper.c','-o',obj]);objects.extend([obj,upstream])
    elif kind=='c':
        (build/'wrapper.c').write_text(src)
        for i,cf in enumerate([build/'wrapper.c',*cfiles]):
            out=build/f'c{i}.o';execute([CC,*CFLAGS,*inc,'-c',cf,'-o',out]);objects.append(out)
    else:
        (build/'kernel.rs').write_text(src if '#[panic_handler]' in src else src+'\n#[panic_handler] fn panic(_: &core::panic::PanicInfo) -> ! { loop { core::hint::spin_loop(); } }\n')
        out=build/'kernel.a';execute(['rustc',*RFLAGS,build/'kernel.rs','-o',out]);objects.append(out)
    elf=build/'kernel.elf'
    execute([CC,*CFLAGS,'-nostdlib','-Wl,--gc-sections','-Wl,--no-relax','-Wl,-Map='+str(build/'link.map'),'-T',ROOT/'qemu/link.ld',*objects,'-lgcc','-o',elf])
    compile_seconds=time.monotonic()-start
    (build/'assembly.txt').write_text(execute(['riscv64-unknown-elf-objdump','-d',elf]))
    (build/'attributes.txt').write_text(execute(['riscv64-unknown-elf-readelf','-A',elf]))
    size=execute(['riscv64-unknown-elf-size','-A',elf]);(build/'size.txt').write_text(size)
    sections={m[0]:int(m[1]) for m in re.findall(r'^(\.\S+)\s+(\d+)\s+\d+',size,re.M)}
    # Flash includes immutable weights/input/oracle; text is reported separately.
    flash=sum(sections.get(x,0) for x in ('.text','.rodata','.data'))
    samples=[]; qseconds=time.monotonic()
    for rep in range(3):
        output=execute([QEMU,*QFLAGS,'-kernel',elf],timeout=120)
        (build/f'qemu-{rep}.txt').write_text(output)
        match=re.search(r'RESULT (\d+) (\d+) (\d+) (\d+) (\d+) (\d+) (\d+)',output)
        if not match: raise RuntimeError('No RESULT from QEMU: '+output)
        samples.append(list(map(int,match.groups())))
    if len(set(tuple(s) for s in samples))!=1: raise RuntimeError('Nondeterministic QEMU counts')
    raw,overhead,mismatch,checksum,stack,cal1,cal2=samples[0]
    if cal2-cal1!=2000: raise RuntimeError('Instruction counter calibration failed')
    r=dict(workload=w['name'],spec=w,kind=kind,cache_key=key,quantization=dict(multipliers=vals[3],shifts=vals[4]),tensor_bytes=dict(input=len(vals[0]),weights=len(vals[1]),bias=4*len(vals[2]),quant=4*(len(vals[3])+len(vals[4])),output=len(vals[5]),oracle=len(vals[5])),correct=mismatch==0,mismatches=mismatch,raw_instructions=raw,measurement_overhead=overhead,instructions=raw-overhead,checksum=checksum,stack_watermark_bytes=stack,text_bytes=sections.get('.text',0),rodata_bytes=sections.get('.rodata',0),flash_bytes=flash,static_ram_bytes=sections.get('.data',0)+sections.get('.bss',0),reserved_stack_bytes=65536,scratch_bytes=int((re.search(r'Scratch: (\d+) bytes',src) or [None,'0'])[1]),scratch_location='stack',compile_seconds=compile_seconds,qemu_seconds=time.monotonic()-qseconds,toolchains=version,toolchain_hashes=fingerprints(),cflags=CFLAGS,rustflags=RFLAGS,qemu_flags=QFLAGS,calibration=[cal1,cal2],samples=samples,build_dir=str(build.relative_to(ROOT) if build.is_relative_to(ROOT) else build),commands=commands)
    if upstream: r.update(upstream_commit=json.loads((ROOT/'baselines/muriscv-nn.lock.json').read_text())['commit'],upstream_defines=mb.defines(kind),upstream_archive_key=upstream_key,upstream_build_commands=str(upstream.parent.relative_to(ROOT)/'commands.json'))
    result.write_text(json.dumps(r,indent=2)+'\n')
    if mismatch: raise RuntimeError(f'Numerical mismatch: {mismatch}; evidence: {result}')
    return r

def save(rows,path):
    path=Path(path);path.parent.mkdir(parents=True,exist_ok=True)
    path.write_text(json.dumps(rows,indent=2)+'\n')
    fields=['workload','kind','instructions','raw_instructions','measurement_overhead','text_bytes','flash_bytes','static_ram_bytes','scratch_bytes','stack_watermark_bytes','correct','compile_seconds','cache_key']
    with path.with_suffix('.csv').open('w') as f:
        writer=csv.DictWriter(f,fields,extrasaction='ignore');writer.writeheader();writer.writerows(rows)

def main():
    p=argparse.ArgumentParser();p.add_argument('command',choices=['baseline','one']);p.add_argument('--workload');p.add_argument('--kind',default='c');p.add_argument('--source');p.add_argument('--out');p.add_argument('--force',action='store_true');a=p.parse_args()
    if a.command=='baseline':
        rows=[]
        for path in sorted((ROOT/'workloads').glob('*.toml')):
            r=benchmark(path,'c',force=a.force);rows.append(r);save(rows,ROOT/'results/baseline.json');print(r['workload'],r['instructions'],flush=True)
        (ROOT/'results/baseline-milestone.json').write_text(json.dumps(dict(validated=True,workloads=len(rows),baseline='ESP-NN generic optimized',upstream_commit=json.loads((ROOT/'baselines/esp-nn.lock.json').read_text())['commit'],metric='QEMU executed guest instructions',completed_utc=run(['date','-u','+%FT%TZ']).strip()),indent=2)+'\n')
    else:
        r=benchmark(a.workload,a.kind,a.source,a.force)
        if a.out: Path(a.out).write_text(json.dumps(r,indent=2)+'\n')
        print(json.dumps({k:r[k] for k in ('workload','kind','correct','instructions','text_bytes','cache_key')}))
if __name__=='__main__':main()
