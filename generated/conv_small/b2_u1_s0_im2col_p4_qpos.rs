#![no_std]
// Generated for conv_small with b2_u1_s0_im2col_p4_qpos. Scratch: 216 bytes (local stack partial im2col, included in watermark).
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
let mut columns=[0i16;108];
for y in 0..4 { let mut x=0usize; while x<4 {
for ky in 0..3 { for kx in 0..3 { let iy=(y*2+ky) as isize-1; let ix=((x+0)*2+kx) as isize-1; for i in 0..3 {
columns[0*27+(ky*3+kx)*3+i]=if iy>=0 && iy<7 && ix>=0 && ix<9 {(input[(iy as usize*9+ix as usize)*3+i] as i32+-3) as i16}else{0}; } } }
for ky in 0..3 { for kx in 0..3 { let iy=(y*2+ky) as isize-1; let ix=((x+1)*2+kx) as isize-1; for i in 0..3 {
columns[1*27+(ky*3+kx)*3+i]=if iy>=0 && iy<7 && ix>=0 && ix<9 {(input[(iy as usize*9+ix as usize)*3+i] as i32+-3) as i16}else{0}; } } }
for ky in 0..3 { for kx in 0..3 { let iy=(y*2+ky) as isize-1; let ix=((x+2)*2+kx) as isize-1; for i in 0..3 {
columns[2*27+(ky*3+kx)*3+i]=if iy>=0 && iy<7 && ix>=0 && ix<9 {(input[(iy as usize*9+ix as usize)*3+i] as i32+-3) as i16}else{0}; } } }
for ky in 0..3 { for kx in 0..3 { let iy=(y*2+ky) as isize-1; let ix=((x+3)*2+kx) as isize-1; for i in 0..3 {
columns[3*27+(ky*3+kx)*3+i]=if iy>=0 && iy<7 && ix>=0 && ix<9 {(input[(iy as usize*9+ix as usize)*3+i] as i32+-3) as i16}else{0}; } } }
let mut c=0usize; while c<6 {
let base0=bias[c+0];
let mut a0_0=base0;
let mut a1_0=base0;
let mut a2_0=base0;
let mut a3_0=base0;
let base1=bias[c+1];
let mut a0_1=base1;
let mut a1_1=base1;
let mut a2_1=base1;
let mut a3_1=base1;
let mut i=0usize; while i<27 {
{
let v0=columns[0*27+i+0] as i32;
let v1=columns[1*27+i+0] as i32;
let v2=columns[2*27+i+0] as i32;
let v3=columns[3*27+i+0] as i32;
let wb=weights[(c+0)*27+i+0] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
a2_0 += v2*wb;
a3_0 += v3*wb;
let wb=weights[(c+1)*27+i+0] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
a2_1 += v2*wb;
a3_1 += v3*wb;
}
i+=1; }
output[(y*5+x+0)*7+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*5+x+1)*7+c+0]=(requantize(a1_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*5+x+2)*7+c+0]=(requantize(a2_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*5+x+3)*7+c+0]=(requantize(a3_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*5+x+0)*7+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+5).clamp(-111,103) as i8;
output[(y*5+x+1)*7+c+1]=(requantize(a1_1,mult[c+1],shift[c+1])+5).clamp(-111,103) as i8;
output[(y*5+x+2)*7+c+1]=(requantize(a2_1,mult[c+1],shift[c+1])+5).clamp(-111,103) as i8;
output[(y*5+x+3)*7+c+1]=(requantize(a3_1,mult[c+1],shift[c+1])+5).clamp(-111,103) as i8;
c+=2; }
while c<7 {
let base0=bias[c+0];
let mut a0_0=base0;
let mut a1_0=base0;
let mut a2_0=base0;
let mut a3_0=base0;
let mut i=0usize; while i<27 {
{
let v0=columns[0*27+i+0] as i32;
let v1=columns[1*27+i+0] as i32;
let v2=columns[2*27+i+0] as i32;
let v3=columns[3*27+i+0] as i32;
let wb=weights[(c+0)*27+i+0] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
a2_0 += v2*wb;
a3_0 += v3*wb;
}
i+=1; }
output[(y*5+x+0)*7+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*5+x+1)*7+c+0]=(requantize(a1_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*5+x+2)*7+c+0]=(requantize(a2_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*5+x+3)*7+c+0]=(requantize(a3_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
c+=1; }
x+=4; }
while x<5 {
for ky in 0..3 { for kx in 0..3 { let iy=(y*2+ky) as isize-1; let ix=((x+0)*2+kx) as isize-1; for i in 0..3 {
columns[0*27+(ky*3+kx)*3+i]=if iy>=0 && iy<7 && ix>=0 && ix<9 {(input[(iy as usize*9+ix as usize)*3+i] as i32+-3) as i16}else{0}; } } }
let mut c=0usize; while c<6 {
let base0=bias[c+0];
let mut a0_0=base0;
let base1=bias[c+1];
let mut a0_1=base1;
let mut i=0usize; while i<27 {
{
let v0=columns[0*27+i+0] as i32;
let wb=weights[(c+0)*27+i+0] as i32;
a0_0 += v0*wb;
let wb=weights[(c+1)*27+i+0] as i32;
a0_1 += v0*wb;
}
i+=1; }
output[(y*5+x+0)*7+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*5+x+0)*7+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+5).clamp(-111,103) as i8;
c+=2; }
while c<7 {
let base0=bias[c+0];
let mut a0_0=base0;
let mut i=0usize; while i<27 {
{
let v0=columns[0*27+i+0] as i32;
let wb=weights[(c+0)*27+i+0] as i32;
a0_0 += v0*wb;
}
i+=1; }
output[(y*5+x+0)*7+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
c+=1; }
x+=1; }
} }
/// # Safety
/// Pointers must be aligned, valid for lengths 189, 189, 7, 7, 7, 140 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn kernel(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,189),core::slice::from_raw_parts(w,189),core::slice::from_raw_parts(b,7),core::slice::from_raw_parts(m,7),core::slice::from_raw_parts(s,7),core::slice::from_raw_parts_mut(o,140)); }
