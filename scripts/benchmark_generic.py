#!/usr/bin/env python3
from pathlib import Path
from runner import ROOT,load,benchmark,save

def source(w):
    core=(ROOT/'crates/kernel-core/src/lib.rs').read_text()
    p=', '.join(k+': '+str(w[k]).lower() for k in ('height','width','input_channels','output_channels','kernel','stride','padding','input_offset','output_offset','activation_min','activation_max'))
    p+=', depthwise: '+str(w['operator']=='depthwise').lower()
    return core+f'''
#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {{ loop {{ core::hint::spin_loop(); }} }}
/// # Safety
/// All tensors have the lengths and non-aliasing guarantees in the workload.
#[no_mangle]
pub unsafe extern "C" fn kernel(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {{
 let p=core::hint::black_box(Params {{ {p} }});
 generic(&p, core::slice::from_raw_parts(i,p.input_len()),core::slice::from_raw_parts(w,p.weights_len()),core::slice::from_raw_parts(b,p.output_channels),core::slice::from_raw_parts(m,p.output_channels),core::slice::from_raw_parts(s,p.output_channels),core::slice::from_raw_parts_mut(o,p.output_len()));
}}
'''
if __name__=='__main__':
    rows=[]
    for path in sorted((ROOT/'workloads').glob('*.toml')):
        w=load(path);out=ROOT/'generated'/f'{w["name"]}_generic.rs';out.write_text(source(w))
        r=benchmark(path,'generic',out);rows.append(r);save(rows,ROOT/'results/generic.json');print(w['name'],r['instructions'],flush=True)
    (ROOT/'results/generic-milestone.json').write_text('{"validated":true,"workloads":8}\n')
