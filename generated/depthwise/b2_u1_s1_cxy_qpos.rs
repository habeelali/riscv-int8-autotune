#![no_std]
// Generated for depthwise with b2_u1_s1_cxy_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 16384, 144, 16384.
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
assert!(input.len()>=16384 && weights.len()>=144 && output.len()>=16384);
assert!(bias.len()>=16 && mult.len()>=16 && shift.len()>=16);
assert!(mult[..16].iter().all(|&m|m>=0) && shift[..16].iter().all(|&q|(-30..=0).contains(&q)));
let mut c=0usize; while c<16 { for y in 0..32 { for x in 0..32 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
{ let iy=(y*1+0) as isize-1; let ix=(x*1+0) as isize-1;
if iy>=0 && iy<32 && ix>=0 && ix<32 {
let ip=(iy as usize*32+ix as usize)*16; let wp=0;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
} }
{ let iy=(y*1+0) as isize-1; let ix=(x*1+1) as isize-1;
if iy>=0 && iy<32 && ix>=0 && ix<32 {
let ip=(iy as usize*32+ix as usize)*16; let wp=16;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
} }
{ let iy=(y*1+0) as isize-1; let ix=(x*1+2) as isize-1;
if iy>=0 && iy<32 && ix>=0 && ix<32 {
let ip=(iy as usize*32+ix as usize)*16; let wp=32;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
} }
{ let iy=(y*1+1) as isize-1; let ix=(x*1+0) as isize-1;
if iy>=0 && iy<32 && ix>=0 && ix<32 {
let ip=(iy as usize*32+ix as usize)*16; let wp=48;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
} }
{ let iy=(y*1+1) as isize-1; let ix=(x*1+1) as isize-1;
if iy>=0 && iy<32 && ix>=0 && ix<32 {
let ip=(iy as usize*32+ix as usize)*16; let wp=64;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
} }
{ let iy=(y*1+1) as isize-1; let ix=(x*1+2) as isize-1;
if iy>=0 && iy<32 && ix>=0 && ix<32 {
let ip=(iy as usize*32+ix as usize)*16; let wp=80;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
} }
{ let iy=(y*1+2) as isize-1; let ix=(x*1+0) as isize-1;
if iy>=0 && iy<32 && ix>=0 && ix<32 {
let ip=(iy as usize*32+ix as usize)*16; let wp=96;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
} }
{ let iy=(y*1+2) as isize-1; let ix=(x*1+1) as isize-1;
if iy>=0 && iy<32 && ix>=0 && ix<32 {
let ip=(iy as usize*32+ix as usize)*16; let wp=112;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
} }
{ let iy=(y*1+2) as isize-1; let ix=(x*1+2) as isize-1;
if iy>=0 && iy<32 && ix>=0 && ix<32 {
let ip=(iy as usize*32+ix as usize)*16; let wp=128;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
} }
output[(y*32+x)*16+c+0] = (requantize(a0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*32+x)*16+c+1] = (requantize(a1,mult[c+1],shift[c+1])+5).clamp(-111,103) as i8;
} } c+=2; }
}
/// # Safety
/// Pointers must be aligned, valid for lengths 16384, 144, 16, 16, 16, 16384 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn kernel(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,16384),core::slice::from_raw_parts(w,144),core::slice::from_raw_parts(b,16),core::slice::from_raw_parts(m,16),core::slice::from_raw_parts(s,16),core::slice::from_raw_parts_mut(o,16384)); }
