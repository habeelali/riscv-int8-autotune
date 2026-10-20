#!/usr/bin/env python3
"""End-to-end pretrained person-detector execution on scalar RV32IMC QEMU."""
import argparse
import csv
import hashlib
import json
import re
import time
from pathlib import Path

import benchmark_generic
import muriscv_backend as mb
from model_import import load_model
from runner import CC, CFLAGS, QEMU, QFLAGS, RFLAGS, ROOT, fingerprints, run, versions, wrapper

DEST = ROOT / 'results/model/person-detection'
BASELINES = ('c', 'muriscv', 'muriscv-portable', 'muriscv-fixed-asm','muriscv-portable-fixed-asm')
KINDS = (*BASELINES, *(k+'-native' for k in BASELINES), 'generic', 'generated-0', 'generated-1',
         'generated-2', 'generated-3', 'generated-selected')


def graph_source(manifest, kind='c'):
    layers = manifest['layers']
    maximum = max(__import__('math').prod(l['outshape']) for l in layers)
    padded = max((l['workload']['height'] * l['workload']['width'] * l['workload']['input_channels']
                  for l in layers if l.get('explicit_padding')), default=1)
    source = f'''#include <stdint.h>
#include "constants.h"
#include "vectors.h"
typedef void (*step_fn)(const int8_t*,int8_t*);
static int8_t arena[2][{maximum}], padded[{padded}];
extern void *memcpy(void*,const void*,unsigned);
extern void muriscv_nn_softmax_s8(const int8_t*,int32_t,int32_t,int32_t,int32_t,int32_t,int8_t*);
static void putc_uart(char c) {{ *(volatile unsigned char*)0x10000000=c; }}
static void puts_uart(const char*s) {{while(*s)putc_uart(*s++);}}
static void number(uint32_t v) {{char b[12];int n=0;do{{b[n++]='0'+v%10;v/=10;}}while(v);while(n)putc_uart(b[--n]);}}
static void signed_number(int v) {{if(v<0){{putc_uart('-');v=-v;}}number(v);}}
'''
    for l in layers:
        j = l['index']
        if 'workload' in l:
            w = l['workload']
            source += f'extern void layer_{j}(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);\n'
            source += f'__attribute__((noinline)) static void step_{j}(const int8_t*i,int8_t*o){{\n'
            if l['explicit_padding']:
                h, wd, ic = l['shape'][1:]
                ph, pw, before, zero = w['height'], w['width'], l['pad_before'], l['input_zero']
                source += f'''for(unsigned y=0;y<{ph};y++)for(unsigned x=0;x<{pw};x++)for(unsigned c=0;c<{ic};c++){{
unsigned a=y-{before},b=x-{before};
padded[(y*{pw}+x)*{ic}+c]=(a<{h}&&b<{wd})?i[(a*{wd}+b)*{ic}+c]:{zero};}}i=padded;
'''
            weights = f'weights_native_{j}' if kind.endswith('-native') and 'lowering' in l else f'weights_{j}'
            source += f'layer_{j}(i,{weights},bias_{j},mult_{j},shift_{j},o);}}\n'
        elif l['op'] == 'AVERAGE_POOL_2D':
            channels = l['outshape'][-1]
            source += f'''__attribute__((noinline)) static void step_{j}(const int8_t*i,int8_t*o){{
for(unsigned c=0;c<{channels};c++){{int sum=0;for(unsigned p=0;p<9;p++)sum+=i[p*{channels}+c];o[c]=(sum+(sum>0?4:-4))/9;}}}}
'''
        elif l['op'] == 'RESHAPE':
            source += f'__attribute__((noinline)) static void step_{j}(const int8_t*i,int8_t*o){{memcpy(o,i,2);}}\n'
        elif l['op'] == 'SOFTMAX':
            source += f'__attribute__((noinline)) static void step_{j}(const int8_t*i,int8_t*o){{muriscv_nn_softmax_s8(i,1,2,{l["multiplier"]},{l["shift"]},{l["diff_min"]},o);}}\n'
    source += 'static step_fn steps[]={' + ','.join(f'step_{l["index"]}' for l in layers) + '};\n'
    source += 'static const unsigned lengths[]={' + ','.join(str(__import__('math').prod(l['outshape'])) for l in layers) + '};\n'
    source += f'''
__attribute__((noinline)) static void empty_step(const int8_t*i,int8_t*o){{asm volatile("" ::: "memory");}}
__attribute__((noinline)) static void empty_model(const int8_t*i){{asm volatile("" ::: "memory");}}
__attribute__((noinline)) static void model(const int8_t*i){{
for(unsigned j=0;j<{len(layers)};j++){{int8_t*o=arena[j&1];steps[j](i,o);i=o;}}}}
__attribute__((noinline)) static uint32_t measure_model(void(*f)(const int8_t*),const int8_t*i){{
uint32_t a,b;asm volatile("rdinstret %0":"=r"(a)::"memory");f(i);asm volatile("rdinstret %0":"=r"(b)::"memory");return b-a;}}
__attribute__((noinline)) static uint32_t measure_step(step_fn f,const int8_t*i,int8_t*o){{
uint32_t a,b;asm volatile("rdinstret %0":"=r"(a)::"memory");f(i,o);asm volatile("rdinstret %0":"=r"(b)::"memory");return b-a;}}
extern unsigned char __stack_bottom[],__stack_top[];
int main(void){{
for(unsigned sample=0;sample<{len(manifest['samples'])};sample++){{
uint32_t sp;asm volatile("mv %0,sp":"=r"(sp));
for(unsigned char*p=__stack_bottom;p<(unsigned char*)(sp-256);p++)*p=0xa5;
uint32_t overhead=measure_model(empty_model,images[sample]);
uint32_t raw=measure_model(model,images[sample]);
unsigned char*p=__stack_bottom;while(p<(unsigned char*)(sp-256)&&*p==0xa5)p++;
uint32_t stack=(uint32_t)(__stack_top-p);
int8_t timed_a=arena[{(len(layers)-1)&1}][0],timed_b=arena[{(len(layers)-1)&1}][1];
uint32_t mismatches=(timed_a!=oracles[sample][{len(layers)-1}][0])+(timed_b!=oracles[sample][{len(layers)-1}][1]);const int8_t*i=images[sample];
for(unsigned j=0;j<{len(layers)};j++){{int8_t*o=arena[j&1];
uint32_t empty=measure_step(empty_step,i,o),count=measure_step(steps[j],i,o),wrong=0;
for(unsigned n=0;n<lengths[j];n++)wrong+=o[n]!=oracles[sample][j][n];
mismatches+=wrong;
puts_uart("LAYER ");number(sample);putc_uart(' ');number(j);putc_uart(' ');number(count-empty);putc_uart(' ');number(wrong);putc_uart(10);i=o;}}
puts_uart("MODEL ");number(sample);putc_uart(' ');number(raw);putc_uart(' ');number(overhead);putc_uart(' ');number(mismatches);putc_uart(' ');number(stack);
putc_uart(' ');signed_number(i[0]);putc_uart(' ');signed_number(i[1]);putc_uart(10);
}}return 0;}}
'''
    return source


def sources(manifest, kind, build):
    base_kind = kind.removesuffix('-native')
    if kind.startswith('generated-'):
        source = build / 'kernels.rs'
        run(['cargo', 'run', '--locked', '--quiet', '-p', 'kernel-generator', '--example',
             'model_sources', '--', DEST/'manifest.json', source,
             str(DEST/'selection.json') if kind == 'generated-selected' else kind.split('-')[-1]], cwd=ROOT)
        return source
    parts = []
    for layer in manifest['layers']:
        if 'workload' not in layer:
            continue
        j, w = layer['index'], layer['workload']
        if kind == 'generic':
            part = benchmark_generic.source(w).replace('#![no_std]', '')
            part = re.sub(r'#\[panic_handler\]\nfn panic\([^\n]*\n', '', part)
            part = part.replace('fn kernel(', f'fn layer_{j}(')
            parts.append(f'mod layer_{j} {{\n{part}\n}}\n')
        else:
            native = kind.endswith('-native') and 'lowering' in layer
            if native:
                w = dict(w, operator='depthwise')
            part = mb.wrapper(w, base_kind) if kind.startswith('muriscv') else wrapper(w)
            if native:
                # Exact original depthwise multiplier; the rest of the graph is unchanged.
                marker = f"p={{{w['input_offset']},{w['output_offset']},1,"
                assert part.count(marker) == 1
                part = part.replace(marker, f"p={{{w['input_offset']},{w['output_offset']},8,")
            parts.append(part.replace('void kernel(', f'void layer_{j}('))
    if kind == 'generic':
        src = '#![no_std]\n' + ''.join(parts) + '\n#[panic_handler] fn panic(_: &core::panic::PanicInfo) -> ! { loop { core::hint::spin_loop(); } }\n'
        path = build / 'kernels.rs'
    else:
        src, path = ''.join(parts), build / 'kernels.c'
    path.write_text(src)
    return path


def benchmark(manifest, kind, force=False, source_override=None, rustflags=None, constants_override=None, graph_override=None, linker_override=None):
    base_kind = kind.removesuffix('-native')
    rustflags = RFLAGS if rustflags is None else rustflags
    # All target inputs, sources, libraries, flags and tool binaries participate in the cache.
    hh = hashlib.sha256(json.dumps([kind, manifest, CFLAGS, rustflags, QFLAGS, fingerprints()], sort_keys=True).encode())
    if source_override is not None:
        hh.update(Path(source_override).read_bytes())
    if constants_override is not None:
        hh.update(Path(constants_override).read_bytes())
    for override in (graph_override,linker_override):
        if override is not None:hh.update(Path(override).read_bytes())
    dependencies = [Path(__file__), ROOT/'scripts/model_import.py', ROOT/'scripts/benchmark_generic.py',
                    ROOT/'scripts/runner.py', ROOT/'scripts/muriscv_backend.py',
                    ROOT/'qemu/memory.c', ROOT/'qemu/start.S', ROOT/'qemu/link.ld',
                    DEST/'constants.h', DEST/'vectors.h',
                    ROOT/'crates/kernel-generator/examples/model_sources.rs',
                    ROOT/'crates/kernel-generator/src/lib.rs', ROOT/'crates/kernel-generator/src/quant_fast.rs',
                    ROOT/'crates/kernel-core/src/lib.rs']
    dependencies += sorted((ROOT/'baselines/esp-nn').rglob('*.c')) + sorted((ROOT/'baselines/esp-nn').rglob('*.h'))
    dependencies += sorted((mb.UP/'Source').rglob('*.c')) + sorted((mb.UP/'Include').rglob('*.h'))
    if kind == 'generated-selected':
        dependencies.append(DEST/'selection.json')
    for path in dependencies:
        hh.update(path.read_bytes())
    key = hh.hexdigest()
    build = DEST/'build'/key
    build.mkdir(parents=True, exist_ok=True)
    result_path = build/'result.json'
    if result_path.exists() and not force:
        result = json.loads(result_path.read_text())
        if not result['correct']:
            raise RuntimeError(f'Incorrect cached model: {result_path}')
        return result
    (build/'graph.c').write_text(Path(graph_override).read_text() if graph_override else graph_source(manifest, kind))
    if constants_override is not None:
        (build/'constants.h').write_text(Path(constants_override).read_text())
    commands = []

    def execute(cmd, **kw):
        commands.append(list(map(str, cmd)))
        return run(cmd, **kw)

    started = time.monotonic()
    if source_override is None:
        source = sources(manifest, kind, build)
    else:
        source = build/'kernels.rs'
        source.write_text(Path(source_override).read_text())
    inc = ['-I'+str(DEST), '-I'+str(ROOT/'baselines/c/include'),
           '-I'+str(ROOT/'baselines/esp-nn/include'), '-I'+str(ROOT/'baselines/esp-nn/src/common')]
    objects = []
    files = [build/'graph.c', ROOT/'qemu/memory.c', ROOT/'qemu/start.S']
    # Identical unmodified integer softmax is linked into every implementation.
    soft = mb.UP/'Source/SoftmaxFunctions'
    files += [soft/'muriscv_nn_softmax_s8.c', soft/'muriscv_nn_softmax_common_s8.c']
    if source.suffix == '.c':
        files.append(source)
    if base_kind == 'c':
        files += [ROOT/'baselines/esp-nn/src'/p for p in ('convolution/esp_nn_conv_opt.c', 'convolution/esp_nn_conv_ansi.c',
                  'convolution/esp_nn_depthwise_conv_opt.c')]
    if kind.startswith('muriscv'):
        library, _ = mb.archive(base_kind)
        includes = ['-I'+str(mb.include_dir(base_kind)), '-I'+str(mb.UP/'Include')]
    else:
        includes = ['-I'+str(mb.UP/'Include')]
    for j, file in enumerate(files):
        obj = build/f'{j}.o'
        # CSR extension is confined to harness/runtime; all inference libraries use RV32IMC.
        flags = ['-march=rv32imc_zicsr'] if file.name in ('graph.c', 'start.S') else []
        defines = mb.defines(base_kind) if kind.startswith('muriscv') and file == source else []
        execute([CC, *CFLAGS, *flags, *defines, *inc, *includes, '-c', file, '-o', obj])
        objects.append(obj)
    if kind.startswith('muriscv'):
        objects.append(library)
    if source.suffix == '.rs':
        obj = build/'kernels.a'
        execute(['rustc', *rustflags, source, '-o', obj])
        objects.append(obj)
    elf = build/'model.elf'
    execute([CC, *CFLAGS, '-nostdlib', '-Wl,--gc-sections', '-Wl,--no-relax',
             '-Wl,-Map='+str(build/'link.map'), '-T', linker_override or ROOT/'qemu/link.ld', *objects, '-lgcc', '-o', elf])
    compile_seconds = time.monotonic()-started
    assembly = execute(['riscv64-unknown-elf-objdump', '-d', elf])
    (build/'assembly.txt').write_text(assembly)
    (build/'attributes.txt').write_text(execute(['riscv64-unknown-elf-readelf','-A',elf]))
    size = execute(['riscv64-unknown-elf-size', '-A', elf])
    (build/'size.txt').write_text(size)
    sections = {m[0]: int(m[1]) for m in re.findall(r'^(\.\S+)\s+(\d+)\s+\d+', size, re.M)}
    repetitions = []
    started = time.monotonic()
    for rep in range(3):
        output = execute([QEMU, *QFLAGS, '-kernel', elf], timeout=300)
        (build/f'qemu-{rep}.txt').write_text(output)
        models = [list(map(int, m)) for m in re.findall(r'^MODEL (\d+) (\d+) (\d+) (\d+) (\d+) (-?\d+) (-?\d+)$', output, re.M)]
        layers = [list(map(int, m)) for m in re.findall(r'^LAYER (\d+) (\d+) (\d+) (\d+)$', output, re.M)]
        assert len(models) == len(manifest['samples']) and len(layers) == len(manifest['samples'])*len(manifest['layers']), output
        repetitions.append(dict(models=models, layers=layers))
    assert repetitions[0] == repetitions[1] == repetitions[2], 'Nondeterministic model counts'
    samples = []
    correct = True
    for row in repetitions[0]['models']:
        sample, raw, overhead, mismatches, stack, a, b = row
        correct &= mismatches == 0 and [a,b] == manifest['samples'][sample]['output']
        samples.append(dict(name=manifest['samples'][sample]['name'], instructions=raw-overhead,
                            raw_instructions=raw, measurement_overhead=overhead,
                            mismatches=mismatches, output=[a,b], stack_watermark_bytes=stack))
    result = dict(kind=kind, correct=correct, cache_key=key, samples=samples,
                  layers=[dict(sample=a, layer=b, instructions=c, mismatches=d) for a,b,c,d in repetitions[0]['layers']],
                  mean_instructions=sum(s['instructions'] for s in samples)/len(samples),
                  text_bytes=sections.get('.text',0), rodata_bytes=sections.get('.rodata',0),
                  static_ram_bytes=sections.get('.data',0)+sections.get('.bss',0),
                  reserved_stack_bytes=65536, compile_seconds=compile_seconds,
                  qemu_seconds=time.monotonic()-started, toolchains=versions(),toolchain_hashes=fingerprints(),
                  cflags=CFLAGS, rustflags=rustflags, qemu_flags=QFLAGS,
                  constants_override=str(Path(constants_override).resolve().relative_to(ROOT)) if constants_override else None,
                  graph_override=str(Path(graph_override).resolve().relative_to(ROOT)) if graph_override else None,
                  linker_override=str(Path(linker_override).resolve().relative_to(ROOT)) if linker_override else None,
                  build_dir=str(build.relative_to(ROOT)), commands=commands)
    result_path.write_text(json.dumps(result,indent=2)+'\n')
    if not correct:
        raise RuntimeError(f'Model numerical mismatch: {result_path}')
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--kind', choices=KINDS)
    parser.add_argument('--force', action='store_true')
    args = parser.parse_args()
    manifest = load_model(DEST)
    rows = json.loads((DEST/'results.json').read_text()) if (DEST/'results.json').exists() else []
    for kind in ([args.kind] if args.kind else KINDS):
        if kind == 'generated-selected':
            candidates = [r for r in rows if r['kind'] in ('generated-0','generated-1','generated-2','generated-3')]
            assert len(candidates) == 4 and all(r['correct'] for r in candidates), 'Measure all four schedules before layer selection'
            schedules = {}
            for layer in manifest['layers']:
                if 'workload' not in layer:
                    continue
                index = layer['index']
                winner = min(candidates, key=lambda r: sum(v['instructions'] for v in r['layers'] if v['layer'] == index and v['sample'] < 2))
                schedules[str(index)] = int(winner['kind'].split('-')[-1])
            selection = dict(schedules=schedules, candidate_keys={r['kind']:r['cache_key'] for r in candidates},
                             method='minimum mean profiled layer instructions on two original images only; four whole-model schedules; recompile and remeasure selected model on originals and two held-out flips')
            (DEST/'selection.json').write_text(json.dumps(selection,indent=2)+'\n')
        result = benchmark(manifest, kind, args.force)
        rows = [r for r in rows if r['kind'] != kind] + [result]
        (DEST/'results.json').write_text(json.dumps(rows,indent=2)+'\n')
        with (DEST/'results.csv').open('w') as f:
            fields = ['kind','sample','instructions','text_bytes','static_ram_bytes','stack_watermark_bytes','mismatches']
            writer = csv.DictWriter(f,fields)
            writer.writeheader()
            for r in rows:
                for s in r['samples']:
                    writer.writerow(dict(kind=r['kind'],sample=s['name'],instructions=s['instructions'],
                                         text_bytes=r['text_bytes'],static_ram_bytes=r['static_ram_bytes'],
                                         stack_watermark_bytes=s['stack_watermark_bytes'],mismatches=s['mismatches']))
        print(kind, result['mean_instructions'], 'instructions; correct=',result['correct'],flush=True)


if __name__ == '__main__':
    main()
