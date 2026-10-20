#![no_std]
// Generated for fc_small with b4_u8_s0_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 31, 403, 13.
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
assert!(input.len()>=31 && weights.len()>=403 && output.len()>=13);
assert!(bias.len()>=13 && mult.len()>=13 && shift.len()>=13);
assert!(mult[..13].iter().all(|&m|m>=0) && shift[..13].iter().all(|&q|(-30..=0).contains(&q)));
for y in 0..1 { for x in 0..1 {
let mut c=0usize; while c<12 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
let by=(y*1) as isize-0; let bx=(x*1) as isize-0;
let ys=0isize.max(-by) as usize; let ye=(1isize).min(1isize-by) as usize;
let xs=0isize.max(-bx) as usize; let xe=(1isize).min(1isize-bx) as usize;
for ky in ys..ye { for kx in xs..xe {
let ip=(((by+ky as isize) as usize)*1+(bx+kx as isize) as usize)*31; let wp=(ky*1+kx)*31;
let mut i=0usize; while i < 24 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+i+0] as i32;
a1 += v * weights[((c+1)*31+wp)+i+0] as i32;
a2 += v * weights[((c+2)*31+wp)+i+0] as i32;
a3 += v * weights[((c+3)*31+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+i+1] as i32;
a1 += v * weights[((c+1)*31+wp)+i+1] as i32;
a2 += v * weights[((c+2)*31+wp)+i+1] as i32;
a3 += v * weights[((c+3)*31+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+i+2] as i32;
a1 += v * weights[((c+1)*31+wp)+i+2] as i32;
a2 += v * weights[((c+2)*31+wp)+i+2] as i32;
a3 += v * weights[((c+3)*31+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+i+3] as i32;
a1 += v * weights[((c+1)*31+wp)+i+3] as i32;
a2 += v * weights[((c+2)*31+wp)+i+3] as i32;
a3 += v * weights[((c+3)*31+wp)+i+3] as i32;
let v = input[ip+i+4] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+i+4] as i32;
a1 += v * weights[((c+1)*31+wp)+i+4] as i32;
a2 += v * weights[((c+2)*31+wp)+i+4] as i32;
a3 += v * weights[((c+3)*31+wp)+i+4] as i32;
let v = input[ip+i+5] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+i+5] as i32;
a1 += v * weights[((c+1)*31+wp)+i+5] as i32;
a2 += v * weights[((c+2)*31+wp)+i+5] as i32;
a3 += v * weights[((c+3)*31+wp)+i+5] as i32;
let v = input[ip+i+6] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+i+6] as i32;
a1 += v * weights[((c+1)*31+wp)+i+6] as i32;
a2 += v * weights[((c+2)*31+wp)+i+6] as i32;
a3 += v * weights[((c+3)*31+wp)+i+6] as i32;
let v = input[ip+i+7] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+i+7] as i32;
a1 += v * weights[((c+1)*31+wp)+i+7] as i32;
a2 += v * weights[((c+2)*31+wp)+i+7] as i32;
a3 += v * weights[((c+3)*31+wp)+i+7] as i32;
i+=8; }
let v = input[ip+24] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+24] as i32;
a1 += v * weights[((c+1)*31+wp)+24] as i32;
a2 += v * weights[((c+2)*31+wp)+24] as i32;
a3 += v * weights[((c+3)*31+wp)+24] as i32;
let v = input[ip+25] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+25] as i32;
a1 += v * weights[((c+1)*31+wp)+25] as i32;
a2 += v * weights[((c+2)*31+wp)+25] as i32;
a3 += v * weights[((c+3)*31+wp)+25] as i32;
let v = input[ip+26] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+26] as i32;
a1 += v * weights[((c+1)*31+wp)+26] as i32;
a2 += v * weights[((c+2)*31+wp)+26] as i32;
a3 += v * weights[((c+3)*31+wp)+26] as i32;
let v = input[ip+27] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+27] as i32;
a1 += v * weights[((c+1)*31+wp)+27] as i32;
a2 += v * weights[((c+2)*31+wp)+27] as i32;
a3 += v * weights[((c+3)*31+wp)+27] as i32;
let v = input[ip+28] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+28] as i32;
a1 += v * weights[((c+1)*31+wp)+28] as i32;
a2 += v * weights[((c+2)*31+wp)+28] as i32;
a3 += v * weights[((c+3)*31+wp)+28] as i32;
let v = input[ip+29] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+29] as i32;
a1 += v * weights[((c+1)*31+wp)+29] as i32;
a2 += v * weights[((c+2)*31+wp)+29] as i32;
a3 += v * weights[((c+3)*31+wp)+29] as i32;
let v = input[ip+30] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+30] as i32;
a1 += v * weights[((c+1)*31+wp)+30] as i32;
a2 += v * weights[((c+2)*31+wp)+30] as i32;
a3 += v * weights[((c+3)*31+wp)+30] as i32;
} }
output[(y*1+x)*13+c+0] = (requantize(a0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*1+x)*13+c+1] = (requantize(a1,mult[c+1],shift[c+1])+5).clamp(-111,103) as i8;
output[(y*1+x)*13+c+2] = (requantize(a2,mult[c+2],shift[c+2])+5).clamp(-111,103) as i8;
output[(y*1+x)*13+c+3] = (requantize(a3,mult[c+3],shift[c+3])+5).clamp(-111,103) as i8;
c+=4; }
while c<13 {
let mut a0=bias[c+0];
let by=(y*1) as isize-0; let bx=(x*1) as isize-0;
let ys=0isize.max(-by) as usize; let ye=(1isize).min(1isize-by) as usize;
let xs=0isize.max(-bx) as usize; let xe=(1isize).min(1isize-bx) as usize;
for ky in ys..ye { for kx in xs..xe {
let ip=(((by+ky as isize) as usize)*1+(bx+kx as isize) as usize)*31; let wp=(ky*1+kx)*31;
let mut i=0usize; while i < 24 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+i+3] as i32;
let v = input[ip+i+4] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+i+4] as i32;
let v = input[ip+i+5] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+i+5] as i32;
let v = input[ip+i+6] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+i+6] as i32;
let v = input[ip+i+7] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+i+7] as i32;
i+=8; }
let v = input[ip+24] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+24] as i32;
let v = input[ip+25] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+25] as i32;
let v = input[ip+26] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+26] as i32;
let v = input[ip+27] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+27] as i32;
let v = input[ip+28] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+28] as i32;
let v = input[ip+29] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+29] as i32;
let v = input[ip+30] as i32 + -3;
a0 += v * weights[((c+0)*31+wp)+30] as i32;
} }
output[(y*1+x)*13+c+0] = (requantize(a0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
c+=1; }
} } }
/// # Safety
/// Pointers must be aligned, valid for lengths 31, 403, 13, 13, 13, 13 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn kernel(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,31),core::slice::from_raw_parts(w,403),core::slice::from_raw_parts(b,13),core::slice::from_raw_parts(m,13),core::slice::from_raw_parts(s,13),core::slice::from_raw_parts_mut(o,13)); }
