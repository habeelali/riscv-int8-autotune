#!/usr/bin/env python3
"""Real deploy-image sizing, fifth-input execution, and independent CSR checks."""
import hashlib
import json
import re
import shutil
import subprocess
from pathlib import Path

import numpy as np
from ai_edge_litert.interpreter import Interpreter, OpResolverType

from benchmark_model import graph_source
from runner import ROOT, CC, CFLAGS, QEMU, QFLAGS, run

DEST = ROOT/'results/flash'


def deployment(row, manifest, label, oracle):
    source = ROOT/row['build_dir']
    build = DEST/'deployment'/label
    build.mkdir(parents=True,exist_ok=True)
    if row.get('constants_override'):
        shutil.copyfile(ROOT/row['constants_override'],build/'constants.h')
    elif (build/'constants.h').exists():
        (build/'constants.h').unlink()
    # Same graph steps and buffers, with no fixture input/oracle arrays linked.
    graph = ((ROOT/row['graph_override']).read_text() if row.get('graph_override') else graph_source(manifest)).split('__attribute__((noinline)) static void empty_step',1)[0]
    graph = graph.replace('#include "vectors.h"\n','')
    final = (len(manifest['layers'])-1)&1
    graph += f'''
static int8_t model_input[9216];
__attribute__((noinline)) static void model(const int8_t*i){{
for(unsigned j=0;j<31;j++){{int8_t*o=arena[j&1];steps[j](i,o);i=o;}}}}
__attribute__((noinline)) static uint32_t deployment_measure(const int8_t*i){{
uint32_t a,b;asm volatile("rdinstret %0":"=r"(a)::"memory");model(i);asm volatile("rdinstret %0":"=r"(b)::"memory");return b-a;}}
int main(void){{uint32_t count=deployment_measure(model_input);
puts_uart("DEPLOY ");number(count);putc_uart(' ');signed_number(arena[{final}][0]);putc_uart(' ');signed_number(arena[{final}][1]);putc_uart(10);return 0;}}
'''
    (build/'graph.c').write_text(graph)
    # Explicitly collect small read-only data and align the BSS section start,
    # instead of charging address-dependent alignment padding to static RAM.
    linker = (ROOT/'qemu/link.ld').read_text()
    linker = linker.replace('*(.rodata .rodata.*)', '*(.rodata .rodata.* .srodata .srodata.*)')
    linker = linker.replace('.bss (NOLOAD)', '.bss ALIGN(16) (NOLOAD)')
    (build/'link.ld').write_text(linker)
    commands = []

    def execute(cmd,**kwargs):
        commands.append(list(map(str,cmd)))
        return run(cmd,**kwargs)

    # Reuse exact tested memory/start/softmax/kernel object files, change harness only.
    obj = build/'graph.o'
    execute([CC,*CFLAGS,'-march=rv32imc_zicsr','-I'+str(ROOT/'results/model/person-detection'),
             '-c',build/'graph.c','-o',obj])
    elf = build/'model.elf'
    execute([CC,*CFLAGS,'-nostdlib','-Wl,--gc-sections','-Wl,--no-relax',
             '-Wl,-Map='+str(build/'link.map'),'-T',build/'link.ld',obj,
             *[source/f'{i}.o' for i in range(1,5)],source/'kernels.a','-lgcc','-o',elf])
    size = execute(['riscv64-unknown-elf-size','-A',elf])
    (build/'size.txt').write_text(size)
    sections = {m[0]:int(m[1]) for m in re.findall(r'^(\.\S+)\s+(\d+)\s+\d+',size,re.M)}
    assert not any(s.startswith('.srodata') for s in sections)
    (build/'assembly.txt').write_text(execute(['riscv64-unknown-elf-objdump','-d',elf]))
    symbols = execute(['riscv64-unknown-elf-nm','-S',elf])
    (build/'symbols.txt').write_text(symbols)
    assert not re.search(r'\b(expected_\d|image_\d|oracles)\b',symbols)
    raw = []
    for repeat in range(3):
        output = execute([QEMU,*QFLAGS,'-kernel',elf],timeout=120)
        (build/f'qemu-{repeat}.txt').write_text(output)
        match = re.search(r'DEPLOY (\d+) (-?\d+) (-?\d+)',output)
        assert match,output
        values = list(map(int,match.groups()))
        assert values[1:] == oracle,(values,oracle)
        raw.append(values)
    assert raw[0] == raw[1] == raw[2]
    execute(['riscv64-unknown-elf-objcopy','-O','binary',elf,build/'model.bin'])
    result = dict(label=label,correct=True,output=oracle,instructions_raw=raw[0][0],
                  text_bytes=sections.get('.text',0),rodata_bytes=sections.get('.rodata',0),
                  data_bytes=sections.get('.data',0),
                  flash_bytes=sum(sections.get(s,0) for s in ('.text','.rodata','.data')),
                  binary_file_bytes=(build/'model.bin').stat().st_size,
                  static_ram_bytes=sections.get('.data',0)+sections.get('.bss',0),
                  reserved_stack_bytes=65536,kernel_benchmark_key=row['cache_key'],
                  elf_sha256=hashlib.sha256(elf.read_bytes()).hexdigest(),
                  build_dir=str(build.relative_to(ROOT)),commands=commands,samples=raw)
    (build/'result.json').write_text(json.dumps(result,indent=2)+'\n')
    return result


def plugin_check(row):
    build = ROOT/row['build_dir']
    assembly = (build/'assembly.txt').read_text()
    regions = re.findall(r'^[0-9a-f]+ <measure_model[^>]*>:\n(.*?)(?=\n\n)',assembly,re.M|re.S)
    region = [r for r in regions if re.search(r'<model>',r)]
    assert len(region) == 1
    pcs = re.findall(r'^\s*([0-9a-f]+):.*\brdinstret\b',region[0],re.M)
    assert len(pcs) == 2
    command = [QEMU,*QFLAGS,'-plugin',f'{ROOT}/qemu/count-plugin.so,0x{pcs[0]},0x{pcs[1]}',
               '-kernel',str(build/'model.elf')]
    p = subprocess.run(command,capture_output=True,text=True,timeout=300)
    assert p.returncode == 0,p.stderr
    (build/'flash-plugin-stdout.txt').write_text(p.stdout)
    (build/'flash-plugin-stderr.txt').write_text(p.stderr)
    actual = list(map(int,re.findall(r'PLUGIN_REGION (\d+)',p.stderr)))
    expected = [s['raw_instructions'] for s in row['samples']]
    assert actual == expected,(actual,expected)
    return dict(kind=row['kind'],cache_key=row['cache_key'],plugin_counts=actual,csr_counts=expected,match=True,command=command)


def main():
    best = json.loads((DEST/'best.json').read_text())
    manifest = json.loads((DEST/'reference/manifest.json').read_text())
    compatible = ROOT/'results/model/person-detection/interpreter-compatible.tflite'
    oracle = Interpreter(model_path=str(compatible),experimental_op_resolver_type=OpResolverType.BUILTIN_REF,num_threads=1)
    oracle.allocate_tensors()
    oracle.set_tensor(oracle.get_input_details()[0]['index'],np.zeros((1,96,96,1),dtype=np.int8))
    oracle.invoke()
    expected = oracle.get_tensor(oracle.get_output_details()[0]['index']).reshape(-1).tolist()
    before = deployment(best['reference'],manifest,'before',expected)
    after = deployment(best['best'],manifest,'after',expected)
    assert after['flash_bytes'] < before['flash_bytes']
    assert after['instructions_raw'] <= before['instructions_raw']
    assert after['static_ram_bytes'] <= before['static_ram_bytes']
    checks = [plugin_check(best[key]) for key in ('reference','best')]
    result = dict(before=before,after=after,counter_crosscheck=checks,
                  held_out_input='96x96 all-zero quantized image, unused by schedule or flash selection',
                  no_test_images_or_oracles_linked=True)
    (DEST/'deployment-comparison.json').write_text(json.dumps(result,indent=2)+'\n')
    # Keep source + logs in lean archives; full ELF and binary remain in deployment/.
    for row in (best['reference'],best['best']):
        src=ROOT/row['build_dir'];out=DEST/'evidence'/row['kind'];out.mkdir(parents=True,exist_ok=True)
        for name in ('qemu-0.txt','qemu-1.txt','qemu-2.txt','assembly.txt','size.txt','graph.c','kernels.rs',
                     'flash-plugin-stdout.txt','flash-plugin-stderr.txt'):
            shutil.copyfile(src/name,out/name)
    print('Deployment flash:',before['flash_bytes'],'->',after['flash_bytes'],'bytes;',
          'instructions:',before['instructions_raw'],'->',after['instructions_raw'],flush=True)


if __name__ == '__main__':
    main()
