#!/usr/bin/env python3
"""Create a real ESP-IDF target from the measured, versioned model artifacts."""
import argparse
import hashlib
import json
import re
import shutil
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'scripts'))
from benchmark_model import wrapper
import muriscv_backend as mb
from runner import run,RFLAGS

parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--cache-candidates',action='store_true')
args=parser.parse_args()

OUT=Path(__file__).resolve().parent/'main/generated'
OUT.mkdir(parents=True,exist_ok=True)
original=json.loads((ROOT/'generated/person-detection/manifest.json').read_text())
selection=json.loads((ROOT/'results/algorithm/best.json').read_text())['best']
candidate=ROOT/selection['source']
algorithm=json.loads((candidate.parent/'manifest.json').read_text())
algo_graph=json.loads((candidate.parent/'graph-manifest.json').read_text())
old_source=(ROOT/'generated/person-detection/flash/kernels.rs').read_text()
new_source=candidate.read_text()
base_header=(ROOT/'results/model/person-detection/constants.h').read_text()
new_header=(candidate.parent/'constants.h').read_text()
vectors=(ROOT/'results/model/person-detection/vectors.h').read_text()
provenance=dict(idf='v5.4.2',model_sha256=original['original_sha256'],algorithm=selection['variant'],
                qemu_before_instructions=json.loads((ROOT/'results/flash/best.json').read_text())['best']['mean_instructions'],
                qemu_candidate_instructions=selection['mean_instructions'],
                sources={name:hashlib.sha256(value.encode()).hexdigest() for name,value in
                         [('compact.rs',old_source),('algorithm.rs',new_source),('original-parameters',base_header),('algorithm-parameters',new_header)]})
extras=[]
if args.cache_candidates:
    extra_rows=json.loads((ROOT/'results/hardware-cache/candidates.json').read_text())
    for prefix,row in zip(('cache4','cache2'),extra_rows):
        path=ROOT/row['source']
        extras.append((prefix,path))
        provenance['sources'][prefix+'.rs']=hashlib.sha256(path.read_bytes()).hexdigest()
        provenance['sources'][prefix+'-parameters']=hashlib.sha256((path.parent/'constants.h').read_bytes()).hexdigest()
    provenance['diagnostic_revision']=2
build_id=hashlib.sha256(json.dumps(provenance,sort_keys=True).encode()).hexdigest()[:16]
provenance['build_id']=build_id
(OUT/'provenance.json').write_text(json.dumps(provenance,indent=2)+'\n')

def rename_parameters(text):
    return re.sub(r'\b(weights(?:_native)?|bias|mult|shift)_(\d+)\b',r'alg_\1_\2',text)

(OUT/'original.h').write_text(base_header)
(OUT/'algorithm.h').write_text(rename_parameters(new_header))
(OUT/'vectors.h').write_text(vectors)
rust='#![no_std]\n'
for name,source in [('compact',old_source),('algorithm',new_source)]+[(n,p.read_text()) for n,p in extras]:
    source=source.split('#[panic_handler]',1)[0].replace('#![no_std]','')
    source=re.sub(r'\bfn layer_(\d+)\(',lambda m:f'fn {name}_layer_{m[1]}(',source)
    rust+=f'mod {name} {{\n{source}\n}}\n'
rust+='extern "C" { fn hw_rust_panic() -> !; }\n#[panic_handler] fn panic(_: &core::panic::PanicInfo) -> ! { unsafe { hw_rust_panic() } }\n'
(OUT/'kernels.rs').write_text(rust)
run(['rustc',*RFLAGS,OUT/'kernels.rs','-o',OUT/'kernels.a'],cwd=ROOT)

# C backends use the exact wrappers and unmodified upstream implementations.
backends=['esp_nn_native','mu_scalar','mu_portable','mu_fixed','mu_portable_fixed','rust_compact','rust_candidate']+['rust_'+n for n,p in extras]
backend_count=len(backends)
layer_records=backend_count*4*31
run_records=backend_count*4*7
prefixes={'rust_compact':'compact','rust_candidate':'algorithm',**{'rust_'+n:n for n,p in extras}}
for n,p in extras:
    header=(p.parent/'constants.h').read_text()
    header=re.sub(r'\b(weights(?:_native)?|bias|mult|shift)_(\d+)\b',lambda m:n+'_'+m[1]+'_'+m[2],header)
    (OUT/(n+'.h')).write_text(header)
    g=json.loads((p.parent/'graph-manifest.json').read_text())
    assert g==algo_graph,'cache variants must share candidate graph'

kinds=['c','muriscv','muriscv-portable','muriscv-fixed-asm','muriscv-portable-fixed-asm']
tokens=sorted(set(re.findall(r'\bmuriscv_nn_\w+\b','\n'.join(p.read_text() for p in (mb.UP/'Include').glob('*.h')))))
mu_files=sorted(p for folder in ('ConvolutionFunctions','FullyConnectedFunctions','NNSupportFunctions') for p in (mb.UP/'Source'/folder).glob('*.c'))
cmake=''
for name,kind in zip(backends,kinds):
    parts=['#include <stdint.h>\n#include <stdlib.h>\nextern void hw_backend_error(int);\n']
    for layer in original['layers']:
        if 'workload' not in layer:continue
        j=layer['index'];w=layer['workload'];native=name=='esp_nn_native' and j==0
        if native:w=dict(w,operator='depthwise')
        source=mb.wrapper(w,kind) if kind.startswith('muriscv') else wrapper(w)
        if native:source=source.replace(f"p={{{w['input_offset']},{w['output_offset']},1,",f"p={{{w['input_offset']},{w['output_offset']},8,")
        source=source.replace('void kernel(',f'void {name}_layer_{j}(')
        source=re.sub(r'if\(status!=MURISCV_NN_SUCCESS\)\{.*?\}\n',
                      'if(status!=MURISCV_NN_SUCCESS){ hw_backend_error(status); }\n',source,flags=re.S)
        parts.append(source)
    (OUT/(name+'.c')).write_text(''.join(parts))
    if kind.startswith('muriscv'):
        header=OUT/(name+'-names.h')
        header.write_text(''.join(f'#define {token} {name}_{token}\n' for token in tokens))
        overlay=mb.include_dir(kind)
        lib='hw_'+name
        cmake+=f'add_library({lib} STATIC\n'+''.join(f' "{p}"\n' for p in mu_files)+')\n'
        cmake+=f'target_include_directories({lib} PRIVATE "{overlay}" "{mb.UP / "Include"}")\n'
        opts=' '.join(mb.defines(kind))
        cmake+=f'target_compile_options({lib} PRIVATE -O3 -fno-lto -fno-tree-vectorize -include "{header}" {opts})\n'
        cmake+=f'set_source_files_properties("${{CMAKE_CURRENT_LIST_DIR}}/{name}.c" PROPERTIES COMPILE_OPTIONS "-include;{header};{opts.replace(" ",";")}" INCLUDE_DIRECTORIES "{overlay};{mb.UP / "Include"}")\n'
        cmake+=f'target_link_libraries(${{COMPONENT_LIB}} PRIVATE {lib})\n'
esp=ROOT/'baselines/esp-nn'
sources=[esp/'src/convolution'/n for n in ('esp_nn_conv_opt.c','esp_nn_conv_ansi.c','esp_nn_depthwise_conv_opt.c')]
soft=[mb.UP/'Source/SoftmaxFunctions'/n for n in ('muriscv_nn_softmax_s8.c','muriscv_nn_softmax_common_s8.c')]
cmake+='target_sources(${COMPONENT_LIB} PRIVATE '+ ' '.join('"'+str(p)+'"' for p in [*sources,*soft])+')\n'
cmake+=f'target_include_directories(${{COMPONENT_LIB}} PRIVATE "{esp / "include"}" "{esp / "src/common"}" "{mb.UP / "Include"}")\n'
(OUT/'baselines.cmake').write_text(cmake)

driver='''#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <inttypes.h>
#include "sdkconfig.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "esp_chip_info.h"
#include "esp_system.h"
#include "esp_cpu.h"
#include "esp_timer.h"
#include "esp_random.h"
#include "esp_attr.h"
#include "esp_heap_caps.h"
#include "esp_flash.h"
#include "esp_private/esp_clk.h"
#include "original.h"
#include "algorithm.h"
#include "vectors.h"
typedef void (*kernel_fn)(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);
static int8_t arena[2][36864],padded[38416];
extern void muriscv_nn_softmax_s8(const int8_t*,int32_t,int32_t,int32_t,int32_t,int32_t,int8_t*);
RTC_NOINIT_ATTR static uint32_t capture_boot_counter;
static unsigned errors=0,current_backend=0,current_layer=0;
static inline uint32_t ticks(void){asm volatile("":::"memory");uint32_t c=esp_cpu_get_cycle_count();asm volatile("":::"memory");return c;}
void hw_backend_error(int status){printf("HW_ERROR {\\"backend\\":%u,\\"layer\\":%u,\\"status\\":%d}\\n",current_backend,current_layer,status);fflush(stdout);abort();}
void hw_rust_panic(void){printf("HW_ERROR {\\"backend\\":%u,\\"layer\\":%u,\\"rust_panic\\":true}\\n",current_backend,current_layer);fflush(stdout);abort();}
'''
for index,name in enumerate(backends):
    prefix=prefixes.get(name,name)
    for layer in original['layers']:
        if 'workload' in layer:
            j=layer['index'];driver+=f'extern void {prefix}_layer_{j}(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);\n'
driver+=f'static const kernel_fn kernels[{backend_count}][31]={{\n'
for name in backends:
    prefix=prefixes.get(name,name)
    driver+='{'+','.join(f'{prefix}_layer_{l["index"]}' if 'workload' in l else '0' for l in original['layers'])+'},\n'
driver+='};\nstatic const char*names[]={'+','.join('"'+n+'"' for n in backends)+'};\n'
for prefix in ('','alg_',*[n+'_' for n,p in extras]):
    for key,typ in [('weights','int8_t'),('bias','int32_t'),('mult','int32_t'),('shift','int32_t')]:
        refs=[]
        for layer in original['layers']:
            if 'workload' not in layer:refs.append('0');continue
            j=layer['index'];refs.append(f'{prefix}{key}_{j}')
        driver+=f'static const {typ} *const {prefix}{key}_table[]={{'+','.join(refs)+'};\n'
for key,typ in [('weights','int8_t'),('bias','int32_t'),('mult','int32_t'),('shift','int32_t')]:
    refs=[key+'_table']*6+['alg_'+key+'_table']+[n+'_'+key+'_table' for n,p in extras]
    driver+=f'static const {typ} *const *const {key}_backends[]={{'+','.join(refs)+'};\n'
driver+='static const unsigned lengths[]={'+','.join(str(int(__import__('math').prod(l['outshape']))) for l in original['layers'])+'};\n'
# Padding descriptors: h,w,channels,padded_h,padded_w,pad_before,zero_point.
for name,graph in [('normal',original),('candidate',algo_graph)]:
    driver+=f'static const int {name}_padding[31][7]={{\n'
    for l in graph['layers']:
        if l.get('explicit_padding'):
            w=l['workload'];vals=[*l['shape'][1:],w['height'],w['width'],l['pad_before'],l['input_zero']]
        else:vals=[0]*7
        driver+='{'+','.join(map(str,vals))+'},\n'
    driver+='};\n'
pool=next(l for l in original['layers'] if l['op']=='AVERAGE_POOL_2D')
sm=original['layers'][-1]
driver+=f'''
static void step(unsigned backend,unsigned layer,const int8_t*i,int8_t*o){{
const int *d=(backend>=6?candidate_padding:normal_padding)[layer];
if(d[0]){{for(unsigned y=0;y<(unsigned)d[3];y++)for(unsigned x=0;x<(unsigned)d[4];x++)for(unsigned c=0;c<(unsigned)d[2];c++){{
unsigned a=y-d[5],b=x-d[5];padded[(y*d[4]+x)*d[2]+c]=(a<(unsigned)d[0]&&b<(unsigned)d[1])?i[(a*d[1]+b)*d[2]+c]:d[6];}}i=padded;}}
if(kernels[backend][layer]){{
const int8_t*w=weights_backends[backend][layer];
if(backend==0&&layer==0)w=weights_native_0;
kernels[backend][layer](i,w,bias_backends[backend][layer],mult_backends[backend][layer],shift_backends[backend][layer],o);
}}else if(layer==27){{for(unsigned c=0;c<256;c++){{int sum=0;for(unsigned p=0;p<9;p++)sum+=i[p*256+c];o[c]=(sum+(sum>0?4:-4))/9;}}}}
else if(layer==29)memcpy(o,i,2);
else muriscv_nn_softmax_s8(i,1,2,{sm['multiplier']},{sm['shift']},{sm['diff_min']},o);
}}
__attribute__((noinline)) static void model(unsigned backend,unsigned sample){{
const int8_t*i=images[sample];for(unsigned j=0;j<31;j++){{int8_t*o=arena[j&1];current_layer=j;step(backend,j,i,o);i=o;}}}}
static unsigned final_check(unsigned sample){{return (arena[0][0]!=oracles[sample][30][0])+(arena[0][1]!=oracles[sample][30][1]);}}
static void benchmark_task(void*unused){{
vTaskDelay(pdMS_TO_TICKS(8000));
if(esp_reset_reason()==ESP_RST_POWERON)capture_boot_counter=0;
capture_boot_counter++;
printf("HW_CAPTURE {{\\"boot_counter\\":%u,\\"nonce_hi\\":%u,\\"nonce_lo\\":%u}}\\n",(unsigned)capture_boot_counter,(unsigned)esp_random(),(unsigned)esp_random());
esp_chip_info_t chip;esp_chip_info(&chip);uint32_t flash=0;esp_flash_get_size(NULL,&flash);
printf("HW_BEGIN {{\\"schema\\":1,\\"build_id\\":\\"{build_id}\\",\\"idf\\":\\"%s\\",\\"chip_model\\":%d,\\"revision\\":%d,\\"cpu_hz\\":%d,\\"flash_bytes\\":%"PRIu32",\\"reset_reason\\":%d,\\"free_heap\\":%u,\\"worker_stack_bytes\\":16384,\\"interrupts_enabled\\":true,\\"model_sha256\\":\\"{original['original_sha256']}\\"}}\\n",esp_get_idf_version(),chip.model,chip.revision,esp_clk_cpu_freq(),flash,esp_reset_reason(),(unsigned)esp_get_free_heap_size());
printf("HW_INFO {{\\"backends\\":{backend_count},\\"samples\\":4,\\"operators\\":31,\\"repeats\\":7,\\"check\\":\\"full_intermediate_tensor_bytes\\",\\"timing\\":\\"hardware_mcycle_with_interrupts\\",\\"console\\":\\"UART0_and_USB_serial_JTAG_115200\\",\\"static_tensor_ram_bytes\\":112144}}\\n");
fflush(stdout);
for(unsigned b=0;b<{backend_count};b++)for(unsigned sample=0;sample<4;sample++){{
current_backend=b;const int8_t*i=images[sample];
for(unsigned j=0;j<31;j++){{current_layer=j;int8_t*o=arena[j&1];uint32_t a=ticks();step(b,j,i,o);uint32_t cycles=ticks()-a;
unsigned wrong=0,first=0;for(unsigned n=0;n<lengths[j];n++)if(o[n]!=oracles[sample][j][n]){{if(!wrong)first=n;wrong++;}}
errors+=wrong;
printf("HW_LAYER {{\\"backend\\":\\"%s\\",\\"sample\\":%u,\\"layer\\":%u,\\"cycles_raw\\":%"PRIu32",\\"mismatches\\":%u,\\"first_mismatch\\":%u}}\\n",names[b],sample,j,cycles,wrong,first);
fflush(stdout);i=o;vTaskDelay(1);
}}}}
for(unsigned sample=0;sample<4;sample++)for(unsigned round=0;round<7;round++)for(unsigned k=0;k<{backend_count};k++){{
unsigned b=(k+round)%{backend_count};current_backend=b;current_layer=99;
model(b,sample);errors+=final_check(sample);vTaskDelay(1);
int64_t t0=esp_timer_get_time();uint32_t c0=ticks();model(b,sample);uint32_t cycles=ticks()-c0;int64_t us=esp_timer_get_time()-t0;
unsigned wrong=final_check(sample);errors+=wrong;
printf("HW_RUN {{\\"backend\\":\\"%s\\",\\"sample\\":%u,\\"round\\":%u,\\"cycles_raw\\":%"PRIu32",\\"wall_us\\":%"PRId64",\\"mismatches\\":%u,\\"output\\":[%d,%d],\\"free_heap\\":%u,\\"worker_min_free_stack_bytes\\":%u}}\\n",names[b],sample,round,cycles,us,wrong,arena[0][0],arena[0][1],(unsigned)esp_get_free_heap_size(),(unsigned)uxTaskGetStackHighWaterMark(NULL));
fflush(stdout);vTaskDelay(1);
}}
printf("HW_DONE {{\\"build_id\\":\\"{build_id}\\",\\"correct\\":%s,\\"mismatches_total\\":%u,\\"layer_records\\":{layer_records},\\"run_records\\":{run_records},\\"free_heap\\":%u,\\"minimum_free_heap\\":%u,\\"worker_min_free_stack_bytes\\":%u}}\\n",errors?"false":"true",errors,(unsigned)esp_get_free_heap_size(),(unsigned)esp_get_minimum_free_heap_size(),(unsigned)uxTaskGetStackHighWaterMark(NULL));fflush(stdout);
for(;;)vTaskDelay(pdMS_TO_TICKS(1000));
}}
void app_main(void){{if(xTaskCreate(benchmark_task,"int8_bench",16384,NULL,5,NULL)!=pdPASS){{printf("HW_ERROR {{\\"task_allocation_failed\\":true}}\\n");abort();}}}}
'''
driver=driver.replace('#include \"vectors.h\"', '#include \"vectors.h\"\n'+''.join('#include \"'+n+'.h\"\n' for n,p in extras))
(OUT/'hardware.c').write_text(driver)
print('Prepared ESP32-C3 image sources:',build_id)
