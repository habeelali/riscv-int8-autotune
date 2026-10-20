#![no_std]
// Generated for fully_connected with b16_u1_s0_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 256, 32768, 128.
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
assert!(input.len()>=256 && weights.len()>=32768 && output.len()>=128);
assert!(bias.len()>=128 && mult.len()>=128 && shift.len()>=128);
assert!(mult[..128].iter().all(|&m|m>=0) && shift[..128].iter().all(|&q|(-30..=0).contains(&q)));
for y in 0..1 { for x in 0..1 {
let mut c=0usize; while c<128 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
let mut a4=bias[c+4];
let mut a5=bias[c+5];
let mut a6=bias[c+6];
let mut a7=bias[c+7];
let mut a8=bias[c+8];
let mut a9=bias[c+9];
let mut a10=bias[c+10];
let mut a11=bias[c+11];
let mut a12=bias[c+12];
let mut a13=bias[c+13];
let mut a14=bias[c+14];
let mut a15=bias[c+15];
let by=(y*1) as isize-0; let bx=(x*1) as isize-0;
let ys=0isize.max(-by) as usize; let ye=(1isize).min(1isize-by) as usize;
let xs=0isize.max(-bx) as usize; let xe=(1isize).min(1isize-bx) as usize;
for ky in ys..ye { for kx in xs..xe {
let ip=(((by+ky as isize) as usize)*1+(bx+kx as isize) as usize)*256; let wp=(ky*1+kx)*256;
let mut i=0usize; while i < 256 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*256+wp)+i+0] as i32;
a1 += v * weights[((c+1)*256+wp)+i+0] as i32;
a2 += v * weights[((c+2)*256+wp)+i+0] as i32;
a3 += v * weights[((c+3)*256+wp)+i+0] as i32;
a4 += v * weights[((c+4)*256+wp)+i+0] as i32;
a5 += v * weights[((c+5)*256+wp)+i+0] as i32;
a6 += v * weights[((c+6)*256+wp)+i+0] as i32;
a7 += v * weights[((c+7)*256+wp)+i+0] as i32;
a8 += v * weights[((c+8)*256+wp)+i+0] as i32;
a9 += v * weights[((c+9)*256+wp)+i+0] as i32;
a10 += v * weights[((c+10)*256+wp)+i+0] as i32;
a11 += v * weights[((c+11)*256+wp)+i+0] as i32;
a12 += v * weights[((c+12)*256+wp)+i+0] as i32;
a13 += v * weights[((c+13)*256+wp)+i+0] as i32;
a14 += v * weights[((c+14)*256+wp)+i+0] as i32;
a15 += v * weights[((c+15)*256+wp)+i+0] as i32;
i+=1; }
} }
output[(y*1+x)*128+c+0] = (requantize(a0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*1+x)*128+c+1] = (requantize(a1,mult[c+1],shift[c+1])+5).clamp(-111,103) as i8;
output[(y*1+x)*128+c+2] = (requantize(a2,mult[c+2],shift[c+2])+5).clamp(-111,103) as i8;
output[(y*1+x)*128+c+3] = (requantize(a3,mult[c+3],shift[c+3])+5).clamp(-111,103) as i8;
output[(y*1+x)*128+c+4] = (requantize(a4,mult[c+4],shift[c+4])+5).clamp(-111,103) as i8;
output[(y*1+x)*128+c+5] = (requantize(a5,mult[c+5],shift[c+5])+5).clamp(-111,103) as i8;
output[(y*1+x)*128+c+6] = (requantize(a6,mult[c+6],shift[c+6])+5).clamp(-111,103) as i8;
output[(y*1+x)*128+c+7] = (requantize(a7,mult[c+7],shift[c+7])+5).clamp(-111,103) as i8;
output[(y*1+x)*128+c+8] = (requantize(a8,mult[c+8],shift[c+8])+5).clamp(-111,103) as i8;
output[(y*1+x)*128+c+9] = (requantize(a9,mult[c+9],shift[c+9])+5).clamp(-111,103) as i8;
output[(y*1+x)*128+c+10] = (requantize(a10,mult[c+10],shift[c+10])+5).clamp(-111,103) as i8;
output[(y*1+x)*128+c+11] = (requantize(a11,mult[c+11],shift[c+11])+5).clamp(-111,103) as i8;
output[(y*1+x)*128+c+12] = (requantize(a12,mult[c+12],shift[c+12])+5).clamp(-111,103) as i8;
output[(y*1+x)*128+c+13] = (requantize(a13,mult[c+13],shift[c+13])+5).clamp(-111,103) as i8;
output[(y*1+x)*128+c+14] = (requantize(a14,mult[c+14],shift[c+14])+5).clamp(-111,103) as i8;
output[(y*1+x)*128+c+15] = (requantize(a15,mult[c+15],shift[c+15])+5).clamp(-111,103) as i8;
c+=16; }
} } }
/// # Safety
/// Pointers must be aligned, valid for lengths 256, 32768, 128, 128, 128, 128 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn kernel(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,256),core::slice::from_raw_parts(w,32768),core::slice::from_raw_parts(b,128),core::slice::from_raw_parts(m,128),core::slice::from_raw_parts(s,128),core::slice::from_raw_parts_mut(o,128)); }
