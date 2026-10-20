#![no_std]
// Generated for pointwise with b2_u8_s0. Scratch: 0 bytes.
// Input/weight/output lengths: 8192, 1024, 8192.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    let x = x.wrapping_shl(shift.max(0) as u32);
    let overflow = x == i32::MIN && multiplier == i32::MIN;
    let nudge = if (x < 0) == (multiplier < 0) {
        1i64 << 30
    } else {
        1 - (1i64 << 30)
    };
    let product = i64::from(x) * i64::from(multiplier) + nudge;
    let high = if overflow {
        i32::MAX
    } else {
        (product / (1i64 << 31)) as i32
    };
    let exponent = (-shift).max(0) as u32;
    let mask = ((1u32 << exponent) - 1) as i32;
    let base = high >> exponent;
    base + i32::from((high & mask) > ((mask >> 1) + i32::from(base < 0)))
}


/// All buffers are caller owned. INT32 accumulation and shifted values must fit.
#[inline(never)]
pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=8192 && weights.len()>=1024 && output.len()>=8192);
assert!(bias.len()>=32 && mult.len()>=32 && shift.len()>=32);
for y in 0..16 { for x in 0..16 {
let mut c=0usize; while c<32 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let by=(y*1) as isize-0; let bx=(x*1) as isize-0;
let ys=0isize.max(-by) as usize; let ye=(1isize).min(16isize-by) as usize;
let xs=0isize.max(-bx) as usize; let xe=(1isize).min(16isize-bx) as usize;
for ky in ys..ye { for kx in xs..xe {
let ip=(((by+ky as isize) as usize)*16+(bx+kx as isize) as usize)*32; let wp=(ky*1+kx)*32;
let mut i=0usize; while i < 32 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*32+wp)+i+0] as i32;
a1 += v * weights[((c+1)*32+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*32+wp)+i+1] as i32;
a1 += v * weights[((c+1)*32+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*32+wp)+i+2] as i32;
a1 += v * weights[((c+1)*32+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*32+wp)+i+3] as i32;
a1 += v * weights[((c+1)*32+wp)+i+3] as i32;
let v = input[ip+i+4] as i32 + -3;
a0 += v * weights[((c+0)*32+wp)+i+4] as i32;
a1 += v * weights[((c+1)*32+wp)+i+4] as i32;
let v = input[ip+i+5] as i32 + -3;
a0 += v * weights[((c+0)*32+wp)+i+5] as i32;
a1 += v * weights[((c+1)*32+wp)+i+5] as i32;
let v = input[ip+i+6] as i32 + -3;
a0 += v * weights[((c+0)*32+wp)+i+6] as i32;
a1 += v * weights[((c+1)*32+wp)+i+6] as i32;
let v = input[ip+i+7] as i32 + -3;
a0 += v * weights[((c+0)*32+wp)+i+7] as i32;
a1 += v * weights[((c+1)*32+wp)+i+7] as i32;
i+=8; }
} }
output[(y*16+x)*32+c+0] = (requantize(a0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*16+x)*32+c+1] = (requantize(a1,mult[c+1],shift[c+1])+5).clamp(-111,103) as i8;
c+=2; }
} } }
/// # Safety
/// Pointers must be aligned, valid for lengths 8192, 1024, 32, 32, 32, 8192 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn kernel(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,8192),core::slice::from_raw_parts(w,1024),core::slice::from_raw_parts(b,32),core::slice::from_raw_parts(m,32),core::slice::from_raw_parts(s,32),core::slice::from_raw_parts_mut(o,8192)); }
