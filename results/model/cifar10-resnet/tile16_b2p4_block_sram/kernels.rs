#![no_std]
mod layer_0 {

// Scratch: 486 bytes on stack, plus compiler spills.
// Weights [OC/B, K*K*IC, B]; bias includes original input_offset*sum(weights).
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


#[inline(never)] pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=3072 && weights.len()>=432 && output.len()>=16384);
assert!(bias.len()>=16 && mult.len()>=16 && shift.len()>=16);
// Quantization ranges and accumulator bound are proven by the model packer.
let mut patch=[0i8;432];
let mut local_weights=[0i8;54];
for base in (0..1024).step_by(16) { let count=core::cmp::min(16,1024-base);
for (p,dst) in patch[..count*27].chunks_exact_mut(27).enumerate() { let y=(base+p)/32;let x=(base+p)%32;
for ky in 0..3 {for kx in 0..3 {let iy=(y*1+ky) as isize-1;let ix=(x*1+kx) as isize-1;let target=&mut dst[(ky*3+kx)*3..(ky*3+kx+1)*3];
if iy>=0 && iy<32 && ix>=0 && ix<32 {let q=(iy as usize*32+ix as usize)*3;target.copy_from_slice(&input[q..q+3]);} else {target.fill(-128);} } } }
for c in (0..16).step_by(2) {let wb=&weights[c*27..(c+2)*27];
local_weights.copy_from_slice(wb);let wb=&local_weights[..];
for p in (0..count/4*4).step_by(4) {
let v0=&patch[(p+0)*27..(p+0+1)*27];
let v1=&patch[(p+1)*27..(p+1+1)*27];
let v2=&patch[(p+2)*27..(p+2+1)*27];
let v3=&patch[(p+3)*27..(p+3+1)*27];
let mut a0_0=bias[c+0];
let mut a1_0=bias[c+0];
let mut a2_0=bias[c+0];
let mut a3_0=bias[c+0];
let mut a0_1=bias[c+1];
let mut a1_1=bias[c+1];
let mut a2_1=bias[c+1];
let mut a3_1=bias[c+1];
for ((((&v0,&v1),&v2),&v3),ws) in v0.iter().zip(v1.iter()).zip(v2.iter()).zip(v3.iter()).zip(wb.chunks_exact(2)) {
let v0=v0 as i32;
let v1=v1 as i32;
let v2=v2 as i32;
let v3=v3 as i32;
let w0=ws[0] as i32;
a0_0+=v0*w0;
a1_0+=v1*w0;
a2_0+=v2*w0;
a3_0+=v3*w0;
let w1=ws[1] as i32;
a0_1+=v0*w1;
a1_1+=v1*w1;
a2_1+=v2*w1;
a3_1+=v3*w1;
}
output[(base+p+0)*16+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(base+p+1)*16+c+0]=(requantize(a1_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(base+p+2)*16+c+0]=(requantize(a2_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(base+p+3)*16+c+0]=(requantize(a3_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(base+p+0)*16+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(base+p+1)*16+c+1]=(requantize(a1_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(base+p+2)*16+c+1]=(requantize(a2_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(base+p+3)*16+c+1]=(requantize(a3_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
}
for p in (count/4*4..count).step_by(1) {
let v0=&patch[(p+0)*27..(p+0+1)*27];
let mut a0_0=bias[c+0];
let mut a0_1=bias[c+1];
for (&v0,ws) in v0.iter().zip(wb.chunks_exact(2)) {
let v0=v0 as i32;
let w0=ws[0] as i32;
a0_0+=v0*w0;
let w1=ws[1] as i32;
a0_1+=v0*w1;
}
output[(base+p+0)*16+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(base+p+0)*16+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
}
}}}
/// # Safety
/// Parameters must satisfy the bundled host-validated quantization and accumulator-bound manifest.
/// Pointers must have the generated lengths, not alias, and use the packed weights and folded bias.
#[no_mangle]pub unsafe extern "C" fn r_kernel_layer_0(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8){
run(core::slice::from_raw_parts(i,3072),core::slice::from_raw_parts(w,432),core::slice::from_raw_parts(b,16),core::slice::from_raw_parts(m,16),core::slice::from_raw_parts(s,16),core::slice::from_raw_parts_mut(o,16384));}

}
mod layer_1 {

// Scratch: 2592 bytes on stack, plus compiler spills.
// Weights [OC/B, K*K*IC, B]; bias includes original input_offset*sum(weights).
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


#[inline(never)] pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=16384 && weights.len()>=2304 && output.len()>=16384);
assert!(bias.len()>=16 && mult.len()>=16 && shift.len()>=16);
// Quantization ranges and accumulator bound are proven by the model packer.
let mut patch=[0i8;2304];
let mut local_weights=[0i8;288];
for base in (0..1024).step_by(16) { let count=core::cmp::min(16,1024-base);
for (p,dst) in patch[..count*144].chunks_exact_mut(144).enumerate() { let y=(base+p)/32;let x=(base+p)%32;
for ky in 0..3 {for kx in 0..3 {let iy=(y*1+ky) as isize-1;let ix=(x*1+kx) as isize-1;let target=&mut dst[(ky*3+kx)*16..(ky*3+kx+1)*16];
if iy>=0 && iy<32 && ix>=0 && ix<32 {let q=(iy as usize*32+ix as usize)*16;target.copy_from_slice(&input[q..q+16]);} else {target.fill(-128);} } } }
for c in (0..16).step_by(2) {let wb=&weights[c*144..(c+2)*144];
local_weights.copy_from_slice(wb);let wb=&local_weights[..];
for p in (0..count/4*4).step_by(4) {
let v0=&patch[(p+0)*144..(p+0+1)*144];
let v1=&patch[(p+1)*144..(p+1+1)*144];
let v2=&patch[(p+2)*144..(p+2+1)*144];
let v3=&patch[(p+3)*144..(p+3+1)*144];
let mut a0_0=bias[c+0];
let mut a1_0=bias[c+0];
let mut a2_0=bias[c+0];
let mut a3_0=bias[c+0];
let mut a0_1=bias[c+1];
let mut a1_1=bias[c+1];
let mut a2_1=bias[c+1];
let mut a3_1=bias[c+1];
for ((((&v0,&v1),&v2),&v3),ws) in v0.iter().zip(v1.iter()).zip(v2.iter()).zip(v3.iter()).zip(wb.chunks_exact(2)) {
let v0=v0 as i32;
let v1=v1 as i32;
let v2=v2 as i32;
let v3=v3 as i32;
let w0=ws[0] as i32;
a0_0+=v0*w0;
a1_0+=v1*w0;
a2_0+=v2*w0;
a3_0+=v3*w0;
let w1=ws[1] as i32;
a0_1+=v0*w1;
a1_1+=v1*w1;
a2_1+=v2*w1;
a3_1+=v3*w1;
}
output[(base+p+0)*16+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(base+p+1)*16+c+0]=(requantize(a1_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(base+p+2)*16+c+0]=(requantize(a2_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(base+p+3)*16+c+0]=(requantize(a3_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(base+p+0)*16+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(base+p+1)*16+c+1]=(requantize(a1_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(base+p+2)*16+c+1]=(requantize(a2_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(base+p+3)*16+c+1]=(requantize(a3_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
}
for p in (count/4*4..count).step_by(1) {
let v0=&patch[(p+0)*144..(p+0+1)*144];
let mut a0_0=bias[c+0];
let mut a0_1=bias[c+1];
for (&v0,ws) in v0.iter().zip(wb.chunks_exact(2)) {
let v0=v0 as i32;
let w0=ws[0] as i32;
a0_0+=v0*w0;
let w1=ws[1] as i32;
a0_1+=v0*w1;
}
output[(base+p+0)*16+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(base+p+0)*16+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
}
}}}
/// # Safety
/// Parameters must satisfy the bundled host-validated quantization and accumulator-bound manifest.
/// Pointers must have the generated lengths, not alias, and use the packed weights and folded bias.
#[no_mangle]pub unsafe extern "C" fn r_kernel_layer_1(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8){
run(core::slice::from_raw_parts(i,16384),core::slice::from_raw_parts(w,2304),core::slice::from_raw_parts(b,16),core::slice::from_raw_parts(m,16),core::slice::from_raw_parts(s,16),core::slice::from_raw_parts_mut(o,16384));}

}
mod layer_2 {

// Scratch: 2592 bytes on stack, plus compiler spills.
// Weights [OC/B, K*K*IC, B]; bias includes original input_offset*sum(weights).
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


#[inline(never)] pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=16384 && weights.len()>=2304 && output.len()>=16384);
assert!(bias.len()>=16 && mult.len()>=16 && shift.len()>=16);
// Quantization ranges and accumulator bound are proven by the model packer.
let mut patch=[0i8;2304];
let mut local_weights=[0i8;288];
for base in (0..1024).step_by(16) { let count=core::cmp::min(16,1024-base);
for (p,dst) in patch[..count*144].chunks_exact_mut(144).enumerate() { let y=(base+p)/32;let x=(base+p)%32;
for ky in 0..3 {for kx in 0..3 {let iy=(y*1+ky) as isize-1;let ix=(x*1+kx) as isize-1;let target=&mut dst[(ky*3+kx)*16..(ky*3+kx+1)*16];
if iy>=0 && iy<32 && ix>=0 && ix<32 {let q=(iy as usize*32+ix as usize)*16;target.copy_from_slice(&input[q..q+16]);} else {target.fill(-128);} } } }
for c in (0..16).step_by(2) {let wb=&weights[c*144..(c+2)*144];
local_weights.copy_from_slice(wb);let wb=&local_weights[..];
for p in (0..count/4*4).step_by(4) {
let v0=&patch[(p+0)*144..(p+0+1)*144];
let v1=&patch[(p+1)*144..(p+1+1)*144];
let v2=&patch[(p+2)*144..(p+2+1)*144];
let v3=&patch[(p+3)*144..(p+3+1)*144];
let mut a0_0=bias[c+0];
let mut a1_0=bias[c+0];
let mut a2_0=bias[c+0];
let mut a3_0=bias[c+0];
let mut a0_1=bias[c+1];
let mut a1_1=bias[c+1];
let mut a2_1=bias[c+1];
let mut a3_1=bias[c+1];
for ((((&v0,&v1),&v2),&v3),ws) in v0.iter().zip(v1.iter()).zip(v2.iter()).zip(v3.iter()).zip(wb.chunks_exact(2)) {
let v0=v0 as i32;
let v1=v1 as i32;
let v2=v2 as i32;
let v3=v3 as i32;
let w0=ws[0] as i32;
a0_0+=v0*w0;
a1_0+=v1*w0;
a2_0+=v2*w0;
a3_0+=v3*w0;
let w1=ws[1] as i32;
a0_1+=v0*w1;
a1_1+=v1*w1;
a2_1+=v2*w1;
a3_1+=v3*w1;
}
output[(base+p+0)*16+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+4).clamp(-128,127) as i8;
output[(base+p+1)*16+c+0]=(requantize(a1_0,mult[c+0],shift[c+0])+4).clamp(-128,127) as i8;
output[(base+p+2)*16+c+0]=(requantize(a2_0,mult[c+0],shift[c+0])+4).clamp(-128,127) as i8;
output[(base+p+3)*16+c+0]=(requantize(a3_0,mult[c+0],shift[c+0])+4).clamp(-128,127) as i8;
output[(base+p+0)*16+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+4).clamp(-128,127) as i8;
output[(base+p+1)*16+c+1]=(requantize(a1_1,mult[c+1],shift[c+1])+4).clamp(-128,127) as i8;
output[(base+p+2)*16+c+1]=(requantize(a2_1,mult[c+1],shift[c+1])+4).clamp(-128,127) as i8;
output[(base+p+3)*16+c+1]=(requantize(a3_1,mult[c+1],shift[c+1])+4).clamp(-128,127) as i8;
}
for p in (count/4*4..count).step_by(1) {
let v0=&patch[(p+0)*144..(p+0+1)*144];
let mut a0_0=bias[c+0];
let mut a0_1=bias[c+1];
for (&v0,ws) in v0.iter().zip(wb.chunks_exact(2)) {
let v0=v0 as i32;
let w0=ws[0] as i32;
a0_0+=v0*w0;
let w1=ws[1] as i32;
a0_1+=v0*w1;
}
output[(base+p+0)*16+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+4).clamp(-128,127) as i8;
output[(base+p+0)*16+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+4).clamp(-128,127) as i8;
}
}}}
/// # Safety
/// Parameters must satisfy the bundled host-validated quantization and accumulator-bound manifest.
/// Pointers must have the generated lengths, not alias, and use the packed weights and folded bias.
#[no_mangle]pub unsafe extern "C" fn r_kernel_layer_2(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8){
run(core::slice::from_raw_parts(i,16384),core::slice::from_raw_parts(w,2304),core::slice::from_raw_parts(b,16),core::slice::from_raw_parts(m,16),core::slice::from_raw_parts(s,16),core::slice::from_raw_parts_mut(o,16384));}

}
mod layer_4 {

// Scratch: 2592 bytes on stack, plus compiler spills.
// Weights [OC/B, K*K*IC, B]; bias includes original input_offset*sum(weights).
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


#[inline(never)] pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=17424 && weights.len()>=4608 && output.len()>=8192);
assert!(bias.len()>=32 && mult.len()>=32 && shift.len()>=32);
// Quantization ranges and accumulator bound are proven by the model packer.
let mut patch=[0i8;2304];
let mut local_weights=[0i8;288];
for base in (0..256).step_by(16) { let count=core::cmp::min(16,256-base);
for (p,dst) in patch[..count*144].chunks_exact_mut(144).enumerate() { let y=(base+p)/16;let x=(base+p)%16;
for ky in 0..3 {for kx in 0..3 {let iy=(y*2+ky) as isize-0;let ix=(x*2+kx) as isize-0;let target=&mut dst[(ky*3+kx)*16..(ky*3+kx+1)*16];
if iy>=0 && iy<33 && ix>=0 && ix<33 {let q=(iy as usize*33+ix as usize)*16;target.copy_from_slice(&input[q..q+16]);} else {target.fill(-128);} } } }
for c in (0..32).step_by(2) {let wb=&weights[c*144..(c+2)*144];
local_weights.copy_from_slice(wb);let wb=&local_weights[..];
for p in (0..count/4*4).step_by(4) {
let v0=&patch[(p+0)*144..(p+0+1)*144];
let v1=&patch[(p+1)*144..(p+1+1)*144];
let v2=&patch[(p+2)*144..(p+2+1)*144];
let v3=&patch[(p+3)*144..(p+3+1)*144];
let mut a0_0=bias[c+0];
let mut a1_0=bias[c+0];
let mut a2_0=bias[c+0];
let mut a3_0=bias[c+0];
let mut a0_1=bias[c+1];
let mut a1_1=bias[c+1];
let mut a2_1=bias[c+1];
let mut a3_1=bias[c+1];
for ((((&v0,&v1),&v2),&v3),ws) in v0.iter().zip(v1.iter()).zip(v2.iter()).zip(v3.iter()).zip(wb.chunks_exact(2)) {
let v0=v0 as i32;
let v1=v1 as i32;
let v2=v2 as i32;
let v3=v3 as i32;
let w0=ws[0] as i32;
a0_0+=v0*w0;
a1_0+=v1*w0;
a2_0+=v2*w0;
a3_0+=v3*w0;
let w1=ws[1] as i32;
a0_1+=v0*w1;
a1_1+=v1*w1;
a2_1+=v2*w1;
a3_1+=v3*w1;
}
output[(base+p+0)*32+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(base+p+1)*32+c+0]=(requantize(a1_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(base+p+2)*32+c+0]=(requantize(a2_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(base+p+3)*32+c+0]=(requantize(a3_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(base+p+0)*32+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(base+p+1)*32+c+1]=(requantize(a1_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(base+p+2)*32+c+1]=(requantize(a2_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(base+p+3)*32+c+1]=(requantize(a3_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
}
for p in (count/4*4..count).step_by(1) {
let v0=&patch[(p+0)*144..(p+0+1)*144];
let mut a0_0=bias[c+0];
let mut a0_1=bias[c+1];
for (&v0,ws) in v0.iter().zip(wb.chunks_exact(2)) {
let v0=v0 as i32;
let w0=ws[0] as i32;
a0_0+=v0*w0;
let w1=ws[1] as i32;
a0_1+=v0*w1;
}
output[(base+p+0)*32+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(base+p+0)*32+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
}
}}}
/// # Safety
/// Parameters must satisfy the bundled host-validated quantization and accumulator-bound manifest.
/// Pointers must have the generated lengths, not alias, and use the packed weights and folded bias.
#[no_mangle]pub unsafe extern "C" fn r_kernel_layer_4(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8){
run(core::slice::from_raw_parts(i,17424),core::slice::from_raw_parts(w,4608),core::slice::from_raw_parts(b,32),core::slice::from_raw_parts(m,32),core::slice::from_raw_parts(s,32),core::slice::from_raw_parts_mut(o,8192));}

}
mod layer_5 {

// Scratch: 5184 bytes on stack, plus compiler spills.
// Weights [OC/B, K*K*IC, B]; bias includes original input_offset*sum(weights).
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


#[inline(never)] pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=8192 && weights.len()>=9216 && output.len()>=8192);
assert!(bias.len()>=32 && mult.len()>=32 && shift.len()>=32);
// Quantization ranges and accumulator bound are proven by the model packer.
let mut patch=[0i8;4608];
let mut local_weights=[0i8;576];
for base in (0..256).step_by(16) { let count=core::cmp::min(16,256-base);
for (p,dst) in patch[..count*288].chunks_exact_mut(288).enumerate() { let y=(base+p)/16;let x=(base+p)%16;
for ky in 0..3 {for kx in 0..3 {let iy=(y*1+ky) as isize-1;let ix=(x*1+kx) as isize-1;let target=&mut dst[(ky*3+kx)*32..(ky*3+kx+1)*32];
if iy>=0 && iy<16 && ix>=0 && ix<16 {let q=(iy as usize*16+ix as usize)*32;target.copy_from_slice(&input[q..q+32]);} else {target.fill(-128);} } } }
for c in (0..32).step_by(2) {let wb=&weights[c*288..(c+2)*288];
local_weights.copy_from_slice(wb);let wb=&local_weights[..];
for p in (0..count/4*4).step_by(4) {
let v0=&patch[(p+0)*288..(p+0+1)*288];
let v1=&patch[(p+1)*288..(p+1+1)*288];
let v2=&patch[(p+2)*288..(p+2+1)*288];
let v3=&patch[(p+3)*288..(p+3+1)*288];
let mut a0_0=bias[c+0];
let mut a1_0=bias[c+0];
let mut a2_0=bias[c+0];
let mut a3_0=bias[c+0];
let mut a0_1=bias[c+1];
let mut a1_1=bias[c+1];
let mut a2_1=bias[c+1];
let mut a3_1=bias[c+1];
for ((((&v0,&v1),&v2),&v3),ws) in v0.iter().zip(v1.iter()).zip(v2.iter()).zip(v3.iter()).zip(wb.chunks_exact(2)) {
let v0=v0 as i32;
let v1=v1 as i32;
let v2=v2 as i32;
let v3=v3 as i32;
let w0=ws[0] as i32;
a0_0+=v0*w0;
a1_0+=v1*w0;
a2_0+=v2*w0;
a3_0+=v3*w0;
let w1=ws[1] as i32;
a0_1+=v0*w1;
a1_1+=v1*w1;
a2_1+=v2*w1;
a3_1+=v3*w1;
}
output[(base+p+0)*32+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+4).clamp(-128,127) as i8;
output[(base+p+1)*32+c+0]=(requantize(a1_0,mult[c+0],shift[c+0])+4).clamp(-128,127) as i8;
output[(base+p+2)*32+c+0]=(requantize(a2_0,mult[c+0],shift[c+0])+4).clamp(-128,127) as i8;
output[(base+p+3)*32+c+0]=(requantize(a3_0,mult[c+0],shift[c+0])+4).clamp(-128,127) as i8;
output[(base+p+0)*32+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+4).clamp(-128,127) as i8;
output[(base+p+1)*32+c+1]=(requantize(a1_1,mult[c+1],shift[c+1])+4).clamp(-128,127) as i8;
output[(base+p+2)*32+c+1]=(requantize(a2_1,mult[c+1],shift[c+1])+4).clamp(-128,127) as i8;
output[(base+p+3)*32+c+1]=(requantize(a3_1,mult[c+1],shift[c+1])+4).clamp(-128,127) as i8;
}
for p in (count/4*4..count).step_by(1) {
let v0=&patch[(p+0)*288..(p+0+1)*288];
let mut a0_0=bias[c+0];
let mut a0_1=bias[c+1];
for (&v0,ws) in v0.iter().zip(wb.chunks_exact(2)) {
let v0=v0 as i32;
let w0=ws[0] as i32;
a0_0+=v0*w0;
let w1=ws[1] as i32;
a0_1+=v0*w1;
}
output[(base+p+0)*32+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+4).clamp(-128,127) as i8;
output[(base+p+0)*32+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+4).clamp(-128,127) as i8;
}
}}}
/// # Safety
/// Parameters must satisfy the bundled host-validated quantization and accumulator-bound manifest.
/// Pointers must have the generated lengths, not alias, and use the packed weights and folded bias.
#[no_mangle]pub unsafe extern "C" fn r_kernel_layer_5(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8){
run(core::slice::from_raw_parts(i,8192),core::slice::from_raw_parts(w,9216),core::slice::from_raw_parts(b,32),core::slice::from_raw_parts(m,32),core::slice::from_raw_parts(s,32),core::slice::from_raw_parts_mut(o,8192));}

}
mod layer_6 {

// Scratch: 288 bytes on stack, plus compiler spills.
// Weights [OC/B, K*K*IC, B]; bias includes original input_offset*sum(weights).
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


#[inline(never)] pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=16384 && weights.len()>=512 && output.len()>=8192);
assert!(bias.len()>=32 && mult.len()>=32 && shift.len()>=32);
// Quantization ranges and accumulator bound are proven by the model packer.
let mut patch=[0i8;256];
let mut local_weights=[0i8;32];
for base in (0..256).step_by(16) { let count=core::cmp::min(16,256-base);
for (p,dst) in patch[..count*16].chunks_exact_mut(16).enumerate() { let y=(base+p)/16;let x=(base+p)%16;
for ky in 0..1 {for kx in 0..1 {let iy=(y*2+ky) as isize-0;let ix=(x*2+kx) as isize-0;let target=&mut dst[(ky*1+kx)*16..(ky*1+kx+1)*16];
if iy>=0 && iy<32 && ix>=0 && ix<32 {let q=(iy as usize*32+ix as usize)*16;target.copy_from_slice(&input[q..q+16]);} else {target.fill(-128);} } } }
for c in (0..32).step_by(2) {let wb=&weights[c*16..(c+2)*16];
local_weights.copy_from_slice(wb);let wb=&local_weights[..];
for p in (0..count/4*4).step_by(4) {
let v0=&patch[(p+0)*16..(p+0+1)*16];
let v1=&patch[(p+1)*16..(p+1+1)*16];
let v2=&patch[(p+2)*16..(p+2+1)*16];
let v3=&patch[(p+3)*16..(p+3+1)*16];
let mut a0_0=bias[c+0];
let mut a1_0=bias[c+0];
let mut a2_0=bias[c+0];
let mut a3_0=bias[c+0];
let mut a0_1=bias[c+1];
let mut a1_1=bias[c+1];
let mut a2_1=bias[c+1];
let mut a3_1=bias[c+1];
for ((((&v0,&v1),&v2),&v3),ws) in v0.iter().zip(v1.iter()).zip(v2.iter()).zip(v3.iter()).zip(wb.chunks_exact(2)) {
let v0=v0 as i32;
let v1=v1 as i32;
let v2=v2 as i32;
let v3=v3 as i32;
let w0=ws[0] as i32;
a0_0+=v0*w0;
a1_0+=v1*w0;
a2_0+=v2*w0;
a3_0+=v3*w0;
let w1=ws[1] as i32;
a0_1+=v0*w1;
a1_1+=v1*w1;
a2_1+=v2*w1;
a3_1+=v3*w1;
}
output[(base+p+0)*32+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+-17).clamp(-128,127) as i8;
output[(base+p+1)*32+c+0]=(requantize(a1_0,mult[c+0],shift[c+0])+-17).clamp(-128,127) as i8;
output[(base+p+2)*32+c+0]=(requantize(a2_0,mult[c+0],shift[c+0])+-17).clamp(-128,127) as i8;
output[(base+p+3)*32+c+0]=(requantize(a3_0,mult[c+0],shift[c+0])+-17).clamp(-128,127) as i8;
output[(base+p+0)*32+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+-17).clamp(-128,127) as i8;
output[(base+p+1)*32+c+1]=(requantize(a1_1,mult[c+1],shift[c+1])+-17).clamp(-128,127) as i8;
output[(base+p+2)*32+c+1]=(requantize(a2_1,mult[c+1],shift[c+1])+-17).clamp(-128,127) as i8;
output[(base+p+3)*32+c+1]=(requantize(a3_1,mult[c+1],shift[c+1])+-17).clamp(-128,127) as i8;
}
for p in (count/4*4..count).step_by(1) {
let v0=&patch[(p+0)*16..(p+0+1)*16];
let mut a0_0=bias[c+0];
let mut a0_1=bias[c+1];
for (&v0,ws) in v0.iter().zip(wb.chunks_exact(2)) {
let v0=v0 as i32;
let w0=ws[0] as i32;
a0_0+=v0*w0;
let w1=ws[1] as i32;
a0_1+=v0*w1;
}
output[(base+p+0)*32+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+-17).clamp(-128,127) as i8;
output[(base+p+0)*32+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+-17).clamp(-128,127) as i8;
}
}}}
/// # Safety
/// Parameters must satisfy the bundled host-validated quantization and accumulator-bound manifest.
/// Pointers must have the generated lengths, not alias, and use the packed weights and folded bias.
#[no_mangle]pub unsafe extern "C" fn r_kernel_layer_6(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8){
run(core::slice::from_raw_parts(i,16384),core::slice::from_raw_parts(w,512),core::slice::from_raw_parts(b,32),core::slice::from_raw_parts(m,32),core::slice::from_raw_parts(s,32),core::slice::from_raw_parts_mut(o,8192));}

}
mod layer_8 {

// Scratch: 5184 bytes on stack, plus compiler spills.
// Weights [OC/B, K*K*IC, B]; bias includes original input_offset*sum(weights).
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


#[inline(never)] pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=9248 && weights.len()>=18432 && output.len()>=4096);
assert!(bias.len()>=64 && mult.len()>=64 && shift.len()>=64);
// Quantization ranges and accumulator bound are proven by the model packer.
let mut patch=[0i8;4608];
let mut local_weights=[0i8;576];
for base in (0..64).step_by(16) { let count=core::cmp::min(16,64-base);
for (p,dst) in patch[..count*288].chunks_exact_mut(288).enumerate() { let y=(base+p)/8;let x=(base+p)%8;
for ky in 0..3 {for kx in 0..3 {let iy=(y*2+ky) as isize-0;let ix=(x*2+kx) as isize-0;let target=&mut dst[(ky*3+kx)*32..(ky*3+kx+1)*32];
if iy>=0 && iy<17 && ix>=0 && ix<17 {let q=(iy as usize*17+ix as usize)*32;target.copy_from_slice(&input[q..q+32]);} else {target.fill(-128);} } } }
for c in (0..64).step_by(2) {let wb=&weights[c*288..(c+2)*288];
local_weights.copy_from_slice(wb);let wb=&local_weights[..];
for p in (0..count/4*4).step_by(4) {
let v0=&patch[(p+0)*288..(p+0+1)*288];
let v1=&patch[(p+1)*288..(p+1+1)*288];
let v2=&patch[(p+2)*288..(p+2+1)*288];
let v3=&patch[(p+3)*288..(p+3+1)*288];
let mut a0_0=bias[c+0];
let mut a1_0=bias[c+0];
let mut a2_0=bias[c+0];
let mut a3_0=bias[c+0];
let mut a0_1=bias[c+1];
let mut a1_1=bias[c+1];
let mut a2_1=bias[c+1];
let mut a3_1=bias[c+1];
for ((((&v0,&v1),&v2),&v3),ws) in v0.iter().zip(v1.iter()).zip(v2.iter()).zip(v3.iter()).zip(wb.chunks_exact(2)) {
let v0=v0 as i32;
let v1=v1 as i32;
let v2=v2 as i32;
let v3=v3 as i32;
let w0=ws[0] as i32;
a0_0+=v0*w0;
a1_0+=v1*w0;
a2_0+=v2*w0;
a3_0+=v3*w0;
let w1=ws[1] as i32;
a0_1+=v0*w1;
a1_1+=v1*w1;
a2_1+=v2*w1;
a3_1+=v3*w1;
}
output[(base+p+0)*64+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(base+p+1)*64+c+0]=(requantize(a1_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(base+p+2)*64+c+0]=(requantize(a2_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(base+p+3)*64+c+0]=(requantize(a3_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(base+p+0)*64+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(base+p+1)*64+c+1]=(requantize(a1_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(base+p+2)*64+c+1]=(requantize(a2_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(base+p+3)*64+c+1]=(requantize(a3_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
}
for p in (count/4*4..count).step_by(1) {
let v0=&patch[(p+0)*288..(p+0+1)*288];
let mut a0_0=bias[c+0];
let mut a0_1=bias[c+1];
for (&v0,ws) in v0.iter().zip(wb.chunks_exact(2)) {
let v0=v0 as i32;
let w0=ws[0] as i32;
a0_0+=v0*w0;
let w1=ws[1] as i32;
a0_1+=v0*w1;
}
output[(base+p+0)*64+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(base+p+0)*64+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
}
}}}
/// # Safety
/// Parameters must satisfy the bundled host-validated quantization and accumulator-bound manifest.
/// Pointers must have the generated lengths, not alias, and use the packed weights and folded bias.
#[no_mangle]pub unsafe extern "C" fn r_kernel_layer_8(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8){
run(core::slice::from_raw_parts(i,9248),core::slice::from_raw_parts(w,18432),core::slice::from_raw_parts(b,64),core::slice::from_raw_parts(m,64),core::slice::from_raw_parts(s,64),core::slice::from_raw_parts_mut(o,4096));}

}
mod layer_9 {

// Scratch: 10368 bytes on stack, plus compiler spills.
// Weights [OC/B, K*K*IC, B]; bias includes original input_offset*sum(weights).
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


#[inline(never)] pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=4096 && weights.len()>=36864 && output.len()>=4096);
assert!(bias.len()>=64 && mult.len()>=64 && shift.len()>=64);
// Quantization ranges and accumulator bound are proven by the model packer.
let mut patch=[0i8;9216];
let mut local_weights=[0i8;1152];
for base in (0..64).step_by(16) { let count=core::cmp::min(16,64-base);
for (p,dst) in patch[..count*576].chunks_exact_mut(576).enumerate() { let y=(base+p)/8;let x=(base+p)%8;
for ky in 0..3 {for kx in 0..3 {let iy=(y*1+ky) as isize-1;let ix=(x*1+kx) as isize-1;let target=&mut dst[(ky*3+kx)*64..(ky*3+kx+1)*64];
if iy>=0 && iy<8 && ix>=0 && ix<8 {let q=(iy as usize*8+ix as usize)*64;target.copy_from_slice(&input[q..q+64]);} else {target.fill(-128);} } } }
for c in (0..64).step_by(2) {let wb=&weights[c*576..(c+2)*576];
local_weights.copy_from_slice(wb);let wb=&local_weights[..];
for p in (0..count/4*4).step_by(4) {
let v0=&patch[(p+0)*576..(p+0+1)*576];
let v1=&patch[(p+1)*576..(p+1+1)*576];
let v2=&patch[(p+2)*576..(p+2+1)*576];
let v3=&patch[(p+3)*576..(p+3+1)*576];
let mut a0_0=bias[c+0];
let mut a1_0=bias[c+0];
let mut a2_0=bias[c+0];
let mut a3_0=bias[c+0];
let mut a0_1=bias[c+1];
let mut a1_1=bias[c+1];
let mut a2_1=bias[c+1];
let mut a3_1=bias[c+1];
for ((((&v0,&v1),&v2),&v3),ws) in v0.iter().zip(v1.iter()).zip(v2.iter()).zip(v3.iter()).zip(wb.chunks_exact(2)) {
let v0=v0 as i32;
let v1=v1 as i32;
let v2=v2 as i32;
let v3=v3 as i32;
let w0=ws[0] as i32;
a0_0+=v0*w0;
a1_0+=v1*w0;
a2_0+=v2*w0;
a3_0+=v3*w0;
let w1=ws[1] as i32;
a0_1+=v0*w1;
a1_1+=v1*w1;
a2_1+=v2*w1;
a3_1+=v3*w1;
}
output[(base+p+0)*64+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+-2).clamp(-128,127) as i8;
output[(base+p+1)*64+c+0]=(requantize(a1_0,mult[c+0],shift[c+0])+-2).clamp(-128,127) as i8;
output[(base+p+2)*64+c+0]=(requantize(a2_0,mult[c+0],shift[c+0])+-2).clamp(-128,127) as i8;
output[(base+p+3)*64+c+0]=(requantize(a3_0,mult[c+0],shift[c+0])+-2).clamp(-128,127) as i8;
output[(base+p+0)*64+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+-2).clamp(-128,127) as i8;
output[(base+p+1)*64+c+1]=(requantize(a1_1,mult[c+1],shift[c+1])+-2).clamp(-128,127) as i8;
output[(base+p+2)*64+c+1]=(requantize(a2_1,mult[c+1],shift[c+1])+-2).clamp(-128,127) as i8;
output[(base+p+3)*64+c+1]=(requantize(a3_1,mult[c+1],shift[c+1])+-2).clamp(-128,127) as i8;
}
for p in (count/4*4..count).step_by(1) {
let v0=&patch[(p+0)*576..(p+0+1)*576];
let mut a0_0=bias[c+0];
let mut a0_1=bias[c+1];
for (&v0,ws) in v0.iter().zip(wb.chunks_exact(2)) {
let v0=v0 as i32;
let w0=ws[0] as i32;
a0_0+=v0*w0;
let w1=ws[1] as i32;
a0_1+=v0*w1;
}
output[(base+p+0)*64+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+-2).clamp(-128,127) as i8;
output[(base+p+0)*64+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+-2).clamp(-128,127) as i8;
}
}}}
/// # Safety
/// Parameters must satisfy the bundled host-validated quantization and accumulator-bound manifest.
/// Pointers must have the generated lengths, not alias, and use the packed weights and folded bias.
#[no_mangle]pub unsafe extern "C" fn r_kernel_layer_9(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8){
run(core::slice::from_raw_parts(i,4096),core::slice::from_raw_parts(w,36864),core::slice::from_raw_parts(b,64),core::slice::from_raw_parts(m,64),core::slice::from_raw_parts(s,64),core::slice::from_raw_parts_mut(o,4096));}

}
mod layer_10 {

// Scratch: 576 bytes on stack, plus compiler spills.
// Weights [OC/B, K*K*IC, B]; bias includes original input_offset*sum(weights).
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


#[inline(never)] pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=8192 && weights.len()>=2048 && output.len()>=4096);
assert!(bias.len()>=64 && mult.len()>=64 && shift.len()>=64);
// Quantization ranges and accumulator bound are proven by the model packer.
let mut patch=[0i8;512];
let mut local_weights=[0i8;64];
for base in (0..64).step_by(16) { let count=core::cmp::min(16,64-base);
for (p,dst) in patch[..count*32].chunks_exact_mut(32).enumerate() { let y=(base+p)/8;let x=(base+p)%8;
for ky in 0..1 {for kx in 0..1 {let iy=(y*2+ky) as isize-0;let ix=(x*2+kx) as isize-0;let target=&mut dst[(ky*1+kx)*32..(ky*1+kx+1)*32];
if iy>=0 && iy<16 && ix>=0 && ix<16 {let q=(iy as usize*16+ix as usize)*32;target.copy_from_slice(&input[q..q+32]);} else {target.fill(-128);} } } }
for c in (0..64).step_by(2) {let wb=&weights[c*32..(c+2)*32];
local_weights.copy_from_slice(wb);let wb=&local_weights[..];
for p in (0..count/4*4).step_by(4) {
let v0=&patch[(p+0)*32..(p+0+1)*32];
let v1=&patch[(p+1)*32..(p+1+1)*32];
let v2=&patch[(p+2)*32..(p+2+1)*32];
let v3=&patch[(p+3)*32..(p+3+1)*32];
let mut a0_0=bias[c+0];
let mut a1_0=bias[c+0];
let mut a2_0=bias[c+0];
let mut a3_0=bias[c+0];
let mut a0_1=bias[c+1];
let mut a1_1=bias[c+1];
let mut a2_1=bias[c+1];
let mut a3_1=bias[c+1];
for ((((&v0,&v1),&v2),&v3),ws) in v0.iter().zip(v1.iter()).zip(v2.iter()).zip(v3.iter()).zip(wb.chunks_exact(2)) {
let v0=v0 as i32;
let v1=v1 as i32;
let v2=v2 as i32;
let v3=v3 as i32;
let w0=ws[0] as i32;
a0_0+=v0*w0;
a1_0+=v1*w0;
a2_0+=v2*w0;
a3_0+=v3*w0;
let w1=ws[1] as i32;
a0_1+=v0*w1;
a1_1+=v1*w1;
a2_1+=v2*w1;
a3_1+=v3*w1;
}
output[(base+p+0)*64+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+38).clamp(-128,127) as i8;
output[(base+p+1)*64+c+0]=(requantize(a1_0,mult[c+0],shift[c+0])+38).clamp(-128,127) as i8;
output[(base+p+2)*64+c+0]=(requantize(a2_0,mult[c+0],shift[c+0])+38).clamp(-128,127) as i8;
output[(base+p+3)*64+c+0]=(requantize(a3_0,mult[c+0],shift[c+0])+38).clamp(-128,127) as i8;
output[(base+p+0)*64+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+38).clamp(-128,127) as i8;
output[(base+p+1)*64+c+1]=(requantize(a1_1,mult[c+1],shift[c+1])+38).clamp(-128,127) as i8;
output[(base+p+2)*64+c+1]=(requantize(a2_1,mult[c+1],shift[c+1])+38).clamp(-128,127) as i8;
output[(base+p+3)*64+c+1]=(requantize(a3_1,mult[c+1],shift[c+1])+38).clamp(-128,127) as i8;
}
for p in (count/4*4..count).step_by(1) {
let v0=&patch[(p+0)*32..(p+0+1)*32];
let mut a0_0=bias[c+0];
let mut a0_1=bias[c+1];
for (&v0,ws) in v0.iter().zip(wb.chunks_exact(2)) {
let v0=v0 as i32;
let w0=ws[0] as i32;
a0_0+=v0*w0;
let w1=ws[1] as i32;
a0_1+=v0*w1;
}
output[(base+p+0)*64+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+38).clamp(-128,127) as i8;
output[(base+p+0)*64+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+38).clamp(-128,127) as i8;
}
}}}
/// # Safety
/// Parameters must satisfy the bundled host-validated quantization and accumulator-bound manifest.
/// Pointers must have the generated lengths, not alias, and use the packed weights and folded bias.
#[no_mangle]pub unsafe extern "C" fn r_kernel_layer_10(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8){
run(core::slice::from_raw_parts(i,8192),core::slice::from_raw_parts(w,2048),core::slice::from_raw_parts(b,64),core::slice::from_raw_parts(m,64),core::slice::from_raw_parts(s,64),core::slice::from_raw_parts_mut(o,4096));}

}
mod layer_14 {

// Scratch: 192 bytes on stack, plus compiler spills.
// Weights [OC/B, K*K*IC, B]; bias includes original input_offset*sum(weights).
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


#[inline(never)] pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=64 && weights.len()>=640 && output.len()>=10);
assert!(bias.len()>=10 && mult.len()>=10 && shift.len()>=10);
// Quantization ranges and accumulator bound are proven by the model packer.
let mut patch=[0i8;64];
let mut local_weights=[0i8;128];
for base in (0..1).step_by(1) { let count=core::cmp::min(1,1-base);
for (p,dst) in patch[..count*64].chunks_exact_mut(64).enumerate() { let y=(base+p)/1;let x=(base+p)%1;
for ky in 0..1 {for kx in 0..1 {let iy=(y*1+ky) as isize-0;let ix=(x*1+kx) as isize-0;let target=&mut dst[(ky*1+kx)*64..(ky*1+kx+1)*64];
if iy>=0 && iy<1 && ix>=0 && ix<1 {let q=(iy as usize*1+ix as usize)*64;target.copy_from_slice(&input[q..q+64]);} else {target.fill(-128);} } } }
for c in (0..10).step_by(2) {let wb=&weights[c*64..(c+2)*64];
local_weights.copy_from_slice(wb);let wb=&local_weights[..];
for p in (0..count/1*1).step_by(1) {
let v0=&patch[(p+0)*64..(p+0+1)*64];
let mut a0_0=bias[c+0];
let mut a0_1=bias[c+1];
for (&v0,ws) in v0.iter().zip(wb.chunks_exact(2)) {
let v0=v0 as i32;
let w0=ws[0] as i32;
a0_0+=v0*w0;
let w1=ws[1] as i32;
a0_1+=v0*w1;
}
output[(base+p+0)*10+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+24).clamp(-128,127) as i8;
output[(base+p+0)*10+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+24).clamp(-128,127) as i8;
}
for p in (count/1*1..count).step_by(1) {
let v0=&patch[(p+0)*64..(p+0+1)*64];
let mut a0_0=bias[c+0];
let mut a0_1=bias[c+1];
for (&v0,ws) in v0.iter().zip(wb.chunks_exact(2)) {
let v0=v0 as i32;
let w0=ws[0] as i32;
a0_0+=v0*w0;
let w1=ws[1] as i32;
a0_1+=v0*w1;
}
output[(base+p+0)*10+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+24).clamp(-128,127) as i8;
output[(base+p+0)*10+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+24).clamp(-128,127) as i8;
}
}}}
/// # Safety
/// Parameters must satisfy the bundled host-validated quantization and accumulator-bound manifest.
/// Pointers must have the generated lengths, not alias, and use the packed weights and folded bias.
#[no_mangle]pub unsafe extern "C" fn r_kernel_layer_14(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8){
run(core::slice::from_raw_parts(i,64),core::slice::from_raw_parts(w,640),core::slice::from_raw_parts(b,10),core::slice::from_raw_parts(m,10),core::slice::from_raw_parts(s,10),core::slice::from_raw_parts_mut(o,10));}

}
#[panic_handler] fn panic(_: &core::panic::PanicInfo) -> ! { loop { core::hint::spin_loop(); } }
