#!/usr/bin/env python3
"""Build two genuine pretrained graphs and a host-controlled timing diagnostic."""
import hashlib,json,re,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'scripts'))
from runner import RFLAGS,run
from benchmark_model import wrapper
from resnet_model import graph_source
import muriscv_backend as mb
HERE=Path(__file__).resolve().parent
OUT=HERE/'main/generated';OUT.mkdir(exist_ok=True)
p=json.loads((ROOT/'generated/person-detection/manifest.json').read_text())
cache=ROOT/'results/algorithm/variants/hardware-cache-b2-p4'
pgraph=json.loads((cache/'graph-manifest.json').read_text())
rbase=ROOT/'results/model/cifar10-resnet';r=json.loads((rbase/'manifest.json').read_text())
selected=json.loads((rbase/'selection.json').read_text())['best']['kind'];rtuned=rbase/selected
assert json.loads((rbase/'baseline-milestone.json').read_text())['correct']
backends=['esp_nn','mu_scalar','mu_fixed','rust']
candidates=['tile8_b2p4','tile16_b2p4','tile16_b2p4_sram','tile16_b4p2']
rbackends=backends+candidates
provenance=dict(revision=4,person_model_sha256=p['original_sha256'],resnet_model_sha256=r['model_sha256'],resnet_selection=selected,
                candidates=candidates, sources={str(x.relative_to(ROOT)):hashlib.sha256(x.read_bytes()).hexdigest() for x in [Path(__file__),HERE/'runtime.c',HERE/'selection.py',ROOT/'scripts/resnet_model.py',ROOT/'crates/kernel-generator/src/tiled.rs',cache/'kernels.rs',cache/'constants.h',rtuned/'source.rs',rtuned/'parameters.h',*[rbase/c/f for c in candidates for f in ['source.rs','parameters.h','manifest.json']],rbase/'vectors.h',ROOT/'results/model/person-detection/vectors.h']})
build=hashlib.sha256(json.dumps(provenance,sort_keys=True).encode()).hexdigest()[:16];provenance['build_id']=build
(OUT/'provenance.json').write_text(json.dumps(provenance,indent=2)+'\n')
# Versioned model/fixture hashes and full intermediate byte fingerprints for host audits.
def fnv(data):
 h=2166136261
 for b in data:h=((h^int(b))*16777619)&0xffffffff
 return h

def oracle_json(header,pattern,samples,n):
 out={}
 arrays={name:[int(x)&255 for x in values.split(',')] for name,values in re.findall(r'static const int8_t (\w+)\[\d+\]\s*=\s*\{([^}]*)\};',header)}
 for s in range(samples):
  for l in range(n):
   a=arrays[pattern.format(s=s,l=l)];out[f'{s}:{l}']=dict(bytes=len(a),fnv1a32=fnv(a))
 return out
pvectors=(ROOT/'results/model/person-detection/vectors.h').read_text();rvectors=(rbase/'vectors.h').read_text()
golden=dict(build_id=build,models=[dict(id=0,name='person',sha256=p['original_sha256'],operators=31,samples=p['samples'],tensors=oracle_json(pvectors,'expected_{s}_{l}',4,31)),dict(id=1,name='cifar10_resnet',sha256=r['model_sha256'],operators=16,samples=r['samples'],tensors=oracle_json(rvectors,'r_expected_{s}_{l}',4,16))])
(OUT/'oracle.json').write_text(json.dumps(golden,indent=2)+'\n')
(OUT/'p_original.h').write_text((ROOT/'results/model/person-detection/constants.h').read_text())
(OUT/'p_tuned.h').write_text(re.sub(r'\b(weights(?:_native)?|bias|mult|shift)_(\d+)\b',r'p_t_\1_\2',(cache/'constants.h').read_text()))
(OUT/'p_vectors.h').write_text(pvectors)
(OUT/'constants.h').write_text((rbase/'constants.h').read_text())
(OUT/'vectors.h').write_text(rvectors)
(OUT/'r_tuned.h').write_text(re.sub(r'\br_(weights|bias|mult|shift)_(\d+)\b',r't_\1_\2',(rtuned/'parameters.h').read_text()))
# Every backend uses its own exact parameter layout. Mixed schedules select
# a function AND its parameters, never merely swapping function pointers.
graph=graph_source(r,['r_'+b for b in rbackends],tuned_header='r_tuned.h')
for n,c in enumerate(candidates,4):
 (OUT/f'r_c{n}.h').write_text(re.sub(r'\br_(weights|bias|mult|shift)_(\d+)\b',rf'c{n}_\1_\2',(rbase/c/'parameters.h').read_text()))
 graph='#include "r_c'+str(n)+'.h"\n'+graph
for l in r['layers']:
 j=l['index']
 if 'workload' in l:
  for key in ['weights','bias','mult','shift']:
   expr='(chosen==3?t_'+key+'_'+str(j)+':r_'+key+'_'+str(j)+')'
   for n in range(4,8):expr=f'(chosen=={n}?c{n}_{key}_{j}:'+expr+')'
   graph=graph.replace(f'(backend==7?t_{key}_{j}:r_{key}_{j})',expr)
   if key=='weights':weight_expr=expr
  w=l['workload'];n=w['kernel']**2*w['input_channels']*w['output_channels']
  live=w['height']*w['width']*w['input_channels'] if l['explicit_padding'] else 0
  assert live+n<=38416, 'Whole-weight staging overlaps live padded input'
  call=f'r_kernels[backend][{j}](i,'
  graph=graph.replace(call+weight_expr,f'if(chosen==6)memcpy(padded+{38416-n},{weight_expr},{n});\n'+call+f'(chosen==6?padded+{38416-n}:{weight_expr})')
 graph=graph.replace(f'if(check)record_check(1,backend,sample,{j},',f'profile_end({j});if(check)record_check(1,backend,sample,{j},')
 # The generated graph has one scope per operator. Insert timing before padding.
 pos=graph.index('const int8_t*i=',graph.index('void resnet_execute')) if j==0 else 0
 graph=graph.replace(f'r_kernels[backend][{j}]',f'r_kernels[chosen][{j}]')
# All sixteen scopes have this exact start marker.
parts=graph.split('{\nconst int8_t*i=')
assert len(parts)==17
for j in range(16):parts[j+1]=f'profile_begin({j});unsigned chosen=choose_backend(1,backend,{j});\nconst int8_t*i='+parts[j+1]
graph=parts[0]+''.join('{\n'+v for v in parts[1:])
graph='extern unsigned choose_backend(unsigned,unsigned,unsigned);\nextern void profile_begin(unsigned),profile_end(unsigned);\n'+graph
(OUT/'resnet.c').write_text(graph+'\nconst int8_t*resnet_expected(unsigned s){return r_oracles[s][15];}\n')
rust='#![no_std]\n'
for prefix,source in [('p_rust',(cache/'kernels.rs').read_text()),('r_rust',(rtuned/'source.rs').read_text()),*[(f'r_{c}',(rbase/c/'source.rs').read_text()) for c in candidates]]:
 source=source.split('#[panic_handler]',1)[0].replace('#![no_std]','')
 source=re.sub(r'\bfn layer_(\d+)\(',lambda m:'fn '+prefix+'_layer_'+m[1]+'(',source)
 rust+='mod '+prefix+' {\n'+source+'\n}\n'
rust+='extern "C"{fn hw_rust_panic()->!;}\n#[panic_handler]fn panic(_: &core::panic::PanicInfo)->!{unsafe{hw_rust_panic()}}\n'
(OUT/'kernels.rs').write_text(rust);run(['rustc',*RFLAGS,OUT/'kernels.rs','-o',OUT/'kernels.a'])
tokens=sorted(set(re.findall(r'\bmuriscv_nn_\w+\b','\n'.join(f.read_text() for f in [*(mb.UP/'Include').rglob('*.h'),*(mb.UP/'Source').rglob('*.c')]))))
cmake=''
for name,kind in [('esp_nn','c'),('mu_scalar','muriscv'),('mu_fixed','muriscv-fixed-asm')]:
 parts=['#include <stdint.h>\nextern void hw_backend_error(int);\n']
 for model,prefix in [(p,'p'),(r,'r')]:
  for l in model['layers']:
   if 'workload'not in l:continue
   j=l['index'];w=l['workload'];native=prefix=='p' and j==0 and kind=='c'
   if native:w=dict(w,operator='depthwise')
   src=mb.wrapper(w,kind) if kind!='c' else wrapper(w)
   if native:src=src.replace(f"p={{{w['input_offset']},{w['output_offset']},1,",f"p={{{w['input_offset']},{w['output_offset']},8,")
   src=src.replace('void kernel(',f'void {prefix}_{name}_layer_{j}(')
   src=re.sub(r'if\(status!=MURISCV_NN_SUCCESS\)\{.*?\}\n','if(status!=MURISCV_NN_SUCCESS){hw_backend_error(status);}\n',src,flags=re.S)
   parts.append(src)
 (OUT/(name+'.c')).write_text(''.join(parts))
 if kind!='c':
  nh=OUT/(name+'-names.h');nh.write_text(''.join(f'#define {t} {name}_{t}\n' for t in tokens))
  lib='v4_'+name;files=[f for folder in ['ConvolutionFunctions','FullyConnectedFunctions','NNSupportFunctions'] for f in (mb.UP/'Source'/folder).glob('*.c')]
  cmake+='add_library('+lib+' STATIC '+ ' '.join('"'+str(f)+'"' for f in files)+')\n'
  inc=mb.include_dir(kind);flags=' '.join(mb.defines(kind))
  cmake+=f'target_include_directories({lib} PRIVATE "{inc}" "{mb.UP/"Include"}")\n'
  cmake+=f'target_compile_options({lib} PRIVATE -O3 -fno-lto -fno-tree-vectorize -include "{nh}" {flags})\n'
  cmake+=f'set_source_files_properties("${{CMAKE_CURRENT_LIST_DIR}}/{name}.c" PROPERTIES COMPILE_OPTIONS "-include;{nh};{flags.replace(" ",";")}" INCLUDE_DIRECTORIES "{inc};{mb.UP/"Include"}")\n'
  cmake+=f'target_link_libraries(${{COMPONENT_LIB}} PRIVATE {lib})\n'
esp=ROOT/'baselines/esp-nn'
sources=[esp/'src'/n for n in ['convolution/esp_nn_conv_opt.c','convolution/esp_nn_conv_ansi.c','convolution/esp_nn_depthwise_conv_opt.c','fully_connected/esp_nn_fully_connected_ansi.c']]+[mb.UP/'Source'/n for n in ['SoftmaxFunctions/muriscv_nn_softmax_s8.c','SoftmaxFunctions/muriscv_nn_softmax_common_s8.c','BasicMathFunctions/muriscv_nn_elementwise_add_s8.c']]
cmake+='target_sources(${COMPONENT_LIB} PRIVATE '+' '.join('"'+str(f)+'"' for f in sources)+')\n'
cmake+=f'target_include_directories(${{COMPONENT_LIB}} PRIVATE "{esp/"include"}" "{esp/"src/common"}" "{mb.UP/"Include"}")\n'
(OUT/'baselines.cmake').write_text(cmake)
s='#include <stdint.h>\n#include <string.h>\n#include "p_original.h"\n#include "p_tuned.h"\n#include "p_vectors.h"\nextern int8_t activations[73728],padded[38416];\nextern void record_check(unsigned,unsigned,unsigned,unsigned,const int8_t*,const int8_t*,unsigned);\nextern void muriscv_nn_softmax_s8(const int8_t*,int32_t,int32_t,int32_t,int32_t,int32_t,int8_t*);\n'
for name in backends:
 for l in p['layers']:
  if 'workload'in l:s+=f'extern void p_{name}_layer_{l["index"]}(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);\n'
s+='typedef void(*p_fn)(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);\n'
s+='static const p_fn pk[4][31]={'+','.join('{'+','.join(f'p_{b}_layer_{l["index"]}' if 'workload'in l else '0' for l in p['layers'])+'}' for b in backends)+'};\n'
for pre in ['','p_t_']:
 for key,typ in [('weights','int8_t'),('bias','int32_t'),('mult','int32_t'),('shift','int32_t')]:
  s+=f'static const {typ}*const {pre}{key}_table[]={{'+','.join(f'{pre}{key}_{l["index"]}' if 'workload'in l else '0' for l in p['layers'])+'};\n'
for name,g in [('normal',p),('tuned',pgraph)]:
 vals=[]
 for l in g['layers']:
  if l.get('explicit_padding'):
   w=l['workload'];v=[*l['shape'][1:],w['height'],w['width'],l['pad_before'],l['input_zero']]
  else:v=[0]*7
  vals.append('{'+','.join(map(str,v))+'}')
 s+=f'static const int {name}_pads[31][7]={{'+','.join(vals)+'};\n'
s+='static const unsigned p_lengths[]={'+','.join(str(__import__('math').prod(l['outshape'])) for l in p['layers'])+'};\n'
s+='extern unsigned choose_backend(unsigned,unsigned,unsigned);\nextern void profile_begin(unsigned),profile_end(unsigned);\n'
s+='''void person_execute(unsigned backend,unsigned sample,unsigned check){unsigned requested=backend;const int8_t*i=images[sample];for(unsigned j=0;j<31;j++){profile_begin(j);backend=choose_backend(0,requested,j);int8_t*o=activations+(j&1)*36864;const int*d=(backend==3?tuned_pads:normal_pads)[j];
if(d[0]){for(unsigned y=0;y<(unsigned)d[3];y++)for(unsigned x=0;x<(unsigned)d[4];x++)for(unsigned c=0;c<(unsigned)d[2];c++){unsigned a=y-d[5],b=x-d[5];padded[(y*d[4]+x)*d[2]+c]=(a<(unsigned)d[0]&&b<(unsigned)d[1])?i[(a*d[1]+b)*d[2]+c]:d[6];}i=padded;}
if(pk[backend][j]){const int8_t*w=backend==3?p_t_weights_table[j]:weights_table[j];if(backend==0&&j==0)w=weights_native_0;
pk[backend][j](i,w,backend==3?p_t_bias_table[j]:bias_table[j],backend==3?p_t_mult_table[j]:mult_table[j],backend==3?p_t_shift_table[j]:shift_table[j],o);
}else if(j==27){for(unsigned c=0;c<256;c++){int sum=0;for(unsigned q=0;q<9;q++)sum+=i[q*256+c];o[c]=(sum+(sum>0?4:-4))/9;}}
else if(j==29)memcpy(o,i,2);
'''
l=p['layers'][-1];s+=f'else muriscv_nn_softmax_s8(i,1,2,{l["multiplier"]},{l["shift"]},{l["diff_min"]},o);\n'
s+='profile_end(j);if(check){record_check(0,requested,sample,j,o,oracles[sample][j],p_lengths[j]);}i=o;}}\n'
s+='const int8_t*person_output(void){return activations;}\nconst int8_t*person_expected(unsigned s){return oracles[s][30];}\n'
(OUT/'person.c').write_text(s)
(OUT/'runtime.c').write_text((HERE/'runtime.c').read_text().replace('@BUILD_ID@',build))
print('Prepared v4',build,'resnet',selected)
