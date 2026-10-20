#![no_std]
// Generated for pointwise_small with b2_u8_s0_p2_qpos. Scratch: 0 bytes.
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
for y in 0..5 { let mut x=0usize; while x<6 {
let mut c=0usize; while c<8 {
let base0=bias[c+0];
let mut a0_0=base0;
let mut a1_0=base0;
let base1=bias[c+1];
let mut a0_1=base1;
let mut a1_1=base1;
let mut i=0usize; while i<0 {
{
let v0=input[(y*1*7+(x+0)*1)*7+i+0] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+i+0] as i32+-3;
let wb=weights[(c+0)*7+i+0] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let wb=weights[(c+1)*7+i+0] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+1] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+i+1] as i32+-3;
let wb=weights[(c+0)*7+i+1] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let wb=weights[(c+1)*7+i+1] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+2] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+i+2] as i32+-3;
let wb=weights[(c+0)*7+i+2] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let wb=weights[(c+1)*7+i+2] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+3] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+i+3] as i32+-3;
let wb=weights[(c+0)*7+i+3] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let wb=weights[(c+1)*7+i+3] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+4] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+i+4] as i32+-3;
let wb=weights[(c+0)*7+i+4] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let wb=weights[(c+1)*7+i+4] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+5] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+i+5] as i32+-3;
let wb=weights[(c+0)*7+i+5] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let wb=weights[(c+1)*7+i+5] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+6] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+i+6] as i32+-3;
let wb=weights[(c+0)*7+i+6] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let wb=weights[(c+1)*7+i+6] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+7] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+i+7] as i32+-3;
let wb=weights[(c+0)*7+i+7] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let wb=weights[(c+1)*7+i+7] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
}
i+=8; }
let v0=input[(y*1*7+(x+0)*1)*7+0] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+0] as i32+-3;
let wb=weights[(c+0)*7+0] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let wb=weights[(c+1)*7+0] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
let v0=input[(y*1*7+(x+0)*1)*7+1] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+1] as i32+-3;
let wb=weights[(c+0)*7+1] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let wb=weights[(c+1)*7+1] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
let v0=input[(y*1*7+(x+0)*1)*7+2] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+2] as i32+-3;
let wb=weights[(c+0)*7+2] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let wb=weights[(c+1)*7+2] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
let v0=input[(y*1*7+(x+0)*1)*7+3] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+3] as i32+-3;
let wb=weights[(c+0)*7+3] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let wb=weights[(c+1)*7+3] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
let v0=input[(y*1*7+(x+0)*1)*7+4] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+4] as i32+-3;
let wb=weights[(c+0)*7+4] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let wb=weights[(c+1)*7+4] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
let v0=input[(y*1*7+(x+0)*1)*7+5] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+5] as i32+-3;
let wb=weights[(c+0)*7+5] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let wb=weights[(c+1)*7+5] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
let v0=input[(y*1*7+(x+0)*1)*7+6] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+6] as i32+-3;
let wb=weights[(c+0)*7+6] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let wb=weights[(c+1)*7+6] as i32;
a0_1 += v0*wb;
a1_1 += v1*wb;
output[(y*7+x+0)*9+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*7+x+1)*9+c+0]=(requantize(a1_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*7+x+0)*9+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+5).clamp(-111,103) as i8;
output[(y*7+x+1)*9+c+1]=(requantize(a1_1,mult[c+1],shift[c+1])+5).clamp(-111,103) as i8;
c+=2; }
while c<9 {
let base0=bias[c+0];
let mut a0_0=base0;
let mut a1_0=base0;
let mut i=0usize; while i<0 {
{
let v0=input[(y*1*7+(x+0)*1)*7+i+0] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+i+0] as i32+-3;
let wb=weights[(c+0)*7+i+0] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+1] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+i+1] as i32+-3;
let wb=weights[(c+0)*7+i+1] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+2] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+i+2] as i32+-3;
let wb=weights[(c+0)*7+i+2] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+3] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+i+3] as i32+-3;
let wb=weights[(c+0)*7+i+3] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+4] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+i+4] as i32+-3;
let wb=weights[(c+0)*7+i+4] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+5] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+i+5] as i32+-3;
let wb=weights[(c+0)*7+i+5] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+6] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+i+6] as i32+-3;
let wb=weights[(c+0)*7+i+6] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+7] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+i+7] as i32+-3;
let wb=weights[(c+0)*7+i+7] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
}
i+=8; }
let v0=input[(y*1*7+(x+0)*1)*7+0] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+0] as i32+-3;
let wb=weights[(c+0)*7+0] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let v0=input[(y*1*7+(x+0)*1)*7+1] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+1] as i32+-3;
let wb=weights[(c+0)*7+1] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let v0=input[(y*1*7+(x+0)*1)*7+2] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+2] as i32+-3;
let wb=weights[(c+0)*7+2] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let v0=input[(y*1*7+(x+0)*1)*7+3] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+3] as i32+-3;
let wb=weights[(c+0)*7+3] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let v0=input[(y*1*7+(x+0)*1)*7+4] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+4] as i32+-3;
let wb=weights[(c+0)*7+4] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let v0=input[(y*1*7+(x+0)*1)*7+5] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+5] as i32+-3;
let wb=weights[(c+0)*7+5] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
let v0=input[(y*1*7+(x+0)*1)*7+6] as i32+-3;
let v1=input[(y*1*7+(x+1)*1)*7+6] as i32+-3;
let wb=weights[(c+0)*7+6] as i32;
a0_0 += v0*wb;
a1_0 += v1*wb;
output[(y*7+x+0)*9+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*7+x+1)*9+c+0]=(requantize(a1_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
c+=1; }
x+=2; }
while x<7 {
let mut c=0usize; while c<8 {
let base0=bias[c+0];
let mut a0_0=base0;
let base1=bias[c+1];
let mut a0_1=base1;
let mut i=0usize; while i<0 {
{
let v0=input[(y*1*7+(x+0)*1)*7+i+0] as i32+-3;
let wb=weights[(c+0)*7+i+0] as i32;
a0_0 += v0*wb;
let wb=weights[(c+1)*7+i+0] as i32;
a0_1 += v0*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+1] as i32+-3;
let wb=weights[(c+0)*7+i+1] as i32;
a0_0 += v0*wb;
let wb=weights[(c+1)*7+i+1] as i32;
a0_1 += v0*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+2] as i32+-3;
let wb=weights[(c+0)*7+i+2] as i32;
a0_0 += v0*wb;
let wb=weights[(c+1)*7+i+2] as i32;
a0_1 += v0*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+3] as i32+-3;
let wb=weights[(c+0)*7+i+3] as i32;
a0_0 += v0*wb;
let wb=weights[(c+1)*7+i+3] as i32;
a0_1 += v0*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+4] as i32+-3;
let wb=weights[(c+0)*7+i+4] as i32;
a0_0 += v0*wb;
let wb=weights[(c+1)*7+i+4] as i32;
a0_1 += v0*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+5] as i32+-3;
let wb=weights[(c+0)*7+i+5] as i32;
a0_0 += v0*wb;
let wb=weights[(c+1)*7+i+5] as i32;
a0_1 += v0*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+6] as i32+-3;
let wb=weights[(c+0)*7+i+6] as i32;
a0_0 += v0*wb;
let wb=weights[(c+1)*7+i+6] as i32;
a0_1 += v0*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+7] as i32+-3;
let wb=weights[(c+0)*7+i+7] as i32;
a0_0 += v0*wb;
let wb=weights[(c+1)*7+i+7] as i32;
a0_1 += v0*wb;
}
i+=8; }
let v0=input[(y*1*7+(x+0)*1)*7+0] as i32+-3;
let wb=weights[(c+0)*7+0] as i32;
a0_0 += v0*wb;
let wb=weights[(c+1)*7+0] as i32;
a0_1 += v0*wb;
let v0=input[(y*1*7+(x+0)*1)*7+1] as i32+-3;
let wb=weights[(c+0)*7+1] as i32;
a0_0 += v0*wb;
let wb=weights[(c+1)*7+1] as i32;
a0_1 += v0*wb;
let v0=input[(y*1*7+(x+0)*1)*7+2] as i32+-3;
let wb=weights[(c+0)*7+2] as i32;
a0_0 += v0*wb;
let wb=weights[(c+1)*7+2] as i32;
a0_1 += v0*wb;
let v0=input[(y*1*7+(x+0)*1)*7+3] as i32+-3;
let wb=weights[(c+0)*7+3] as i32;
a0_0 += v0*wb;
let wb=weights[(c+1)*7+3] as i32;
a0_1 += v0*wb;
let v0=input[(y*1*7+(x+0)*1)*7+4] as i32+-3;
let wb=weights[(c+0)*7+4] as i32;
a0_0 += v0*wb;
let wb=weights[(c+1)*7+4] as i32;
a0_1 += v0*wb;
let v0=input[(y*1*7+(x+0)*1)*7+5] as i32+-3;
let wb=weights[(c+0)*7+5] as i32;
a0_0 += v0*wb;
let wb=weights[(c+1)*7+5] as i32;
a0_1 += v0*wb;
let v0=input[(y*1*7+(x+0)*1)*7+6] as i32+-3;
let wb=weights[(c+0)*7+6] as i32;
a0_0 += v0*wb;
let wb=weights[(c+1)*7+6] as i32;
a0_1 += v0*wb;
output[(y*7+x+0)*9+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*7+x+0)*9+c+1]=(requantize(a0_1,mult[c+1],shift[c+1])+5).clamp(-111,103) as i8;
c+=2; }
while c<9 {
let base0=bias[c+0];
let mut a0_0=base0;
let mut i=0usize; while i<0 {
{
let v0=input[(y*1*7+(x+0)*1)*7+i+0] as i32+-3;
let wb=weights[(c+0)*7+i+0] as i32;
a0_0 += v0*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+1] as i32+-3;
let wb=weights[(c+0)*7+i+1] as i32;
a0_0 += v0*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+2] as i32+-3;
let wb=weights[(c+0)*7+i+2] as i32;
a0_0 += v0*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+3] as i32+-3;
let wb=weights[(c+0)*7+i+3] as i32;
a0_0 += v0*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+4] as i32+-3;
let wb=weights[(c+0)*7+i+4] as i32;
a0_0 += v0*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+5] as i32+-3;
let wb=weights[(c+0)*7+i+5] as i32;
a0_0 += v0*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+6] as i32+-3;
let wb=weights[(c+0)*7+i+6] as i32;
a0_0 += v0*wb;
}
{
let v0=input[(y*1*7+(x+0)*1)*7+i+7] as i32+-3;
let wb=weights[(c+0)*7+i+7] as i32;
a0_0 += v0*wb;
}
i+=8; }
let v0=input[(y*1*7+(x+0)*1)*7+0] as i32+-3;
let wb=weights[(c+0)*7+0] as i32;
a0_0 += v0*wb;
let v0=input[(y*1*7+(x+0)*1)*7+1] as i32+-3;
let wb=weights[(c+0)*7+1] as i32;
a0_0 += v0*wb;
let v0=input[(y*1*7+(x+0)*1)*7+2] as i32+-3;
let wb=weights[(c+0)*7+2] as i32;
a0_0 += v0*wb;
let v0=input[(y*1*7+(x+0)*1)*7+3] as i32+-3;
let wb=weights[(c+0)*7+3] as i32;
a0_0 += v0*wb;
let v0=input[(y*1*7+(x+0)*1)*7+4] as i32+-3;
let wb=weights[(c+0)*7+4] as i32;
a0_0 += v0*wb;
let v0=input[(y*1*7+(x+0)*1)*7+5] as i32+-3;
let wb=weights[(c+0)*7+5] as i32;
a0_0 += v0*wb;
let v0=input[(y*1*7+(x+0)*1)*7+6] as i32+-3;
let wb=weights[(c+0)*7+6] as i32;
a0_0 += v0*wb;
output[(y*7+x+0)*9+c+0]=(requantize(a0_0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
c+=1; }
x+=1; }
} }
/// # Safety
/// Pointers must be aligned, valid for lengths 245, 63, 9, 9, 9, 315 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn kernel(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,245),core::slice::from_raw_parts(w,63),core::slice::from_raw_parts(b,9),core::slice::from_raw_parts(m,9),core::slice::from_raw_parts(s,9),core::slice::from_raw_parts_mut(o,315)); }
