#![no_std]
// Generated for pointwise_small with b4_u16_s0_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 245, 63, 315.
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
assert!(input.len()>=245 && weights.len()>=63 && output.len()>=315);
assert!(bias.len()>=9 && mult.len()>=9 && shift.len()>=9);
assert!(mult[..9].iter().all(|&m|m>=0) && shift[..9].iter().all(|&q|(-30..=0).contains(&q)));
for y in 0..5 { for x in 0..7 {
let mut c=0usize; while c<8 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
let by=(y*1) as isize-0; let bx=(x*1) as isize-0;
let ys=0isize.max(-by) as usize; let ye=(1isize).min(5isize-by) as usize;
let xs=0isize.max(-bx) as usize; let xe=(1isize).min(7isize-bx) as usize;
for ky in ys..ye { for kx in xs..xe {
let ip=(((by+ky as isize) as usize)*7+(bx+kx as isize) as usize)*7; let wp=(ky*1+kx)*7;
let mut i=0usize; while i < 0 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+0] as i32;
a1 += v * weights[((c+1)*7+wp)+i+0] as i32;
a2 += v * weights[((c+2)*7+wp)+i+0] as i32;
a3 += v * weights[((c+3)*7+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+1] as i32;
a1 += v * weights[((c+1)*7+wp)+i+1] as i32;
a2 += v * weights[((c+2)*7+wp)+i+1] as i32;
a3 += v * weights[((c+3)*7+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+2] as i32;
a1 += v * weights[((c+1)*7+wp)+i+2] as i32;
a2 += v * weights[((c+2)*7+wp)+i+2] as i32;
a3 += v * weights[((c+3)*7+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+3] as i32;
a1 += v * weights[((c+1)*7+wp)+i+3] as i32;
a2 += v * weights[((c+2)*7+wp)+i+3] as i32;
a3 += v * weights[((c+3)*7+wp)+i+3] as i32;
let v = input[ip+i+4] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+4] as i32;
a1 += v * weights[((c+1)*7+wp)+i+4] as i32;
a2 += v * weights[((c+2)*7+wp)+i+4] as i32;
a3 += v * weights[((c+3)*7+wp)+i+4] as i32;
let v = input[ip+i+5] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+5] as i32;
a1 += v * weights[((c+1)*7+wp)+i+5] as i32;
a2 += v * weights[((c+2)*7+wp)+i+5] as i32;
a3 += v * weights[((c+3)*7+wp)+i+5] as i32;
let v = input[ip+i+6] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+6] as i32;
a1 += v * weights[((c+1)*7+wp)+i+6] as i32;
a2 += v * weights[((c+2)*7+wp)+i+6] as i32;
a3 += v * weights[((c+3)*7+wp)+i+6] as i32;
let v = input[ip+i+7] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+7] as i32;
a1 += v * weights[((c+1)*7+wp)+i+7] as i32;
a2 += v * weights[((c+2)*7+wp)+i+7] as i32;
a3 += v * weights[((c+3)*7+wp)+i+7] as i32;
let v = input[ip+i+8] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+8] as i32;
a1 += v * weights[((c+1)*7+wp)+i+8] as i32;
a2 += v * weights[((c+2)*7+wp)+i+8] as i32;
a3 += v * weights[((c+3)*7+wp)+i+8] as i32;
let v = input[ip+i+9] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+9] as i32;
a1 += v * weights[((c+1)*7+wp)+i+9] as i32;
a2 += v * weights[((c+2)*7+wp)+i+9] as i32;
a3 += v * weights[((c+3)*7+wp)+i+9] as i32;
let v = input[ip+i+10] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+10] as i32;
a1 += v * weights[((c+1)*7+wp)+i+10] as i32;
a2 += v * weights[((c+2)*7+wp)+i+10] as i32;
a3 += v * weights[((c+3)*7+wp)+i+10] as i32;
let v = input[ip+i+11] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+11] as i32;
a1 += v * weights[((c+1)*7+wp)+i+11] as i32;
a2 += v * weights[((c+2)*7+wp)+i+11] as i32;
a3 += v * weights[((c+3)*7+wp)+i+11] as i32;
let v = input[ip+i+12] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+12] as i32;
a1 += v * weights[((c+1)*7+wp)+i+12] as i32;
a2 += v * weights[((c+2)*7+wp)+i+12] as i32;
a3 += v * weights[((c+3)*7+wp)+i+12] as i32;
let v = input[ip+i+13] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+13] as i32;
a1 += v * weights[((c+1)*7+wp)+i+13] as i32;
a2 += v * weights[((c+2)*7+wp)+i+13] as i32;
a3 += v * weights[((c+3)*7+wp)+i+13] as i32;
let v = input[ip+i+14] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+14] as i32;
a1 += v * weights[((c+1)*7+wp)+i+14] as i32;
a2 += v * weights[((c+2)*7+wp)+i+14] as i32;
a3 += v * weights[((c+3)*7+wp)+i+14] as i32;
let v = input[ip+i+15] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+15] as i32;
a1 += v * weights[((c+1)*7+wp)+i+15] as i32;
a2 += v * weights[((c+2)*7+wp)+i+15] as i32;
a3 += v * weights[((c+3)*7+wp)+i+15] as i32;
i+=16; }
let v = input[ip+0] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+0] as i32;
a1 += v * weights[((c+1)*7+wp)+0] as i32;
a2 += v * weights[((c+2)*7+wp)+0] as i32;
a3 += v * weights[((c+3)*7+wp)+0] as i32;
let v = input[ip+1] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+1] as i32;
a1 += v * weights[((c+1)*7+wp)+1] as i32;
a2 += v * weights[((c+2)*7+wp)+1] as i32;
a3 += v * weights[((c+3)*7+wp)+1] as i32;
let v = input[ip+2] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+2] as i32;
a1 += v * weights[((c+1)*7+wp)+2] as i32;
a2 += v * weights[((c+2)*7+wp)+2] as i32;
a3 += v * weights[((c+3)*7+wp)+2] as i32;
let v = input[ip+3] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+3] as i32;
a1 += v * weights[((c+1)*7+wp)+3] as i32;
a2 += v * weights[((c+2)*7+wp)+3] as i32;
a3 += v * weights[((c+3)*7+wp)+3] as i32;
let v = input[ip+4] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+4] as i32;
a1 += v * weights[((c+1)*7+wp)+4] as i32;
a2 += v * weights[((c+2)*7+wp)+4] as i32;
a3 += v * weights[((c+3)*7+wp)+4] as i32;
let v = input[ip+5] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+5] as i32;
a1 += v * weights[((c+1)*7+wp)+5] as i32;
a2 += v * weights[((c+2)*7+wp)+5] as i32;
a3 += v * weights[((c+3)*7+wp)+5] as i32;
let v = input[ip+6] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+6] as i32;
a1 += v * weights[((c+1)*7+wp)+6] as i32;
a2 += v * weights[((c+2)*7+wp)+6] as i32;
a3 += v * weights[((c+3)*7+wp)+6] as i32;
} }
output[(y*7+x)*9+c+0] = (requantize(a0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*7+x)*9+c+1] = (requantize(a1,mult[c+1],shift[c+1])+5).clamp(-111,103) as i8;
output[(y*7+x)*9+c+2] = (requantize(a2,mult[c+2],shift[c+2])+5).clamp(-111,103) as i8;
output[(y*7+x)*9+c+3] = (requantize(a3,mult[c+3],shift[c+3])+5).clamp(-111,103) as i8;
c+=4; }
while c<9 {
let mut a0=bias[c+0];
let by=(y*1) as isize-0; let bx=(x*1) as isize-0;
let ys=0isize.max(-by) as usize; let ye=(1isize).min(5isize-by) as usize;
let xs=0isize.max(-bx) as usize; let xe=(1isize).min(7isize-bx) as usize;
for ky in ys..ye { for kx in xs..xe {
let ip=(((by+ky as isize) as usize)*7+(bx+kx as isize) as usize)*7; let wp=(ky*1+kx)*7;
let mut i=0usize; while i < 0 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+3] as i32;
let v = input[ip+i+4] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+4] as i32;
let v = input[ip+i+5] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+5] as i32;
let v = input[ip+i+6] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+6] as i32;
let v = input[ip+i+7] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+7] as i32;
let v = input[ip+i+8] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+8] as i32;
let v = input[ip+i+9] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+9] as i32;
let v = input[ip+i+10] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+10] as i32;
let v = input[ip+i+11] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+11] as i32;
let v = input[ip+i+12] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+12] as i32;
let v = input[ip+i+13] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+13] as i32;
let v = input[ip+i+14] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+14] as i32;
let v = input[ip+i+15] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+i+15] as i32;
i+=16; }
let v = input[ip+0] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+0] as i32;
let v = input[ip+1] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+1] as i32;
let v = input[ip+2] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+2] as i32;
let v = input[ip+3] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+3] as i32;
let v = input[ip+4] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+4] as i32;
let v = input[ip+5] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+5] as i32;
let v = input[ip+6] as i32 + -3;
a0 += v * weights[((c+0)*7+wp)+6] as i32;
} }
output[(y*7+x)*9+c+0] = (requantize(a0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
c+=1; }
} } }
/// # Safety
/// Pointers must be aligned, valid for lengths 245, 63, 9, 9, 9, 315 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn kernel(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,245),core::slice::from_raw_parts(w,63),core::slice::from_raw_parts(b,9),core::slice::from_raw_parts(m,9),core::slice::from_raw_parts(s,9),core::slice::from_raw_parts_mut(o,315)); }
