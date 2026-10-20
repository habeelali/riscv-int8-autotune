#![no_std]
// Generated for depthwise with b1_u1_s0_cxy. Scratch: 0 bytes.
// Input/weight/output lengths: 16384, 144, 16384.
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
assert!(input.len()>=16384 && weights.len()>=144 && output.len()>=16384);
assert!(bias.len()>=16 && mult.len()>=16 && shift.len()>=16);
let mut c=0usize; while c<16 { for y in 0..32 { for x in 0..32 {
let mut a0=bias[c+0];
let by=(y*1) as isize-1; let bx=(x*1) as isize-1;
let ys=0isize.max(-by) as usize; let ye=(3isize).min(32isize-by) as usize;
let xs=0isize.max(-bx) as usize; let xe=(3isize).min(32isize-bx) as usize;
for ky in ys..ye { for kx in xs..xe {
let ip=(((by+ky as isize) as usize)*32+(bx+kx as isize) as usize)*16; let wp=(ky*3+kx)*16;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
} }
output[(y*32+x)*16+c+0] = (requantize(a0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
} } c+=1; }
}
/// # Safety
/// Pointers must be aligned, valid for lengths 16384, 144, 16, 16, 16, 16384 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn kernel(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,16384),core::slice::from_raw_parts(w,144),core::slice::from_raw_parts(b,16),core::slice::from_raw_parts(m,16),core::slice::from_raw_parts(s,16),core::slice::from_raw_parts_mut(o,16384)); }
