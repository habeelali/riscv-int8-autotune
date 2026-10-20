#![no_std]
// Generated for conv_small with b2_u16_s0_im2col. Scratch: 54 bytes (local stack partial im2col, included in watermark).
// Input/weight/output lengths: 189, 189, 140.
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
assert!(input.len()>=189 && weights.len()>=189 && output.len()>=140);
assert!(bias.len()>=7 && mult.len()>=7 && shift.len()>=7);
let mut column=[0i16;27];
for y in 0..4 { for x in 0..5 {
for ky in 0..3 { for kx in 0..3 { let iy=(y*2+ky) as isize-1; let ix=(x*2+kx) as isize-1; for i in 0..3 {
column[(ky*3+kx)*3+i]=if iy>=0 && iy<7 && ix>=0 && ix<9 {(input[(iy as usize*9+ix as usize)*3+i] as i32+-3) as i16}else{0}; } } }
let mut c=0usize; while c<6 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut i=0usize; while i<16 {
let v=column[i+0] as i32;
a0 += v * weights[(c+0)*27+i+0] as i32;
a1 += v * weights[(c+1)*27+i+0] as i32;
let v=column[i+1] as i32;
a0 += v * weights[(c+0)*27+i+1] as i32;
a1 += v * weights[(c+1)*27+i+1] as i32;
let v=column[i+2] as i32;
a0 += v * weights[(c+0)*27+i+2] as i32;
a1 += v * weights[(c+1)*27+i+2] as i32;
let v=column[i+3] as i32;
a0 += v * weights[(c+0)*27+i+3] as i32;
a1 += v * weights[(c+1)*27+i+3] as i32;
let v=column[i+4] as i32;
a0 += v * weights[(c+0)*27+i+4] as i32;
a1 += v * weights[(c+1)*27+i+4] as i32;
let v=column[i+5] as i32;
a0 += v * weights[(c+0)*27+i+5] as i32;
a1 += v * weights[(c+1)*27+i+5] as i32;
let v=column[i+6] as i32;
a0 += v * weights[(c+0)*27+i+6] as i32;
a1 += v * weights[(c+1)*27+i+6] as i32;
let v=column[i+7] as i32;
a0 += v * weights[(c+0)*27+i+7] as i32;
a1 += v * weights[(c+1)*27+i+7] as i32;
let v=column[i+8] as i32;
a0 += v * weights[(c+0)*27+i+8] as i32;
a1 += v * weights[(c+1)*27+i+8] as i32;
let v=column[i+9] as i32;
a0 += v * weights[(c+0)*27+i+9] as i32;
a1 += v * weights[(c+1)*27+i+9] as i32;
let v=column[i+10] as i32;
a0 += v * weights[(c+0)*27+i+10] as i32;
a1 += v * weights[(c+1)*27+i+10] as i32;
let v=column[i+11] as i32;
a0 += v * weights[(c+0)*27+i+11] as i32;
a1 += v * weights[(c+1)*27+i+11] as i32;
let v=column[i+12] as i32;
a0 += v * weights[(c+0)*27+i+12] as i32;
a1 += v * weights[(c+1)*27+i+12] as i32;
let v=column[i+13] as i32;
a0 += v * weights[(c+0)*27+i+13] as i32;
a1 += v * weights[(c+1)*27+i+13] as i32;
let v=column[i+14] as i32;
a0 += v * weights[(c+0)*27+i+14] as i32;
a1 += v * weights[(c+1)*27+i+14] as i32;
let v=column[i+15] as i32;
a0 += v * weights[(c+0)*27+i+15] as i32;
a1 += v * weights[(c+1)*27+i+15] as i32;
i+=16; }
let v=column[16] as i32;
a0 += v * weights[(c+0)*27+16] as i32;
a1 += v * weights[(c+1)*27+16] as i32;
let v=column[17] as i32;
a0 += v * weights[(c+0)*27+17] as i32;
a1 += v * weights[(c+1)*27+17] as i32;
let v=column[18] as i32;
a0 += v * weights[(c+0)*27+18] as i32;
a1 += v * weights[(c+1)*27+18] as i32;
let v=column[19] as i32;
a0 += v * weights[(c+0)*27+19] as i32;
a1 += v * weights[(c+1)*27+19] as i32;
let v=column[20] as i32;
a0 += v * weights[(c+0)*27+20] as i32;
a1 += v * weights[(c+1)*27+20] as i32;
let v=column[21] as i32;
a0 += v * weights[(c+0)*27+21] as i32;
a1 += v * weights[(c+1)*27+21] as i32;
let v=column[22] as i32;
a0 += v * weights[(c+0)*27+22] as i32;
a1 += v * weights[(c+1)*27+22] as i32;
let v=column[23] as i32;
a0 += v * weights[(c+0)*27+23] as i32;
a1 += v * weights[(c+1)*27+23] as i32;
let v=column[24] as i32;
a0 += v * weights[(c+0)*27+24] as i32;
a1 += v * weights[(c+1)*27+24] as i32;
let v=column[25] as i32;
a0 += v * weights[(c+0)*27+25] as i32;
a1 += v * weights[(c+1)*27+25] as i32;
let v=column[26] as i32;
a0 += v * weights[(c+0)*27+26] as i32;
a1 += v * weights[(c+1)*27+26] as i32;
output[(y*5+x)*7+c+0] = (requantize(a0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*5+x)*7+c+1] = (requantize(a1,mult[c+1],shift[c+1])+5).clamp(-111,103) as i8;
c+=2; }
while c<7 {
let mut a0=bias[c+0];
let mut i=0usize; while i<16 {
let v=column[i+0] as i32;
a0 += v * weights[(c+0)*27+i+0] as i32;
let v=column[i+1] as i32;
a0 += v * weights[(c+0)*27+i+1] as i32;
let v=column[i+2] as i32;
a0 += v * weights[(c+0)*27+i+2] as i32;
let v=column[i+3] as i32;
a0 += v * weights[(c+0)*27+i+3] as i32;
let v=column[i+4] as i32;
a0 += v * weights[(c+0)*27+i+4] as i32;
let v=column[i+5] as i32;
a0 += v * weights[(c+0)*27+i+5] as i32;
let v=column[i+6] as i32;
a0 += v * weights[(c+0)*27+i+6] as i32;
let v=column[i+7] as i32;
a0 += v * weights[(c+0)*27+i+7] as i32;
let v=column[i+8] as i32;
a0 += v * weights[(c+0)*27+i+8] as i32;
let v=column[i+9] as i32;
a0 += v * weights[(c+0)*27+i+9] as i32;
let v=column[i+10] as i32;
a0 += v * weights[(c+0)*27+i+10] as i32;
let v=column[i+11] as i32;
a0 += v * weights[(c+0)*27+i+11] as i32;
let v=column[i+12] as i32;
a0 += v * weights[(c+0)*27+i+12] as i32;
let v=column[i+13] as i32;
a0 += v * weights[(c+0)*27+i+13] as i32;
let v=column[i+14] as i32;
a0 += v * weights[(c+0)*27+i+14] as i32;
let v=column[i+15] as i32;
a0 += v * weights[(c+0)*27+i+15] as i32;
i+=16; }
let v=column[16] as i32;
a0 += v * weights[(c+0)*27+16] as i32;
let v=column[17] as i32;
a0 += v * weights[(c+0)*27+17] as i32;
let v=column[18] as i32;
a0 += v * weights[(c+0)*27+18] as i32;
let v=column[19] as i32;
a0 += v * weights[(c+0)*27+19] as i32;
let v=column[20] as i32;
a0 += v * weights[(c+0)*27+20] as i32;
let v=column[21] as i32;
a0 += v * weights[(c+0)*27+21] as i32;
let v=column[22] as i32;
a0 += v * weights[(c+0)*27+22] as i32;
let v=column[23] as i32;
a0 += v * weights[(c+0)*27+23] as i32;
let v=column[24] as i32;
a0 += v * weights[(c+0)*27+24] as i32;
let v=column[25] as i32;
a0 += v * weights[(c+0)*27+25] as i32;
let v=column[26] as i32;
a0 += v * weights[(c+0)*27+26] as i32;
output[(y*5+x)*7+c+0] = (requantize(a0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
c+=1; }
} } }
/// # Safety
/// Pointers must be aligned, valid for lengths 189, 189, 7, 7, 7, 140 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn kernel(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,189),core::slice::from_raw_parts(w,189),core::slice::from_raw_parts(b,7),core::slice::from_raw_parts(m,7),core::slice::from_raw_parts(s,7),core::slice::from_raw_parts_mut(o,140)); }
