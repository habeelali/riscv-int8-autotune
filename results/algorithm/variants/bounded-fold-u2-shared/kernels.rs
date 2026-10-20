#![no_std]
mod layer_0 {

// Generated for person_layer_0 with b8_u1_s0_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 9409, 72, 18432.
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


/// All buffers are caller owned. INT32 accumulation and shifted values must fit.
#[inline(never)]
pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=9409 && weights.len()>=72 && output.len()>=18432);
assert!(bias.len()>=8 && mult.len()>=8 && shift.len()>=8);
assert!(mult[..8].iter().all(|&m|m>=0) && shift[..8].iter().all(|&q|(-30..=-1).contains(&q)));
for y in 0..48 { for x in 0..48 {
let mut c=0usize; while c<8 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
let mut a4=bias[c+4];
let mut a5=bias[c+5];
let mut a6=bias[c+6];
let mut a7=bias[c+7];
let by=(y*2) as isize-0; let bx=(x*2) as isize-0;
let ys=0isize.max(-by) as usize; let ye=(3isize).min(97isize-by) as usize;
let xs=0isize.max(-bx) as usize; let xe=(3isize).min(97isize-bx) as usize;
for ky in ys..ye { for kx in xs..xe {
let ip=(((by+ky as isize) as usize)*97+(bx+kx as isize) as usize)*1; let wp=(ky*3+kx)*1;
let mut i=0usize; while i < 1 {
let v = input[ip+i+0] as i32 + 0;
a0 += v * weights[((c+0)*9+wp)+i+0] as i32;
a1 += v * weights[((c+1)*9+wp)+i+0] as i32;
a2 += v * weights[((c+2)*9+wp)+i+0] as i32;
a3 += v * weights[((c+3)*9+wp)+i+0] as i32;
a4 += v * weights[((c+4)*9+wp)+i+0] as i32;
a5 += v * weights[((c+5)*9+wp)+i+0] as i32;
a6 += v * weights[((c+6)*9+wp)+i+0] as i32;
a7 += v * weights[((c+7)*9+wp)+i+0] as i32;
i+=1; }
} }
output[(y*48+x)*8+c+0] = (requantize(a0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*48+x)*8+c+1] = (requantize(a1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*48+x)*8+c+2] = (requantize(a2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*48+x)*8+c+3] = (requantize(a3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
output[(y*48+x)*8+c+4] = (requantize(a4,mult[c+4],shift[c+4])+-128).clamp(-128,127) as i8;
output[(y*48+x)*8+c+5] = (requantize(a5,mult[c+5],shift[c+5])+-128).clamp(-128,127) as i8;
output[(y*48+x)*8+c+6] = (requantize(a6,mult[c+6],shift[c+6])+-128).clamp(-128,127) as i8;
output[(y*48+x)*8+c+7] = (requantize(a7,mult[c+7],shift[c+7])+-128).clamp(-128,127) as i8;
c+=8; }
} } }
/// # Safety
/// Pointers must be aligned, valid for lengths 9409, 72, 8, 8, 8, 18432 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_0(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,9409),core::slice::from_raw_parts(w,72),core::slice::from_raw_parts(b,8),core::slice::from_raw_parts(m,8),core::slice::from_raw_parts(s,8),core::slice::from_raw_parts_mut(o,18432)); }

}
mod layer_1 {

// Generated for person_layer_1 with b8_u1_s1_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 18432, 72, 18432.
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


/// All buffers are caller owned. INT32 accumulation and shifted values must fit.
#[inline(never)]
pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=18432 && weights.len()>=72 && output.len()>=18432);
assert!(bias.len()>=8 && mult.len()>=8 && shift.len()>=8);
assert!(mult[..8].iter().all(|&m|m>=0) && shift[..8].iter().all(|&q|(-30..=-1).contains(&q)));
for y in 0..48 { for x in 0..48 {
let mut c=0usize; while c<8 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
let mut a4=bias[c+4];
let mut a5=bias[c+5];
let mut a6=bias[c+6];
let mut a7=bias[c+7];
{ let iy=(y*1+0) as isize-1; let ix=(x*1+0) as isize-1;
if iy>=0 && iy<48 && ix>=0 && ix<48 {
let ip=(iy as usize*48+ix as usize)*8; let wp=0;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+0) as isize-1; let ix=(x*1+1) as isize-1;
if iy>=0 && iy<48 && ix>=0 && ix<48 {
let ip=(iy as usize*48+ix as usize)*8; let wp=8;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+0) as isize-1; let ix=(x*1+2) as isize-1;
if iy>=0 && iy<48 && ix>=0 && ix<48 {
let ip=(iy as usize*48+ix as usize)*8; let wp=16;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+1) as isize-1; let ix=(x*1+0) as isize-1;
if iy>=0 && iy<48 && ix>=0 && ix<48 {
let ip=(iy as usize*48+ix as usize)*8; let wp=24;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+1) as isize-1; let ix=(x*1+1) as isize-1;
if iy>=0 && iy<48 && ix>=0 && ix<48 {
let ip=(iy as usize*48+ix as usize)*8; let wp=32;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+1) as isize-1; let ix=(x*1+2) as isize-1;
if iy>=0 && iy<48 && ix>=0 && ix<48 {
let ip=(iy as usize*48+ix as usize)*8; let wp=40;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+2) as isize-1; let ix=(x*1+0) as isize-1;
if iy>=0 && iy<48 && ix>=0 && ix<48 {
let ip=(iy as usize*48+ix as usize)*8; let wp=48;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+2) as isize-1; let ix=(x*1+1) as isize-1;
if iy>=0 && iy<48 && ix>=0 && ix<48 {
let ip=(iy as usize*48+ix as usize)*8; let wp=56;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+2) as isize-1; let ix=(x*1+2) as isize-1;
if iy>=0 && iy<48 && ix>=0 && ix<48 {
let ip=(iy as usize*48+ix as usize)*8; let wp=64;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
output[(y*48+x)*8+c+0] = (requantize(a0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*48+x)*8+c+1] = (requantize(a1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*48+x)*8+c+2] = (requantize(a2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*48+x)*8+c+3] = (requantize(a3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
output[(y*48+x)*8+c+4] = (requantize(a4,mult[c+4],shift[c+4])+-128).clamp(-128,127) as i8;
output[(y*48+x)*8+c+5] = (requantize(a5,mult[c+5],shift[c+5])+-128).clamp(-128,127) as i8;
output[(y*48+x)*8+c+6] = (requantize(a6,mult[c+6],shift[c+6])+-128).clamp(-128,127) as i8;
output[(y*48+x)*8+c+7] = (requantize(a7,mult[c+7],shift[c+7])+-128).clamp(-128,127) as i8;
c+=8; }
} } }
/// # Safety
/// Pointers must be aligned, valid for lengths 18432, 72, 8, 8, 8, 18432 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_1(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,18432),core::slice::from_raw_parts(w,72),core::slice::from_raw_parts(b,8),core::slice::from_raw_parts(m,8),core::slice::from_raw_parts(s,8),core::slice::from_raw_parts_mut(o,18432)); }

}
mod layer_2 {

// Generated for person_layer_2 with b2_u1_s0_p4_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 18432, 128, 36864.
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


/// All buffers are caller owned. INT32 accumulation and shifted values must fit.
#[inline(never)]
pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=18432 && weights.len()>=128 && output.len()>=36864);
assert!(bias.len()>=16 && mult.len()>=16 && shift.len()>=16);
assert!(mult[..16].iter().all(|&m|m>=0) && shift[..16].iter().all(|&q|(-30..=-1).contains(&q)));
for y in 0..48 { let mut x=0usize; while x<48 {
let mut c=0usize; while c<16 {
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
let mut i=0usize; while i<8 {
{
let v0=input[(y*1*48+(x+0)*1)*8+i+0] as i32+0;
let v1=input[(y*1*48+(x+1)*1)*8+i+0] as i32+0;
let v2=input[(y*1*48+(x+2)*1)*8+i+0] as i32+0;
let v3=input[(y*1*48+(x+3)*1)*8+i+0] as i32+0;
let wb=weights[(c+0)*8+i+0] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
a2_0 += v2*wb;
a3_0 += v3*wb;
let wb=weights[(c+1)*8+i+0] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
a2_1 += v2*wb;
a3_1 += v3*wb;
}
i+=1; }
output[(y*48+x+0)*16+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*48+x+1)*16+c+0]=(requantize(a1_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*48+x+2)*16+c+0]=(requantize(a2_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*48+x+3)*16+c+0]=(requantize(a3_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*48+x+0)*16+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*48+x+1)*16+c+1]=(requantize(a1_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*48+x+2)*16+c+1]=(requantize(a2_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*48+x+3)*16+c+1]=(requantize(a3_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
c+=2; }
x+=4; }
} }
/// # Safety
/// Pointers must be aligned, valid for lengths 18432, 128, 16, 16, 16, 36864 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_2(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,18432),core::slice::from_raw_parts(w,128),core::slice::from_raw_parts(b,16),core::slice::from_raw_parts(m,16),core::slice::from_raw_parts(s,16),core::slice::from_raw_parts_mut(o,36864)); }

}
mod layer_3 {

// Generated for person_layer_3 with b1_u1_s1_cxy_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 38416, 144, 9216.
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


/// All buffers are caller owned. INT32 accumulation and shifted values must fit.
#[inline(never)]
pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=38416 && weights.len()>=144 && output.len()>=9216);
assert!(bias.len()>=16 && mult.len()>=16 && shift.len()>=16);
assert!(mult[..16].iter().all(|&m|m>=0) && shift[..16].iter().all(|&q|(-30..=-1).contains(&q)));
let mut c=0usize; while c<16 { for y in 0..24 { for x in 0..24 {
let mut a0=bias[c+0];
{ let iy=(y*2+0) as isize-0; let ix=(x*2+0) as isize-0;
if iy>=0 && iy<49 && ix>=0 && ix<49 {
let ip=(iy as usize*49+ix as usize)*16; let wp=0;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+0) as isize-0; let ix=(x*2+1) as isize-0;
if iy>=0 && iy<49 && ix>=0 && ix<49 {
let ip=(iy as usize*49+ix as usize)*16; let wp=16;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+0) as isize-0; let ix=(x*2+2) as isize-0;
if iy>=0 && iy<49 && ix>=0 && ix<49 {
let ip=(iy as usize*49+ix as usize)*16; let wp=32;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+1) as isize-0; let ix=(x*2+0) as isize-0;
if iy>=0 && iy<49 && ix>=0 && ix<49 {
let ip=(iy as usize*49+ix as usize)*16; let wp=48;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+1) as isize-0; let ix=(x*2+1) as isize-0;
if iy>=0 && iy<49 && ix>=0 && ix<49 {
let ip=(iy as usize*49+ix as usize)*16; let wp=64;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+1) as isize-0; let ix=(x*2+2) as isize-0;
if iy>=0 && iy<49 && ix>=0 && ix<49 {
let ip=(iy as usize*49+ix as usize)*16; let wp=80;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+2) as isize-0; let ix=(x*2+0) as isize-0;
if iy>=0 && iy<49 && ix>=0 && ix<49 {
let ip=(iy as usize*49+ix as usize)*16; let wp=96;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+2) as isize-0; let ix=(x*2+1) as isize-0;
if iy>=0 && iy<49 && ix>=0 && ix<49 {
let ip=(iy as usize*49+ix as usize)*16; let wp=112;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+2) as isize-0; let ix=(x*2+2) as isize-0;
if iy>=0 && iy<49 && ix>=0 && ix<49 {
let ip=(iy as usize*49+ix as usize)*16; let wp=128;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
output[(y*24+x)*16+c+0] = (requantize(a0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
} } c+=1; }
}
/// # Safety
/// Pointers must be aligned, valid for lengths 38416, 144, 16, 16, 16, 9216 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_3(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,38416),core::slice::from_raw_parts(w,144),core::slice::from_raw_parts(b,16),core::slice::from_raw_parts(m,16),core::slice::from_raw_parts(s,16),core::slice::from_raw_parts_mut(o,9216)); }

}
mod layer_4 {

// Generated for person_layer_4 with b4_u1_s0_p2_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 9216, 512, 18432.
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


/// All buffers are caller owned. INT32 accumulation and shifted values must fit.
#[inline(never)]
pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=9216 && weights.len()>=512 && output.len()>=18432);
assert!(bias.len()>=32 && mult.len()>=32 && shift.len()>=32);
assert!(mult[..32].iter().all(|&m|m>=0) && shift[..32].iter().all(|&q|(-30..=-1).contains(&q)));
for y in 0..24 { let mut x=0usize; while x<24 {
let mut c=0usize; while c<32 {
let base0=bias[c+0];
let mut a0_0=base0;
let mut a1_0=base0;
let base1=bias[c+1];
let mut a0_1=base1;
let mut a1_1=base1;
let base2=bias[c+2];
let mut a0_2=base2;
let mut a1_2=base2;
let base3=bias[c+3];
let mut a0_3=base3;
let mut a1_3=base3;
let mut i=0usize; while i<16 {
{
let v0=input[(y*1*24+(x+0)*1)*16+i+0] as i32+0;
let v1=input[(y*1*24+(x+1)*1)*16+i+0] as i32+0;
let wb=weights[(c+0)*16+i+0] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let wb=weights[(c+1)*16+i+0] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
let wb=weights[(c+2)*16+i+0] as i32;
a0_2 += v0*wb;
a1_2 += v1*wb;
let wb=weights[(c+3)*16+i+0] as i32;
a0_3 += v0*wb;
a1_3 += v1*wb;
}
i+=1; }
output[(y*24+x+0)*32+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*24+x+1)*32+c+0]=(requantize(a1_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*24+x+0)*32+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*24+x+1)*32+c+1]=(requantize(a1_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*24+x+0)*32+c+2]=(requantize(a0_2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*24+x+1)*32+c+2]=(requantize(a1_2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*24+x+0)*32+c+3]=(requantize(a0_3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
output[(y*24+x+1)*32+c+3]=(requantize(a1_3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
c+=4; }
x+=2; }
} }
/// # Safety
/// Pointers must be aligned, valid for lengths 9216, 512, 32, 32, 32, 18432 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_4(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,9216),core::slice::from_raw_parts(w,512),core::slice::from_raw_parts(b,32),core::slice::from_raw_parts(m,32),core::slice::from_raw_parts(s,32),core::slice::from_raw_parts_mut(o,18432)); }

}
mod layer_5 {

// Generated for person_layer_5 with b4_u1_s1_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 18432, 288, 18432.
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


/// All buffers are caller owned. INT32 accumulation and shifted values must fit.
#[inline(never)]
pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=18432 && weights.len()>=288 && output.len()>=18432);
assert!(bias.len()>=32 && mult.len()>=32 && shift.len()>=32);
assert!(mult[..32].iter().all(|&m|m>=0) && shift[..32].iter().all(|&q|(-30..=-1).contains(&q)));
for y in 0..24 { for x in 0..24 {
let mut c=0usize; while c<32 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
{ let iy=(y*1+0) as isize-1; let ix=(x*1+0) as isize-1;
if iy>=0 && iy<24 && ix>=0 && ix<24 {
let ip=(iy as usize*24+ix as usize)*32; let wp=0;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
} }
{ let iy=(y*1+0) as isize-1; let ix=(x*1+1) as isize-1;
if iy>=0 && iy<24 && ix>=0 && ix<24 {
let ip=(iy as usize*24+ix as usize)*32; let wp=32;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
} }
{ let iy=(y*1+0) as isize-1; let ix=(x*1+2) as isize-1;
if iy>=0 && iy<24 && ix>=0 && ix<24 {
let ip=(iy as usize*24+ix as usize)*32; let wp=64;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
} }
{ let iy=(y*1+1) as isize-1; let ix=(x*1+0) as isize-1;
if iy>=0 && iy<24 && ix>=0 && ix<24 {
let ip=(iy as usize*24+ix as usize)*32; let wp=96;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
} }
{ let iy=(y*1+1) as isize-1; let ix=(x*1+1) as isize-1;
if iy>=0 && iy<24 && ix>=0 && ix<24 {
let ip=(iy as usize*24+ix as usize)*32; let wp=128;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
} }
{ let iy=(y*1+1) as isize-1; let ix=(x*1+2) as isize-1;
if iy>=0 && iy<24 && ix>=0 && ix<24 {
let ip=(iy as usize*24+ix as usize)*32; let wp=160;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
} }
{ let iy=(y*1+2) as isize-1; let ix=(x*1+0) as isize-1;
if iy>=0 && iy<24 && ix>=0 && ix<24 {
let ip=(iy as usize*24+ix as usize)*32; let wp=192;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
} }
{ let iy=(y*1+2) as isize-1; let ix=(x*1+1) as isize-1;
if iy>=0 && iy<24 && ix>=0 && ix<24 {
let ip=(iy as usize*24+ix as usize)*32; let wp=224;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
} }
{ let iy=(y*1+2) as isize-1; let ix=(x*1+2) as isize-1;
if iy>=0 && iy<24 && ix>=0 && ix<24 {
let ip=(iy as usize*24+ix as usize)*32; let wp=256;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
} }
output[(y*24+x)*32+c+0] = (requantize(a0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*24+x)*32+c+1] = (requantize(a1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*24+x)*32+c+2] = (requantize(a2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*24+x)*32+c+3] = (requantize(a3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
c+=4; }
} } }
/// # Safety
/// Pointers must be aligned, valid for lengths 18432, 288, 32, 32, 32, 18432 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_5(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,18432),core::slice::from_raw_parts(w,288),core::slice::from_raw_parts(b,32),core::slice::from_raw_parts(m,32),core::slice::from_raw_parts(s,32),core::slice::from_raw_parts_mut(o,18432)); }

}
mod layer_6 {

// Generated for person_layer_6 with b4_u1_s0_p2_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 18432, 1024, 18432.
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


/// All buffers are caller owned. INT32 accumulation and shifted values must fit.
#[inline(never)]
pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=18432 && weights.len()>=1024 && output.len()>=18432);
assert!(bias.len()>=32 && mult.len()>=32 && shift.len()>=32);
assert!(mult[..32].iter().all(|&m|m>=0) && shift[..32].iter().all(|&q|(-30..=-1).contains(&q)));
for y in 0..24 { let mut x=0usize; while x<24 {
let mut c=0usize; while c<32 {
let base0=bias[c+0];
let mut a0_0=base0;
let mut a1_0=base0;
let base1=bias[c+1];
let mut a0_1=base1;
let mut a1_1=base1;
let base2=bias[c+2];
let mut a0_2=base2;
let mut a1_2=base2;
let base3=bias[c+3];
let mut a0_3=base3;
let mut a1_3=base3;
let mut i=0usize; while i<32 {
{
let v0=input[(y*1*24+(x+0)*1)*32+i+0] as i32+0;
let v1=input[(y*1*24+(x+1)*1)*32+i+0] as i32+0;
let wb=weights[(c+0)*32+i+0] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let wb=weights[(c+1)*32+i+0] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
let wb=weights[(c+2)*32+i+0] as i32;
a0_2 += v0*wb;
a1_2 += v1*wb;
let wb=weights[(c+3)*32+i+0] as i32;
a0_3 += v0*wb;
a1_3 += v1*wb;
}
i+=1; }
output[(y*24+x+0)*32+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*24+x+1)*32+c+0]=(requantize(a1_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*24+x+0)*32+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*24+x+1)*32+c+1]=(requantize(a1_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*24+x+0)*32+c+2]=(requantize(a0_2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*24+x+1)*32+c+2]=(requantize(a1_2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*24+x+0)*32+c+3]=(requantize(a0_3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
output[(y*24+x+1)*32+c+3]=(requantize(a1_3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
c+=4; }
x+=2; }
} }
/// # Safety
/// Pointers must be aligned, valid for lengths 18432, 1024, 32, 32, 32, 18432 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_6(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,18432),core::slice::from_raw_parts(w,1024),core::slice::from_raw_parts(b,32),core::slice::from_raw_parts(m,32),core::slice::from_raw_parts(s,32),core::slice::from_raw_parts_mut(o,18432)); }

}
mod layer_7 {

// Generated for person_layer_7 with b1_u1_s1_cxy_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 20000, 288, 4608.
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


/// All buffers are caller owned. INT32 accumulation and shifted values must fit.
#[inline(never)]
pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=20000 && weights.len()>=288 && output.len()>=4608);
assert!(bias.len()>=32 && mult.len()>=32 && shift.len()>=32);
assert!(mult[..32].iter().all(|&m|m>=0) && shift[..32].iter().all(|&q|(-30..=-1).contains(&q)));
let mut c=0usize; while c<32 { for y in 0..12 { for x in 0..12 {
let mut a0=bias[c+0];
{ let iy=(y*2+0) as isize-0; let ix=(x*2+0) as isize-0;
if iy>=0 && iy<25 && ix>=0 && ix<25 {
let ip=(iy as usize*25+ix as usize)*32; let wp=0;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+0) as isize-0; let ix=(x*2+1) as isize-0;
if iy>=0 && iy<25 && ix>=0 && ix<25 {
let ip=(iy as usize*25+ix as usize)*32; let wp=32;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+0) as isize-0; let ix=(x*2+2) as isize-0;
if iy>=0 && iy<25 && ix>=0 && ix<25 {
let ip=(iy as usize*25+ix as usize)*32; let wp=64;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+1) as isize-0; let ix=(x*2+0) as isize-0;
if iy>=0 && iy<25 && ix>=0 && ix<25 {
let ip=(iy as usize*25+ix as usize)*32; let wp=96;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+1) as isize-0; let ix=(x*2+1) as isize-0;
if iy>=0 && iy<25 && ix>=0 && ix<25 {
let ip=(iy as usize*25+ix as usize)*32; let wp=128;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+1) as isize-0; let ix=(x*2+2) as isize-0;
if iy>=0 && iy<25 && ix>=0 && ix<25 {
let ip=(iy as usize*25+ix as usize)*32; let wp=160;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+2) as isize-0; let ix=(x*2+0) as isize-0;
if iy>=0 && iy<25 && ix>=0 && ix<25 {
let ip=(iy as usize*25+ix as usize)*32; let wp=192;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+2) as isize-0; let ix=(x*2+1) as isize-0;
if iy>=0 && iy<25 && ix>=0 && ix<25 {
let ip=(iy as usize*25+ix as usize)*32; let wp=224;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+2) as isize-0; let ix=(x*2+2) as isize-0;
if iy>=0 && iy<25 && ix>=0 && ix<25 {
let ip=(iy as usize*25+ix as usize)*32; let wp=256;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
output[(y*12+x)*32+c+0] = (requantize(a0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
} } c+=1; }
}
/// # Safety
/// Pointers must be aligned, valid for lengths 20000, 288, 32, 32, 32, 4608 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_7(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,20000),core::slice::from_raw_parts(w,288),core::slice::from_raw_parts(b,32),core::slice::from_raw_parts(m,32),core::slice::from_raw_parts(s,32),core::slice::from_raw_parts_mut(o,4608)); }

}
mod layer_8 {

// Generated for person_layer_8 with b4_u1_s0_p2_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 4608, 2048, 9216.
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


/// All buffers are caller owned. INT32 accumulation and shifted values must fit.
#[inline(never)]
pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=4608 && weights.len()>=2048 && output.len()>=9216);
assert!(bias.len()>=64 && mult.len()>=64 && shift.len()>=64);
assert!(mult[..64].iter().all(|&m|m>=0) && shift[..64].iter().all(|&q|(-30..=-1).contains(&q)));
for y in 0..12 { let mut x=0usize; while x<12 {
let mut c=0usize; while c<64 {
let base0=bias[c+0];
let mut a0_0=base0;
let mut a1_0=base0;
let base1=bias[c+1];
let mut a0_1=base1;
let mut a1_1=base1;
let base2=bias[c+2];
let mut a0_2=base2;
let mut a1_2=base2;
let base3=bias[c+3];
let mut a0_3=base3;
let mut a1_3=base3;
let mut i=0usize; while i<32 {
{
let v0=input[(y*1*12+(x+0)*1)*32+i+0] as i32+0;
let v1=input[(y*1*12+(x+1)*1)*32+i+0] as i32+0;
let wb=weights[(c+0)*32+i+0] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let wb=weights[(c+1)*32+i+0] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
let wb=weights[(c+2)*32+i+0] as i32;
a0_2 += v0*wb;
a1_2 += v1*wb;
let wb=weights[(c+3)*32+i+0] as i32;
a0_3 += v0*wb;
a1_3 += v1*wb;
}
i+=1; }
output[(y*12+x+0)*64+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*12+x+1)*64+c+0]=(requantize(a1_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*12+x+0)*64+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*12+x+1)*64+c+1]=(requantize(a1_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*12+x+0)*64+c+2]=(requantize(a0_2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*12+x+1)*64+c+2]=(requantize(a1_2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*12+x+0)*64+c+3]=(requantize(a0_3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
output[(y*12+x+1)*64+c+3]=(requantize(a1_3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
c+=4; }
x+=2; }
} }
/// # Safety
/// Pointers must be aligned, valid for lengths 4608, 2048, 64, 64, 64, 9216 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_8(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,4608),core::slice::from_raw_parts(w,2048),core::slice::from_raw_parts(b,64),core::slice::from_raw_parts(m,64),core::slice::from_raw_parts(s,64),core::slice::from_raw_parts_mut(o,9216)); }

}
mod layer_9 {

// Generated for person_layer_9 with b8_u1_s1_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 9216, 576, 9216.
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


/// All buffers are caller owned. INT32 accumulation and shifted values must fit.
#[inline(never)]
pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=9216 && weights.len()>=576 && output.len()>=9216);
assert!(bias.len()>=64 && mult.len()>=64 && shift.len()>=64);
assert!(mult[..64].iter().all(|&m|m>=0) && shift[..64].iter().all(|&q|(-30..=-1).contains(&q)));
for y in 0..12 { for x in 0..12 {
let mut c=0usize; while c<64 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
let mut a4=bias[c+4];
let mut a5=bias[c+5];
let mut a6=bias[c+6];
let mut a7=bias[c+7];
{ let iy=(y*1+0) as isize-1; let ix=(x*1+0) as isize-1;
if iy>=0 && iy<12 && ix>=0 && ix<12 {
let ip=(iy as usize*12+ix as usize)*64; let wp=0;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+0) as isize-1; let ix=(x*1+1) as isize-1;
if iy>=0 && iy<12 && ix>=0 && ix<12 {
let ip=(iy as usize*12+ix as usize)*64; let wp=64;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+0) as isize-1; let ix=(x*1+2) as isize-1;
if iy>=0 && iy<12 && ix>=0 && ix<12 {
let ip=(iy as usize*12+ix as usize)*64; let wp=128;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+1) as isize-1; let ix=(x*1+0) as isize-1;
if iy>=0 && iy<12 && ix>=0 && ix<12 {
let ip=(iy as usize*12+ix as usize)*64; let wp=192;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+1) as isize-1; let ix=(x*1+1) as isize-1;
if iy>=0 && iy<12 && ix>=0 && ix<12 {
let ip=(iy as usize*12+ix as usize)*64; let wp=256;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+1) as isize-1; let ix=(x*1+2) as isize-1;
if iy>=0 && iy<12 && ix>=0 && ix<12 {
let ip=(iy as usize*12+ix as usize)*64; let wp=320;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+2) as isize-1; let ix=(x*1+0) as isize-1;
if iy>=0 && iy<12 && ix>=0 && ix<12 {
let ip=(iy as usize*12+ix as usize)*64; let wp=384;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+2) as isize-1; let ix=(x*1+1) as isize-1;
if iy>=0 && iy<12 && ix>=0 && ix<12 {
let ip=(iy as usize*12+ix as usize)*64; let wp=448;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+2) as isize-1; let ix=(x*1+2) as isize-1;
if iy>=0 && iy<12 && ix>=0 && ix<12 {
let ip=(iy as usize*12+ix as usize)*64; let wp=512;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
output[(y*12+x)*64+c+0] = (requantize(a0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*12+x)*64+c+1] = (requantize(a1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*12+x)*64+c+2] = (requantize(a2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*12+x)*64+c+3] = (requantize(a3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
output[(y*12+x)*64+c+4] = (requantize(a4,mult[c+4],shift[c+4])+-128).clamp(-128,127) as i8;
output[(y*12+x)*64+c+5] = (requantize(a5,mult[c+5],shift[c+5])+-128).clamp(-128,127) as i8;
output[(y*12+x)*64+c+6] = (requantize(a6,mult[c+6],shift[c+6])+-128).clamp(-128,127) as i8;
output[(y*12+x)*64+c+7] = (requantize(a7,mult[c+7],shift[c+7])+-128).clamp(-128,127) as i8;
c+=8; }
} } }
/// # Safety
/// Pointers must be aligned, valid for lengths 9216, 576, 64, 64, 64, 9216 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_9(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,9216),core::slice::from_raw_parts(w,576),core::slice::from_raw_parts(b,64),core::slice::from_raw_parts(m,64),core::slice::from_raw_parts(s,64),core::slice::from_raw_parts_mut(o,9216)); }

}
mod layer_10 {

// Generated for person_layer_10 with b4_u1_s0_p2_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 9216, 4096, 9216.
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


/// All buffers are caller owned. INT32 accumulation and shifted values must fit.
#[inline(never)]
pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=9216 && weights.len()>=4096 && output.len()>=9216);
assert!(bias.len()>=64 && mult.len()>=64 && shift.len()>=64);
assert!(mult[..64].iter().all(|&m|m>=0) && shift[..64].iter().all(|&q|(-30..=-1).contains(&q)));
for y in 0..12 { let mut x=0usize; while x<12 {
let mut c=0usize; while c<64 {
let base0=bias[c+0];
let mut a0_0=base0;
let mut a1_0=base0;
let base1=bias[c+1];
let mut a0_1=base1;
let mut a1_1=base1;
let base2=bias[c+2];
let mut a0_2=base2;
let mut a1_2=base2;
let base3=bias[c+3];
let mut a0_3=base3;
let mut a1_3=base3;
let mut i=0usize; while i<64 {
{
let v0=input[(y*1*12+(x+0)*1)*64+i+0] as i32+0;
let v1=input[(y*1*12+(x+1)*1)*64+i+0] as i32+0;
let wb=weights[(c+0)*64+i+0] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let wb=weights[(c+1)*64+i+0] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
let wb=weights[(c+2)*64+i+0] as i32;
a0_2 += v0*wb;
a1_2 += v1*wb;
let wb=weights[(c+3)*64+i+0] as i32;
a0_3 += v0*wb;
a1_3 += v1*wb;
}
i+=1; }
output[(y*12+x+0)*64+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*12+x+1)*64+c+0]=(requantize(a1_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*12+x+0)*64+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*12+x+1)*64+c+1]=(requantize(a1_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*12+x+0)*64+c+2]=(requantize(a0_2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*12+x+1)*64+c+2]=(requantize(a1_2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*12+x+0)*64+c+3]=(requantize(a0_3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
output[(y*12+x+1)*64+c+3]=(requantize(a1_3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
c+=4; }
x+=2; }
} }
/// # Safety
/// Pointers must be aligned, valid for lengths 9216, 4096, 64, 64, 64, 9216 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_10(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,9216),core::slice::from_raw_parts(w,4096),core::slice::from_raw_parts(b,64),core::slice::from_raw_parts(m,64),core::slice::from_raw_parts(s,64),core::slice::from_raw_parts_mut(o,9216)); }

}
mod layer_11 {

// Generated for person_layer_11 with b1_u1_s1_cxy_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 10816, 576, 2304.
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


/// All buffers are caller owned. INT32 accumulation and shifted values must fit.
#[inline(never)]
pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=10816 && weights.len()>=576 && output.len()>=2304);
assert!(bias.len()>=64 && mult.len()>=64 && shift.len()>=64);
assert!(mult[..64].iter().all(|&m|m>=0) && shift[..64].iter().all(|&q|(-30..=-1).contains(&q)));
let mut c=0usize; while c<64 { for y in 0..6 { for x in 0..6 {
let mut a0=bias[c+0];
{ let iy=(y*2+0) as isize-0; let ix=(x*2+0) as isize-0;
if iy>=0 && iy<13 && ix>=0 && ix<13 {
let ip=(iy as usize*13+ix as usize)*64; let wp=0;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+0) as isize-0; let ix=(x*2+1) as isize-0;
if iy>=0 && iy<13 && ix>=0 && ix<13 {
let ip=(iy as usize*13+ix as usize)*64; let wp=64;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+0) as isize-0; let ix=(x*2+2) as isize-0;
if iy>=0 && iy<13 && ix>=0 && ix<13 {
let ip=(iy as usize*13+ix as usize)*64; let wp=128;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+1) as isize-0; let ix=(x*2+0) as isize-0;
if iy>=0 && iy<13 && ix>=0 && ix<13 {
let ip=(iy as usize*13+ix as usize)*64; let wp=192;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+1) as isize-0; let ix=(x*2+1) as isize-0;
if iy>=0 && iy<13 && ix>=0 && ix<13 {
let ip=(iy as usize*13+ix as usize)*64; let wp=256;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+1) as isize-0; let ix=(x*2+2) as isize-0;
if iy>=0 && iy<13 && ix>=0 && ix<13 {
let ip=(iy as usize*13+ix as usize)*64; let wp=320;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+2) as isize-0; let ix=(x*2+0) as isize-0;
if iy>=0 && iy<13 && ix>=0 && ix<13 {
let ip=(iy as usize*13+ix as usize)*64; let wp=384;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+2) as isize-0; let ix=(x*2+1) as isize-0;
if iy>=0 && iy<13 && ix>=0 && ix<13 {
let ip=(iy as usize*13+ix as usize)*64; let wp=448;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+2) as isize-0; let ix=(x*2+2) as isize-0;
if iy>=0 && iy<13 && ix>=0 && ix<13 {
let ip=(iy as usize*13+ix as usize)*64; let wp=512;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
output[(y*6+x)*64+c+0] = (requantize(a0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
} } c+=1; }
}
/// # Safety
/// Pointers must be aligned, valid for lengths 10816, 576, 64, 64, 64, 2304 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_11(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,10816),core::slice::from_raw_parts(w,576),core::slice::from_raw_parts(b,64),core::slice::from_raw_parts(m,64),core::slice::from_raw_parts(s,64),core::slice::from_raw_parts_mut(o,2304)); }

}
mod layer_12 {

// Generated for person_layer_12 with b4_u1_s0_p2_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 2304, 8192, 4608.
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


/// All buffers are caller owned. INT32 accumulation and shifted values must fit.
#[inline(never)]
pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=2304 && weights.len()>=8192 && output.len()>=4608);
assert!(bias.len()>=128 && mult.len()>=128 && shift.len()>=128);
assert!(mult[..128].iter().all(|&m|m>=0) && shift[..128].iter().all(|&q|(-30..=-1).contains(&q)));
for y in 0..6 { let mut x=0usize; while x<6 {
let mut c=0usize; while c<128 {
let base0=bias[c+0];
let mut a0_0=base0;
let mut a1_0=base0;
let base1=bias[c+1];
let mut a0_1=base1;
let mut a1_1=base1;
let base2=bias[c+2];
let mut a0_2=base2;
let mut a1_2=base2;
let base3=bias[c+3];
let mut a0_3=base3;
let mut a1_3=base3;
let mut i=0usize; while i<64 {
{
let v0=input[(y*1*6+(x+0)*1)*64+i+0] as i32+0;
let v1=input[(y*1*6+(x+1)*1)*64+i+0] as i32+0;
let wb=weights[(c+0)*64+i+0] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let wb=weights[(c+1)*64+i+0] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
let wb=weights[(c+2)*64+i+0] as i32;
a0_2 += v0*wb;
a1_2 += v1*wb;
let wb=weights[(c+3)*64+i+0] as i32;
a0_3 += v0*wb;
a1_3 += v1*wb;
}
i+=1; }
output[(y*6+x+0)*128+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*6+x+1)*128+c+0]=(requantize(a1_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*6+x+0)*128+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*6+x+1)*128+c+1]=(requantize(a1_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*6+x+0)*128+c+2]=(requantize(a0_2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*6+x+1)*128+c+2]=(requantize(a1_2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*6+x+0)*128+c+3]=(requantize(a0_3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
output[(y*6+x+1)*128+c+3]=(requantize(a1_3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
c+=4; }
x+=2; }
} }
/// # Safety
/// Pointers must be aligned, valid for lengths 2304, 8192, 128, 128, 128, 4608 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_12(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,2304),core::slice::from_raw_parts(w,8192),core::slice::from_raw_parts(b,128),core::slice::from_raw_parts(m,128),core::slice::from_raw_parts(s,128),core::slice::from_raw_parts_mut(o,4608)); }

}
mod layer_13 {

// Generated for person_layer_13 with b8_u1_s1_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 4608, 1152, 4608.
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


/// All buffers are caller owned. INT32 accumulation and shifted values must fit.
#[inline(never)]
pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=4608 && weights.len()>=1152 && output.len()>=4608);
assert!(bias.len()>=128 && mult.len()>=128 && shift.len()>=128);
assert!(mult[..128].iter().all(|&m|m>=0) && shift[..128].iter().all(|&q|(-30..=-1).contains(&q)));
for y in 0..6 { for x in 0..6 {
let mut c=0usize; while c<128 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
let mut a4=bias[c+4];
let mut a5=bias[c+5];
let mut a6=bias[c+6];
let mut a7=bias[c+7];
{ let iy=(y*1+0) as isize-1; let ix=(x*1+0) as isize-1;
if iy>=0 && iy<6 && ix>=0 && ix<6 {
let ip=(iy as usize*6+ix as usize)*128; let wp=0;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+0) as isize-1; let ix=(x*1+1) as isize-1;
if iy>=0 && iy<6 && ix>=0 && ix<6 {
let ip=(iy as usize*6+ix as usize)*128; let wp=128;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+0) as isize-1; let ix=(x*1+2) as isize-1;
if iy>=0 && iy<6 && ix>=0 && ix<6 {
let ip=(iy as usize*6+ix as usize)*128; let wp=256;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+1) as isize-1; let ix=(x*1+0) as isize-1;
if iy>=0 && iy<6 && ix>=0 && ix<6 {
let ip=(iy as usize*6+ix as usize)*128; let wp=384;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+1) as isize-1; let ix=(x*1+1) as isize-1;
if iy>=0 && iy<6 && ix>=0 && ix<6 {
let ip=(iy as usize*6+ix as usize)*128; let wp=512;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+1) as isize-1; let ix=(x*1+2) as isize-1;
if iy>=0 && iy<6 && ix>=0 && ix<6 {
let ip=(iy as usize*6+ix as usize)*128; let wp=640;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+2) as isize-1; let ix=(x*1+0) as isize-1;
if iy>=0 && iy<6 && ix>=0 && ix<6 {
let ip=(iy as usize*6+ix as usize)*128; let wp=768;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+2) as isize-1; let ix=(x*1+1) as isize-1;
if iy>=0 && iy<6 && ix>=0 && ix<6 {
let ip=(iy as usize*6+ix as usize)*128; let wp=896;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
{ let iy=(y*1+2) as isize-1; let ix=(x*1+2) as isize-1;
if iy>=0 && iy<6 && ix>=0 && ix<6 {
let ip=(iy as usize*6+ix as usize)*128; let wp=1024;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + 128) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + 128) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + 128) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + 128) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + 128) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + 128) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + 128) * weights[wp+c+7] as i32;
} }
output[(y*6+x)*128+c+0] = (requantize(a0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*6+x)*128+c+1] = (requantize(a1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*6+x)*128+c+2] = (requantize(a2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*6+x)*128+c+3] = (requantize(a3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
output[(y*6+x)*128+c+4] = (requantize(a4,mult[c+4],shift[c+4])+-128).clamp(-128,127) as i8;
output[(y*6+x)*128+c+5] = (requantize(a5,mult[c+5],shift[c+5])+-128).clamp(-128,127) as i8;
output[(y*6+x)*128+c+6] = (requantize(a6,mult[c+6],shift[c+6])+-128).clamp(-128,127) as i8;
output[(y*6+x)*128+c+7] = (requantize(a7,mult[c+7],shift[c+7])+-128).clamp(-128,127) as i8;
c+=8; }
} } }
/// # Safety
/// Pointers must be aligned, valid for lengths 4608, 1152, 128, 128, 128, 4608 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_13(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,4608),core::slice::from_raw_parts(w,1152),core::slice::from_raw_parts(b,128),core::slice::from_raw_parts(m,128),core::slice::from_raw_parts(s,128),core::slice::from_raw_parts_mut(o,4608)); }

}
mod layer_14 {

// Generated for person_layer_14 with b4_u2_s0_p2_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 4608, 16384, 4608.
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


/// All buffers are caller owned. INT32 accumulation and shifted values must fit.
#[inline(never)]
pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=4608 && weights.len()>=16384 && output.len()>=4608);
assert!(bias.len()>=128 && mult.len()>=128 && shift.len()>=128);
assert!(mult[..128].iter().all(|&m|m>=0) && shift[..128].iter().all(|&q|(-30..=-1).contains(&q)));
for y in 0..6 { let mut x=0usize; while x<6 {
let mut c=0usize; while c<128 {
let base0=bias[c+0];
let mut a0_0=base0;
let mut a1_0=base0;
let base1=bias[c+1];
let mut a0_1=base1;
let mut a1_1=base1;
let base2=bias[c+2];
let mut a0_2=base2;
let mut a1_2=base2;
let base3=bias[c+3];
let mut a0_3=base3;
let mut a1_3=base3;
let mut i=0usize; while i<128 {
{
let v0=input[(y*1*6+(x+0)*1)*128+i+0] as i32+0;
let v1=input[(y*1*6+(x+1)*1)*128+i+0] as i32+0;
let wb=weights[(c+0)*128+i+0] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let wb=weights[(c+1)*128+i+0] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
let wb=weights[(c+2)*128+i+0] as i32;
a0_2 += v0*wb;
a1_2 += v1*wb;
let wb=weights[(c+3)*128+i+0] as i32;
a0_3 += v0*wb;
a1_3 += v1*wb;
}
{
let v0=input[(y*1*6+(x+0)*1)*128+i+1] as i32+0;
let v1=input[(y*1*6+(x+1)*1)*128+i+1] as i32+0;
let wb=weights[(c+0)*128+i+1] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let wb=weights[(c+1)*128+i+1] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
let wb=weights[(c+2)*128+i+1] as i32;
a0_2 += v0*wb;
a1_2 += v1*wb;
let wb=weights[(c+3)*128+i+1] as i32;
a0_3 += v0*wb;
a1_3 += v1*wb;
}
i+=2; }
output[(y*6+x+0)*128+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*6+x+1)*128+c+0]=(requantize(a1_0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*6+x+0)*128+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*6+x+1)*128+c+1]=(requantize(a1_1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*6+x+0)*128+c+2]=(requantize(a0_2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*6+x+1)*128+c+2]=(requantize(a1_2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*6+x+0)*128+c+3]=(requantize(a0_3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
output[(y*6+x+1)*128+c+3]=(requantize(a1_3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
c+=4; }
x+=2; }
} }
/// # Safety
/// Pointers must be aligned, valid for lengths 4608, 16384, 128, 128, 128, 4608 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_14(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,4608),core::slice::from_raw_parts(w,16384),core::slice::from_raw_parts(b,128),core::slice::from_raw_parts(m,128),core::slice::from_raw_parts(s,128),core::slice::from_raw_parts_mut(o,4608)); }

}
mod layer_15 {
/// # Safety
/// Pointers must be aligned, valid for lengths 4608, 1152, 128, 128, 128, 4608 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_15(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
super::layer_13::run(core::slice::from_raw_parts(i,4608),core::slice::from_raw_parts(w,1152),core::slice::from_raw_parts(b,128),core::slice::from_raw_parts(m,128),core::slice::from_raw_parts(s,128),core::slice::from_raw_parts_mut(o,4608)); }

}
mod layer_16 {
/// # Safety
/// Pointers must be aligned, valid for lengths 4608, 16384, 128, 128, 128, 4608 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_16(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
super::layer_14::run(core::slice::from_raw_parts(i,4608),core::slice::from_raw_parts(w,16384),core::slice::from_raw_parts(b,128),core::slice::from_raw_parts(m,128),core::slice::from_raw_parts(s,128),core::slice::from_raw_parts_mut(o,4608)); }

}
mod layer_17 {
/// # Safety
/// Pointers must be aligned, valid for lengths 4608, 1152, 128, 128, 128, 4608 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_17(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
super::layer_13::run(core::slice::from_raw_parts(i,4608),core::slice::from_raw_parts(w,1152),core::slice::from_raw_parts(b,128),core::slice::from_raw_parts(m,128),core::slice::from_raw_parts(s,128),core::slice::from_raw_parts_mut(o,4608)); }

}
mod layer_18 {
/// # Safety
/// Pointers must be aligned, valid for lengths 4608, 16384, 128, 128, 128, 4608 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_18(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
super::layer_14::run(core::slice::from_raw_parts(i,4608),core::slice::from_raw_parts(w,16384),core::slice::from_raw_parts(b,128),core::slice::from_raw_parts(m,128),core::slice::from_raw_parts(s,128),core::slice::from_raw_parts_mut(o,4608)); }

}
mod layer_19 {
/// # Safety
/// Pointers must be aligned, valid for lengths 4608, 1152, 128, 128, 128, 4608 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_19(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
super::layer_13::run(core::slice::from_raw_parts(i,4608),core::slice::from_raw_parts(w,1152),core::slice::from_raw_parts(b,128),core::slice::from_raw_parts(m,128),core::slice::from_raw_parts(s,128),core::slice::from_raw_parts_mut(o,4608)); }

}
mod layer_20 {
/// # Safety
/// Pointers must be aligned, valid for lengths 4608, 16384, 128, 128, 128, 4608 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_20(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
super::layer_14::run(core::slice::from_raw_parts(i,4608),core::slice::from_raw_parts(w,16384),core::slice::from_raw_parts(b,128),core::slice::from_raw_parts(m,128),core::slice::from_raw_parts(s,128),core::slice::from_raw_parts_mut(o,4608)); }

}
mod layer_21 {
/// # Safety
/// Pointers must be aligned, valid for lengths 4608, 1152, 128, 128, 128, 4608 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_21(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
super::layer_13::run(core::slice::from_raw_parts(i,4608),core::slice::from_raw_parts(w,1152),core::slice::from_raw_parts(b,128),core::slice::from_raw_parts(m,128),core::slice::from_raw_parts(s,128),core::slice::from_raw_parts_mut(o,4608)); }

}
mod layer_22 {
/// # Safety
/// Pointers must be aligned, valid for lengths 4608, 16384, 128, 128, 128, 4608 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_22(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
super::layer_14::run(core::slice::from_raw_parts(i,4608),core::slice::from_raw_parts(w,16384),core::slice::from_raw_parts(b,128),core::slice::from_raw_parts(m,128),core::slice::from_raw_parts(s,128),core::slice::from_raw_parts_mut(o,4608)); }

}
mod layer_23 {

// Generated for person_layer_23 with b1_u1_s1_cxy_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 6272, 1152, 1152.
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


/// All buffers are caller owned. INT32 accumulation and shifted values must fit.
#[inline(never)]
pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=6272 && weights.len()>=1152 && output.len()>=1152);
assert!(bias.len()>=128 && mult.len()>=128 && shift.len()>=128);
assert!(mult[..128].iter().all(|&m|m>=0) && shift[..128].iter().all(|&q|(-30..=-1).contains(&q)));
let mut c=0usize; while c<128 { for y in 0..3 { for x in 0..3 {
let mut a0=bias[c+0];
{ let iy=(y*2+0) as isize-0; let ix=(x*2+0) as isize-0;
if iy>=0 && iy<7 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*128; let wp=0;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+0) as isize-0; let ix=(x*2+1) as isize-0;
if iy>=0 && iy<7 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*128; let wp=128;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+0) as isize-0; let ix=(x*2+2) as isize-0;
if iy>=0 && iy<7 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*128; let wp=256;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+1) as isize-0; let ix=(x*2+0) as isize-0;
if iy>=0 && iy<7 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*128; let wp=384;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+1) as isize-0; let ix=(x*2+1) as isize-0;
if iy>=0 && iy<7 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*128; let wp=512;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+1) as isize-0; let ix=(x*2+2) as isize-0;
if iy>=0 && iy<7 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*128; let wp=640;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+2) as isize-0; let ix=(x*2+0) as isize-0;
if iy>=0 && iy<7 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*128; let wp=768;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+2) as isize-0; let ix=(x*2+1) as isize-0;
if iy>=0 && iy<7 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*128; let wp=896;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+2) as isize-0; let ix=(x*2+2) as isize-0;
if iy>=0 && iy<7 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*128; let wp=1024;
a0 += (input[ip+c+0] as i32 + 0) * weights[wp+c+0] as i32;
} }
output[(y*3+x)*128+c+0] = (requantize(a0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
} } c+=1; }
}
/// # Safety
/// Pointers must be aligned, valid for lengths 6272, 1152, 128, 128, 128, 1152 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_23(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,6272),core::slice::from_raw_parts(w,1152),core::slice::from_raw_parts(b,128),core::slice::from_raw_parts(m,128),core::slice::from_raw_parts(s,128),core::slice::from_raw_parts_mut(o,1152)); }

}
mod layer_24 {

// Generated for person_layer_24 with b8_u1_s0_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 1152, 32768, 2304.
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


/// All buffers are caller owned. INT32 accumulation and shifted values must fit.
#[inline(never)]
pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=1152 && weights.len()>=32768 && output.len()>=2304);
assert!(bias.len()>=256 && mult.len()>=256 && shift.len()>=256);
assert!(mult[..256].iter().all(|&m|m>=0) && shift[..256].iter().all(|&q|(-30..=-1).contains(&q)));
for y in 0..3 { for x in 0..3 {
let mut c=0usize; while c<256 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
let mut a4=bias[c+4];
let mut a5=bias[c+5];
let mut a6=bias[c+6];
let mut a7=bias[c+7];
let by=(y*1) as isize-0; let bx=(x*1) as isize-0;
let ys=0isize.max(-by) as usize; let ye=(1isize).min(3isize-by) as usize;
let xs=0isize.max(-bx) as usize; let xe=(1isize).min(3isize-bx) as usize;
for ky in ys..ye { for kx in xs..xe {
let ip=(((by+ky as isize) as usize)*3+(bx+kx as isize) as usize)*128; let wp=(ky*1+kx)*128;
let mut i=0usize; while i < 128 {
let v = input[ip+i+0] as i32 + 0;
a0 += v * weights[((c+0)*128+wp)+i+0] as i32;
a1 += v * weights[((c+1)*128+wp)+i+0] as i32;
a2 += v * weights[((c+2)*128+wp)+i+0] as i32;
a3 += v * weights[((c+3)*128+wp)+i+0] as i32;
a4 += v * weights[((c+4)*128+wp)+i+0] as i32;
a5 += v * weights[((c+5)*128+wp)+i+0] as i32;
a6 += v * weights[((c+6)*128+wp)+i+0] as i32;
a7 += v * weights[((c+7)*128+wp)+i+0] as i32;
i+=1; }
} }
output[(y*3+x)*256+c+0] = (requantize(a0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*3+x)*256+c+1] = (requantize(a1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*3+x)*256+c+2] = (requantize(a2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*3+x)*256+c+3] = (requantize(a3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
output[(y*3+x)*256+c+4] = (requantize(a4,mult[c+4],shift[c+4])+-128).clamp(-128,127) as i8;
output[(y*3+x)*256+c+5] = (requantize(a5,mult[c+5],shift[c+5])+-128).clamp(-128,127) as i8;
output[(y*3+x)*256+c+6] = (requantize(a6,mult[c+6],shift[c+6])+-128).clamp(-128,127) as i8;
output[(y*3+x)*256+c+7] = (requantize(a7,mult[c+7],shift[c+7])+-128).clamp(-128,127) as i8;
c+=8; }
} } }
/// # Safety
/// Pointers must be aligned, valid for lengths 1152, 32768, 256, 256, 256, 2304 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_24(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,1152),core::slice::from_raw_parts(w,32768),core::slice::from_raw_parts(b,256),core::slice::from_raw_parts(m,256),core::slice::from_raw_parts(s,256),core::slice::from_raw_parts_mut(o,2304)); }

}
mod layer_25 {

// Generated for person_layer_25 with b1_u1_s1_cxy_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 2304, 2304, 2304.
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


/// All buffers are caller owned. INT32 accumulation and shifted values must fit.
#[inline(never)]
pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=2304 && weights.len()>=2304 && output.len()>=2304);
assert!(bias.len()>=256 && mult.len()>=256 && shift.len()>=256);
assert!(mult[..256].iter().all(|&m|m>=0) && shift[..256].iter().all(|&q|(-30..=-1).contains(&q)));
let mut c=0usize; while c<256 { for y in 0..3 { for x in 0..3 {
let mut a0=bias[c+0];
{ let iy=(y*1+0) as isize-1; let ix=(x*1+0) as isize-1;
if iy>=0 && iy<3 && ix>=0 && ix<3 {
let ip=(iy as usize*3+ix as usize)*256; let wp=0;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
} }
{ let iy=(y*1+0) as isize-1; let ix=(x*1+1) as isize-1;
if iy>=0 && iy<3 && ix>=0 && ix<3 {
let ip=(iy as usize*3+ix as usize)*256; let wp=256;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
} }
{ let iy=(y*1+0) as isize-1; let ix=(x*1+2) as isize-1;
if iy>=0 && iy<3 && ix>=0 && ix<3 {
let ip=(iy as usize*3+ix as usize)*256; let wp=512;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
} }
{ let iy=(y*1+1) as isize-1; let ix=(x*1+0) as isize-1;
if iy>=0 && iy<3 && ix>=0 && ix<3 {
let ip=(iy as usize*3+ix as usize)*256; let wp=768;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
} }
{ let iy=(y*1+1) as isize-1; let ix=(x*1+1) as isize-1;
if iy>=0 && iy<3 && ix>=0 && ix<3 {
let ip=(iy as usize*3+ix as usize)*256; let wp=1024;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
} }
{ let iy=(y*1+1) as isize-1; let ix=(x*1+2) as isize-1;
if iy>=0 && iy<3 && ix>=0 && ix<3 {
let ip=(iy as usize*3+ix as usize)*256; let wp=1280;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
} }
{ let iy=(y*1+2) as isize-1; let ix=(x*1+0) as isize-1;
if iy>=0 && iy<3 && ix>=0 && ix<3 {
let ip=(iy as usize*3+ix as usize)*256; let wp=1536;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
} }
{ let iy=(y*1+2) as isize-1; let ix=(x*1+1) as isize-1;
if iy>=0 && iy<3 && ix>=0 && ix<3 {
let ip=(iy as usize*3+ix as usize)*256; let wp=1792;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
} }
{ let iy=(y*1+2) as isize-1; let ix=(x*1+2) as isize-1;
if iy>=0 && iy<3 && ix>=0 && ix<3 {
let ip=(iy as usize*3+ix as usize)*256; let wp=2048;
a0 += (input[ip+c+0] as i32 + 128) * weights[wp+c+0] as i32;
} }
output[(y*3+x)*256+c+0] = (requantize(a0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
} } c+=1; }
}
/// # Safety
/// Pointers must be aligned, valid for lengths 2304, 2304, 256, 256, 256, 2304 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_25(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,2304),core::slice::from_raw_parts(w,2304),core::slice::from_raw_parts(b,256),core::slice::from_raw_parts(m,256),core::slice::from_raw_parts(s,256),core::slice::from_raw_parts_mut(o,2304)); }

}
mod layer_26 {

// Generated for person_layer_26 with b8_u1_s0_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 2304, 65536, 2304.
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


/// All buffers are caller owned. INT32 accumulation and shifted values must fit.
#[inline(never)]
pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=2304 && weights.len()>=65536 && output.len()>=2304);
assert!(bias.len()>=256 && mult.len()>=256 && shift.len()>=256);
assert!(mult[..256].iter().all(|&m|m>=0) && shift[..256].iter().all(|&q|(-30..=-1).contains(&q)));
for y in 0..3 { for x in 0..3 {
let mut c=0usize; while c<256 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
let mut a4=bias[c+4];
let mut a5=bias[c+5];
let mut a6=bias[c+6];
let mut a7=bias[c+7];
let by=(y*1) as isize-0; let bx=(x*1) as isize-0;
let ys=0isize.max(-by) as usize; let ye=(1isize).min(3isize-by) as usize;
let xs=0isize.max(-bx) as usize; let xe=(1isize).min(3isize-bx) as usize;
for ky in ys..ye { for kx in xs..xe {
let ip=(((by+ky as isize) as usize)*3+(bx+kx as isize) as usize)*256; let wp=(ky*1+kx)*256;
let mut i=0usize; while i < 256 {
let v = input[ip+i+0] as i32 + 0;
a0 += v * weights[((c+0)*256+wp)+i+0] as i32;
a1 += v * weights[((c+1)*256+wp)+i+0] as i32;
a2 += v * weights[((c+2)*256+wp)+i+0] as i32;
a3 += v * weights[((c+3)*256+wp)+i+0] as i32;
a4 += v * weights[((c+4)*256+wp)+i+0] as i32;
a5 += v * weights[((c+5)*256+wp)+i+0] as i32;
a6 += v * weights[((c+6)*256+wp)+i+0] as i32;
a7 += v * weights[((c+7)*256+wp)+i+0] as i32;
i+=1; }
} }
output[(y*3+x)*256+c+0] = (requantize(a0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*3+x)*256+c+1] = (requantize(a1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*3+x)*256+c+2] = (requantize(a2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*3+x)*256+c+3] = (requantize(a3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
output[(y*3+x)*256+c+4] = (requantize(a4,mult[c+4],shift[c+4])+-128).clamp(-128,127) as i8;
output[(y*3+x)*256+c+5] = (requantize(a5,mult[c+5],shift[c+5])+-128).clamp(-128,127) as i8;
output[(y*3+x)*256+c+6] = (requantize(a6,mult[c+6],shift[c+6])+-128).clamp(-128,127) as i8;
output[(y*3+x)*256+c+7] = (requantize(a7,mult[c+7],shift[c+7])+-128).clamp(-128,127) as i8;
c+=8; }
} } }
/// # Safety
/// Pointers must be aligned, valid for lengths 2304, 65536, 256, 256, 256, 2304 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_26(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,2304),core::slice::from_raw_parts(w,65536),core::slice::from_raw_parts(b,256),core::slice::from_raw_parts(m,256),core::slice::from_raw_parts(s,256),core::slice::from_raw_parts_mut(o,2304)); }

}
mod layer_28 {

// Generated for person_layer_28 with b2_u1_s0_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 256, 512, 2.
/// Exact two-stage requantization for a statically proven accumulator bound:
/// |x| <= 2^30, multiplier >= 0, and shift in -30..=-1.
/// With these bounds high+nudge cannot overflow i32. No rounding is approximated.
#[inline(always)]
pub fn requantize(x: i32, multiplier: i32, shift: i32) -> i32 {
    debug_assert!((-(1 << 30)..=(1 << 30)).contains(&x));
    debug_assert!(multiplier >= 0 && (-30..=-1).contains(&shift));
    let high = ((i64::from(x) * i64::from(multiplier) + (1i64 << 30)) >> 31) as i32;
    let exponent = (-shift) as u32;
    let nudge = (1i32 << (exponent - 1)) - i32::from(high < 0);
    (high + nudge) >> exponent
}


/// All buffers are caller owned. INT32 accumulation and shifted values must fit.
#[inline(never)]
pub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {
assert!(input.len()>=256 && weights.len()>=512 && output.len()>=2);
assert!(bias.len()>=2 && mult.len()>=2 && shift.len()>=2);
assert!(mult[..2].iter().all(|&m|m>=0) && shift[..2].iter().all(|&q|(-30..=-1).contains(&q)));
for y in 0..1 { for x in 0..1 {
let mut c=0usize; while c<2 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let by=(y*1) as isize-0; let bx=(x*1) as isize-0;
let ys=0isize.max(-by) as usize; let ye=(1isize).min(1isize-by) as usize;
let xs=0isize.max(-bx) as usize; let xe=(1isize).min(1isize-bx) as usize;
for ky in ys..ye { for kx in xs..xe {
let ip=(((by+ky as isize) as usize)*1+(bx+kx as isize) as usize)*256; let wp=(ky*1+kx)*256;
let mut i=0usize; while i < 256 {
let v = input[ip+i+0] as i32 + 0;
a0 += v * weights[((c+0)*256+wp)+i+0] as i32;
a1 += v * weights[((c+1)*256+wp)+i+0] as i32;
i+=1; }
} }
output[(y*1+x)*2+c+0] = (requantize(a0,mult[c+0],shift[c+0])+-1).clamp(-128,127) as i8;
output[(y*1+x)*2+c+1] = (requantize(a1,mult[c+1],shift[c+1])+-1).clamp(-128,127) as i8;
c+=2; }
} } }
/// # Safety
/// Pointers must be aligned, valid for lengths 256, 512, 2, 2, 2, 2 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_28(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,256),core::slice::from_raw_parts(w,512),core::slice::from_raw_parts(b,2),core::slice::from_raw_parts(m,2),core::slice::from_raw_parts(s,2),core::slice::from_raw_parts_mut(o,2)); }

}
#[panic_handler] fn panic(_: &core::panic::PanicInfo) -> ! { loop { core::hint::spin_loop(); } }
