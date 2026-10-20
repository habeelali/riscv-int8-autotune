#![no_std]
// Generated for conv3x3 with b1_u1_s0_im2col_p4. Scratch: 1152 bytes (local stack partial im2col, included in watermark).
// Input/weight/output lengths: 4096, 4608, 8192.
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
assert!(input.len()>=4096 && weights.len()>=4608 && output.len()>=8192);
assert!(bias.len()>=32 && mult.len()>=32 && shift.len()>=32);
let mut columns=[0i16;576];
for y in 0..16 { let mut x=0usize; while x<16 {
for ky in 0..3 { for kx in 0..3 { let iy=(y*1+ky) as isize-1; let ix=((x+0)*1+kx) as isize-1; for i in 0..16 {
columns[0*144+(ky*3+kx)*16+i]=if iy>=0 && iy<16 && ix>=0 && ix<16 {(input[(iy as usize*16+ix as usize)*16+i] as i32+-3) as i16}else{0}; } } }
for ky in 0..3 { for kx in 0..3 { let iy=(y*1+ky) as isize-1; let ix=((x+1)*1+kx) as isize-1; for i in 0..16 {
columns[1*144+(ky*3+kx)*16+i]=if iy>=0 && iy<16 && ix>=0 && ix<16 {(input[(iy as usize*16+ix as usize)*16+i] as i32+-3) as i16}else{0}; } } }
for ky in 0..3 { for kx in 0..3 { let iy=(y*1+ky) as isize-1; let ix=((x+2)*1+kx) as isize-1; for i in 0..16 {
columns[2*144+(ky*3+kx)*16+i]=if iy>=0 && iy<16 && ix>=0 && ix<16 {(input[(iy as usize*16+ix as usize)*16+i] as i32+-3) as i16}else{0}; } } }
for ky in 0..3 { for kx in 0..3 { let iy=(y*1+ky) as isize-1; let ix=((x+3)*1+kx) as isize-1; for i in 0..16 {
columns[3*144+(ky*3+kx)*16+i]=if iy>=0 && iy<16 && ix>=0 && ix<16 {(input[(iy as usize*16+ix as usize)*16+i] as i32+-3) as i16}else{0}; } } }
let mut c=0usize; while c<32 {
let base0=bias[c+0];
let mut a0_0=base0;
let mut a1_0=base0;
let mut a2_0=base0;
let mut a3_0=base0;
let mut i=0usize; while i<144 {
{
let v0=columns[0*144+i+0] as i32;
let v1=columns[1*144+i+0] as i32;
let v2=columns[2*144+i+0] as i32;
let v3=columns[3*144+i+0] as i32;
let wb=weights[(c+0)*144+i+0] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
a2_0 += v2*wb;
a3_0 += v3*wb;
}
i+=1; }
output[(y*16+x+0)*32+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*16+x+1)*32+c+0]=(requantize(a1_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*16+x+2)*32+c+0]=(requantize(a2_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*16+x+3)*32+c+0]=(requantize(a3_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
c+=1; }
x+=4; }
} }
/// # Safety
/// Pointers must be aligned, valid for lengths 4096, 4608, 32, 32, 32, 8192 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn kernel(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,4096),core::slice::from_raw_parts(w,4608),core::slice::from_raw_parts(b,32),core::slice::from_raw_parts(m,32),core::slice::from_raw_parts(s,32),core::slice::from_raw_parts_mut(o,8192)); }
