#![no_std]
// Generated for pointwise with b1_u1_s0_p4_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 8192, 1024, 8192.
/// Exact default two-rounding result for nonnegative multipliers and shifts
/// in -30..=0. Generated entry points validate these conditions once per call.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!(multiplier >= 0 && (-30..=0).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let mask = ((1u32 << exponent) - 1) as i32;
    let base = high >> exponent;
    base + i32::from((high & mask) > ((mask >> 1) + i32::from(base < 0)))
}

/// All buffers are caller owned. INT32 accumulation and shifted values must fit.
#[inline(never)]
pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=8192 && weights.len()>=1024 && output.len()>=8192);
assert!(bias.len()>=32 && mult.len()>=32 && shift.len()>=32);
assert!(mult[..32].iter().all(|&m|m>=0) && shift[..32].iter().all(|&q|(-30..=0).contains(&q)));
for y in 0..16 { let mut x=0usize; while x<16 {
let mut c=0usize; while c<32 {
let base0=bias[c+0];
let mut a0_0=base0;
let mut a1_0=base0;
let mut a2_0=base0;
let mut a3_0=base0;
let mut i=0usize; while i<32 {
{
let v0=input[(y*1*16+(x+0)*1)*32+i+0] as i32+-3;
let v1=input[(y*1*16+(x+1)*1)*32+i+0] as i32+-3;
let v2=input[(y*1*16+(x+2)*1)*32+i+0] as i32+-3;
let v3=input[(y*1*16+(x+3)*1)*32+i+0] as i32+-3;
let wb=weights[(c+0)*32+i+0] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
a2_0 += v2*wb;
a3_0 += v3*wb;
}
i+=1; }
output[(y*16+x+0)*32+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*16+x+1)*32+c+0]=(requantize(a1_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*16+x+2)*32+c+0]=(requantize(a2_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*16+x+3)*32+c+0]=(requantize(a3_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
c+=1; }
x+=4; }
} }
/// # Safety
/// Pointers must be aligned, valid for lengths 8192, 1024, 32, 32, 32, 8192 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn kernel(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,8192),core::slice::from_raw_parts(w,1024),core::slice::from_raw_parts(b,32),core::slice::from_raw_parts(m,32),core::slice::from_raw_parts(s,32),core::slice::from_raw_parts_mut(o,8192)); }
