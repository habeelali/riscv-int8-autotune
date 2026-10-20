#![no_std]
// Generated for conv3x3 with b4_u16_s0_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 4096, 4608, 8192.
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
assert!(input.len()>=4096 && weights.len()>=4608 && output.len()>=8192);
assert!(bias.len()>=32 && mult.len()>=32 && shift.len()>=32);
assert!(mult[..32].iter().all(|&m|m>=0) && shift[..32].iter().all(|&q|(-30..=0).contains(&q)));
for y in 0..16 { for x in 0..16 {
let mut c=0usize; while c<32 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
let by=(y*1) as isize-1; let bx=(x*1) as isize-1;
let ys=0isize.max(-by) as usize; let ye=(3isize).min(16isize-by) as usize;
let xs=0isize.max(-bx) as usize; let xe=(3isize).min(16isize-bx) as usize;
for ky in ys..ye { for kx in xs..xe {
let ip=(((by+ky as isize) as usize)*16+(bx+kx as isize) as usize)*16; let wp=(ky*3+kx)*16;
let mut i=0usize; while i < 16 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*144+wp)+i+0] as i32;
a1 += v * weights[((c+1)*144+wp)+i+0] as i32;
a2 += v * weights[((c+2)*144+wp)+i+0] as i32;
a3 += v * weights[((c+3)*144+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*144+wp)+i+1] as i32;
a1 += v * weights[((c+1)*144+wp)+i+1] as i32;
a2 += v * weights[((c+2)*144+wp)+i+1] as i32;
a3 += v * weights[((c+3)*144+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*144+wp)+i+2] as i32;
a1 += v * weights[((c+1)*144+wp)+i+2] as i32;
a2 += v * weights[((c+2)*144+wp)+i+2] as i32;
a3 += v * weights[((c+3)*144+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*144+wp)+i+3] as i32;
a1 += v * weights[((c+1)*144+wp)+i+3] as i32;
a2 += v * weights[((c+2)*144+wp)+i+3] as i32;
a3 += v * weights[((c+3)*144+wp)+i+3] as i32;
let v = input[ip+i+4] as i32 + -3;
a0 += v * weights[((c+0)*144+wp)+i+4] as i32;
a1 += v * weights[((c+1)*144+wp)+i+4] as i32;
a2 += v * weights[((c+2)*144+wp)+i+4] as i32;
a3 += v * weights[((c+3)*144+wp)+i+4] as i32;
let v = input[ip+i+5] as i32 + -3;
a0 += v * weights[((c+0)*144+wp)+i+5] as i32;
a1 += v * weights[((c+1)*144+wp)+i+5] as i32;
a2 += v * weights[((c+2)*144+wp)+i+5] as i32;
a3 += v * weights[((c+3)*144+wp)+i+5] as i32;
let v = input[ip+i+6] as i32 + -3;
a0 += v * weights[((c+0)*144+wp)+i+6] as i32;
a1 += v * weights[((c+1)*144+wp)+i+6] as i32;
a2 += v * weights[((c+2)*144+wp)+i+6] as i32;
a3 += v * weights[((c+3)*144+wp)+i+6] as i32;
let v = input[ip+i+7] as i32 + -3;
a0 += v * weights[((c+0)*144+wp)+i+7] as i32;
a1 += v * weights[((c+1)*144+wp)+i+7] as i32;
a2 += v * weights[((c+2)*144+wp)+i+7] as i32;
a3 += v * weights[((c+3)*144+wp)+i+7] as i32;
let v = input[ip+i+8] as i32 + -3;
a0 += v * weights[((c+0)*144+wp)+i+8] as i32;
a1 += v * weights[((c+1)*144+wp)+i+8] as i32;
a2 += v * weights[((c+2)*144+wp)+i+8] as i32;
a3 += v * weights[((c+3)*144+wp)+i+8] as i32;
let v = input[ip+i+9] as i32 + -3;
a0 += v * weights[((c+0)*144+wp)+i+9] as i32;
a1 += v * weights[((c+1)*144+wp)+i+9] as i32;
a2 += v * weights[((c+2)*144+wp)+i+9] as i32;
a3 += v * weights[((c+3)*144+wp)+i+9] as i32;
let v = input[ip+i+10] as i32 + -3;
a0 += v * weights[((c+0)*144+wp)+i+10] as i32;
a1 += v * weights[((c+1)*144+wp)+i+10] as i32;
a2 += v * weights[((c+2)*144+wp)+i+10] as i32;
a3 += v * weights[((c+3)*144+wp)+i+10] as i32;
let v = input[ip+i+11] as i32 + -3;
a0 += v * weights[((c+0)*144+wp)+i+11] as i32;
a1 += v * weights[((c+1)*144+wp)+i+11] as i32;
a2 += v * weights[((c+2)*144+wp)+i+11] as i32;
a3 += v * weights[((c+3)*144+wp)+i+11] as i32;
let v = input[ip+i+12] as i32 + -3;
a0 += v * weights[((c+0)*144+wp)+i+12] as i32;
a1 += v * weights[((c+1)*144+wp)+i+12] as i32;
a2 += v * weights[((c+2)*144+wp)+i+12] as i32;
a3 += v * weights[((c+3)*144+wp)+i+12] as i32;
let v = input[ip+i+13] as i32 + -3;
a0 += v * weights[((c+0)*144+wp)+i+13] as i32;
a1 += v * weights[((c+1)*144+wp)+i+13] as i32;
a2 += v * weights[((c+2)*144+wp)+i+13] as i32;
a3 += v * weights[((c+3)*144+wp)+i+13] as i32;
let v = input[ip+i+14] as i32 + -3;
a0 += v * weights[((c+0)*144+wp)+i+14] as i32;
a1 += v * weights[((c+1)*144+wp)+i+14] as i32;
a2 += v * weights[((c+2)*144+wp)+i+14] as i32;
a3 += v * weights[((c+3)*144+wp)+i+14] as i32;
let v = input[ip+i+15] as i32 + -3;
a0 += v * weights[((c+0)*144+wp)+i+15] as i32;
a1 += v * weights[((c+1)*144+wp)+i+15] as i32;
a2 += v * weights[((c+2)*144+wp)+i+15] as i32;
a3 += v * weights[((c+3)*144+wp)+i+15] as i32;
i+=16; }
} }
output[(y*16+x)*32+c+0] = (requantize(a0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*16+x)*32+c+1] = (requantize(a1,mult[c+1],shift[c+1])+5).clamp(-111,103) as i8;
output[(y*16+x)*32+c+2] = (requantize(a2,mult[c+2],shift[c+2])+5).clamp(-111,103) as i8;
output[(y*16+x)*32+c+3] = (requantize(a3,mult[c+3],shift[c+3])+5).clamp(-111,103) as i8;
c+=4; }
} } }
/// # Safety
/// Pointers must be aligned, valid for lengths 4096, 4608, 32, 32, 32, 8192 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn kernel(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,4096),core::slice::from_raw_parts(w,4608),core::slice::from_raw_parts(b,32),core::slice::from_raw_parts(m,32),core::slice::from_raw_parts(s,32),core::slice::from_raw_parts_mut(o,8192)); }
