#![no_std]
// Generated for depthwise_small with b4_u1_s0. Scratch: 0 bytes.
// Input/weight/output lengths: 315, 45, 100.
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
assert!(input.len()>=315 && weights.len()>=45 && output.len()>=100);
assert!(bias.len()>=5 && mult.len()>=5 && shift.len()>=5);
for y in 0..5 { for x in 0..4 {
let mut c=0usize; while c<4 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
let by=(y*2) as isize-1; let bx=(x*2) as isize-1;
let ys=0isize.max(-by) as usize; let ye=(3isize).min(9isize-by) as usize;
let xs=0isize.max(-bx) as usize; let xe=(3isize).min(7isize-bx) as usize;
for ky in ys..ye { for kx in xs..xe {
let ip=(((by+ky as isize) as usize)*7+(bx+kx as isize) as usize)*5; let wp=(ky*3+kx)*5;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + -3) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + -3) * weights[wp+c+3] as i32;
} }
output[(y*4+x)*5+c+0] = (requantize(a0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*4+x)*5+c+1] = (requantize(a1,mult[c+1],shift[c+1])+5).clamp(-111,103) as i8;
output[(y*4+x)*5+c+2] = (requantize(a2,mult[c+2],shift[c+2])+5).clamp(-111,103) as i8;
output[(y*4+x)*5+c+3] = (requantize(a3,mult[c+3],shift[c+3])+5).clamp(-111,103) as i8;
c+=4; }
while c<5 {
let mut a0=bias[c+0];
let by=(y*2) as isize-1; let bx=(x*2) as isize-1;
let ys=0isize.max(-by) as usize; let ye=(3isize).min(9isize-by) as usize;
let xs=0isize.max(-bx) as usize; let xe=(3isize).min(7isize-bx) as usize;
for ky in ys..ye { for kx in xs..xe {
let ip=(((by+ky as isize) as usize)*7+(bx+kx as isize) as usize)*5; let wp=(ky*3+kx)*5;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
} }
output[(y*4+x)*5+c+0] = (requantize(a0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
c+=1; }
} } }
/// # Safety
/// Pointers must be aligned, valid for lengths 315, 45, 5, 5, 5, 100 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn kernel(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,315),core::slice::from_raw_parts(w,45),core::slice::from_raw_parts(b,5),core::slice::from_raw_parts(m,5),core::slice::from_raw_parts(s,5),core::slice::from_raw_parts_mut(o,100)); }
