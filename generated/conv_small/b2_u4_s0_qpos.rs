#![no_std]
// Generated for conv_small with b2_u4_s0_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 189, 189, 140.
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
assert!(input.len()>=189 && weights.len()>=189 && output.len()>=140);
assert!(bias.len()>=7 && mult.len()>=7 && shift.len()>=7);
assert!(mult[..7].iter().all(|&m|m>=0) && shift[..7].iter().all(|&q|(-30..=0).contains(&q)));
for y in 0..4 { for x in 0..5 {
let mut c=0usize; while c<6 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let by=(y*2) as isize-1; let bx=(x*2) as isize-1;
let ys=0isize.max(-by) as usize; let ye=(3isize).min(7isize-by) as usize;
let xs=0isize.max(-bx) as usize; let xe=(3isize).min(9isize-bx) as usize;
for ky in ys..ye { for kx in xs..xe {
let ip=(((by+ky as isize) as usize)*9+(bx+kx as isize) as usize)*3; let wp=(ky*3+kx)*3;
let mut i=0usize; while i < 0 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+0] as i32;
a1 += v * weights[((c+1)*27+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+1] as i32;
a1 += v * weights[((c+1)*27+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+2] as i32;
a1 += v * weights[((c+1)*27+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+3] as i32;
a1 += v * weights[((c+1)*27+wp)+i+3] as i32;
i+=4; }
let v = input[ip+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+0] as i32;
a1 += v * weights[((c+1)*27+wp)+0] as i32;
let v = input[ip+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+1] as i32;
a1 += v * weights[((c+1)*27+wp)+1] as i32;
let v = input[ip+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+2] as i32;
a1 += v * weights[((c+1)*27+wp)+2] as i32;
} }
output[(y*5+x)*7+c+0] = (requantize(a0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*5+x)*7+c+1] = (requantize(a1,mult[c+1],shift[c+1])+5).clamp(-111,103) as i8;
c+=2; }
while c<7 {
let mut a0=bias[c+0];
let by=(y*2) as isize-1; let bx=(x*2) as isize-1;
let ys=0isize.max(-by) as usize; let ye=(3isize).min(7isize-by) as usize;
let xs=0isize.max(-bx) as usize; let xe=(3isize).min(9isize-bx) as usize;
for ky in ys..ye { for kx in xs..xe {
let ip=(((by+ky as isize) as usize)*9+(bx+kx as isize) as usize)*3; let wp=(ky*3+kx)*3;
let mut i=0usize; while i < 0 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+3] as i32;
i+=4; }
let v = input[ip+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+0] as i32;
let v = input[ip+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+1] as i32;
let v = input[ip+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+2] as i32;
} }
output[(y*5+x)*7+c+0] = (requantize(a0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
c+=1; }
} } }
/// # Safety
/// Pointers must be aligned, valid for lengths 189, 189, 7, 7, 7, 140 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn kernel(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,189),core::slice::from_raw_parts(w,189),core::slice::from_raw_parts(b,7),core::slice::from_raw_parts(m,7),core::slice::from_raw_parts(s,7),core::slice::from_raw_parts_mut(o,140)); }
