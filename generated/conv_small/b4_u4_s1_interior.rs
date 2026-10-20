#![no_std]
// Generated for conv_small with b4_u4_s1_interior. Scratch: 0 bytes.
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
for y in 0..4 { for x in 0..5 {
let mut c=0usize; while c<4 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
if y*2>=1 && y*2+2<8 && x*2>=1 && x*2+2<10 {
{ let ip=((y*2+0-1)*9+x*2+0-1)*3; let wp=0;
let mut i=0usize; while i < 0 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+0] as i32;
a1 += v * weights[((c+1)*27+wp)+i+0] as i32;
a2 += v * weights[((c+2)*27+wp)+i+0] as i32;
a3 += v * weights[((c+3)*27+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+1] as i32;
a1 += v * weights[((c+1)*27+wp)+i+1] as i32;
a2 += v * weights[((c+2)*27+wp)+i+1] as i32;
a3 += v * weights[((c+3)*27+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+2] as i32;
a1 += v * weights[((c+1)*27+wp)+i+2] as i32;
a2 += v * weights[((c+2)*27+wp)+i+2] as i32;
a3 += v * weights[((c+3)*27+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+3] as i32;
a1 += v * weights[((c+1)*27+wp)+i+3] as i32;
a2 += v * weights[((c+2)*27+wp)+i+3] as i32;
a3 += v * weights[((c+3)*27+wp)+i+3] as i32;
i+=4; }
let v = input[ip+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+0] as i32;
a1 += v * weights[((c+1)*27+wp)+0] as i32;
a2 += v * weights[((c+2)*27+wp)+0] as i32;
a3 += v * weights[((c+3)*27+wp)+0] as i32;
let v = input[ip+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+1] as i32;
a1 += v * weights[((c+1)*27+wp)+1] as i32;
a2 += v * weights[((c+2)*27+wp)+1] as i32;
a3 += v * weights[((c+3)*27+wp)+1] as i32;
let v = input[ip+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+2] as i32;
a1 += v * weights[((c+1)*27+wp)+2] as i32;
a2 += v * weights[((c+2)*27+wp)+2] as i32;
a3 += v * weights[((c+3)*27+wp)+2] as i32;
}
{ let ip=((y*2+0-1)*9+x*2+1-1)*3; let wp=3;
let mut i=0usize; while i < 0 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+0] as i32;
a1 += v * weights[((c+1)*27+wp)+i+0] as i32;
a2 += v * weights[((c+2)*27+wp)+i+0] as i32;
a3 += v * weights[((c+3)*27+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+1] as i32;
a1 += v * weights[((c+1)*27+wp)+i+1] as i32;
a2 += v * weights[((c+2)*27+wp)+i+1] as i32;
a3 += v * weights[((c+3)*27+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+2] as i32;
a1 += v * weights[((c+1)*27+wp)+i+2] as i32;
a2 += v * weights[((c+2)*27+wp)+i+2] as i32;
a3 += v * weights[((c+3)*27+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+3] as i32;
a1 += v * weights[((c+1)*27+wp)+i+3] as i32;
a2 += v * weights[((c+2)*27+wp)+i+3] as i32;
a3 += v * weights[((c+3)*27+wp)+i+3] as i32;
i+=4; }
let v = input[ip+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+0] as i32;
a1 += v * weights[((c+1)*27+wp)+0] as i32;
a2 += v * weights[((c+2)*27+wp)+0] as i32;
a3 += v * weights[((c+3)*27+wp)+0] as i32;
let v = input[ip+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+1] as i32;
a1 += v * weights[((c+1)*27+wp)+1] as i32;
a2 += v * weights[((c+2)*27+wp)+1] as i32;
a3 += v * weights[((c+3)*27+wp)+1] as i32;
let v = input[ip+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+2] as i32;
a1 += v * weights[((c+1)*27+wp)+2] as i32;
a2 += v * weights[((c+2)*27+wp)+2] as i32;
a3 += v * weights[((c+3)*27+wp)+2] as i32;
}
{ let ip=((y*2+0-1)*9+x*2+2-1)*3; let wp=6;
let mut i=0usize; while i < 0 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+0] as i32;
a1 += v * weights[((c+1)*27+wp)+i+0] as i32;
a2 += v * weights[((c+2)*27+wp)+i+0] as i32;
a3 += v * weights[((c+3)*27+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+1] as i32;
a1 += v * weights[((c+1)*27+wp)+i+1] as i32;
a2 += v * weights[((c+2)*27+wp)+i+1] as i32;
a3 += v * weights[((c+3)*27+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+2] as i32;
a1 += v * weights[((c+1)*27+wp)+i+2] as i32;
a2 += v * weights[((c+2)*27+wp)+i+2] as i32;
a3 += v * weights[((c+3)*27+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+3] as i32;
a1 += v * weights[((c+1)*27+wp)+i+3] as i32;
a2 += v * weights[((c+2)*27+wp)+i+3] as i32;
a3 += v * weights[((c+3)*27+wp)+i+3] as i32;
i+=4; }
let v = input[ip+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+0] as i32;
a1 += v * weights[((c+1)*27+wp)+0] as i32;
a2 += v * weights[((c+2)*27+wp)+0] as i32;
a3 += v * weights[((c+3)*27+wp)+0] as i32;
let v = input[ip+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+1] as i32;
a1 += v * weights[((c+1)*27+wp)+1] as i32;
a2 += v * weights[((c+2)*27+wp)+1] as i32;
a3 += v * weights[((c+3)*27+wp)+1] as i32;
let v = input[ip+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+2] as i32;
a1 += v * weights[((c+1)*27+wp)+2] as i32;
a2 += v * weights[((c+2)*27+wp)+2] as i32;
a3 += v * weights[((c+3)*27+wp)+2] as i32;
}
{ let ip=((y*2+1-1)*9+x*2+0-1)*3; let wp=9;
let mut i=0usize; while i < 0 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+0] as i32;
a1 += v * weights[((c+1)*27+wp)+i+0] as i32;
a2 += v * weights[((c+2)*27+wp)+i+0] as i32;
a3 += v * weights[((c+3)*27+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+1] as i32;
a1 += v * weights[((c+1)*27+wp)+i+1] as i32;
a2 += v * weights[((c+2)*27+wp)+i+1] as i32;
a3 += v * weights[((c+3)*27+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+2] as i32;
a1 += v * weights[((c+1)*27+wp)+i+2] as i32;
a2 += v * weights[((c+2)*27+wp)+i+2] as i32;
a3 += v * weights[((c+3)*27+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+3] as i32;
a1 += v * weights[((c+1)*27+wp)+i+3] as i32;
a2 += v * weights[((c+2)*27+wp)+i+3] as i32;
a3 += v * weights[((c+3)*27+wp)+i+3] as i32;
i+=4; }
let v = input[ip+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+0] as i32;
a1 += v * weights[((c+1)*27+wp)+0] as i32;
a2 += v * weights[((c+2)*27+wp)+0] as i32;
a3 += v * weights[((c+3)*27+wp)+0] as i32;
let v = input[ip+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+1] as i32;
a1 += v * weights[((c+1)*27+wp)+1] as i32;
a2 += v * weights[((c+2)*27+wp)+1] as i32;
a3 += v * weights[((c+3)*27+wp)+1] as i32;
let v = input[ip+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+2] as i32;
a1 += v * weights[((c+1)*27+wp)+2] as i32;
a2 += v * weights[((c+2)*27+wp)+2] as i32;
a3 += v * weights[((c+3)*27+wp)+2] as i32;
}
{ let ip=((y*2+1-1)*9+x*2+1-1)*3; let wp=12;
let mut i=0usize; while i < 0 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+0] as i32;
a1 += v * weights[((c+1)*27+wp)+i+0] as i32;
a2 += v * weights[((c+2)*27+wp)+i+0] as i32;
a3 += v * weights[((c+3)*27+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+1] as i32;
a1 += v * weights[((c+1)*27+wp)+i+1] as i32;
a2 += v * weights[((c+2)*27+wp)+i+1] as i32;
a3 += v * weights[((c+3)*27+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+2] as i32;
a1 += v * weights[((c+1)*27+wp)+i+2] as i32;
a2 += v * weights[((c+2)*27+wp)+i+2] as i32;
a3 += v * weights[((c+3)*27+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+3] as i32;
a1 += v * weights[((c+1)*27+wp)+i+3] as i32;
a2 += v * weights[((c+2)*27+wp)+i+3] as i32;
a3 += v * weights[((c+3)*27+wp)+i+3] as i32;
i+=4; }
let v = input[ip+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+0] as i32;
a1 += v * weights[((c+1)*27+wp)+0] as i32;
a2 += v * weights[((c+2)*27+wp)+0] as i32;
a3 += v * weights[((c+3)*27+wp)+0] as i32;
let v = input[ip+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+1] as i32;
a1 += v * weights[((c+1)*27+wp)+1] as i32;
a2 += v * weights[((c+2)*27+wp)+1] as i32;
a3 += v * weights[((c+3)*27+wp)+1] as i32;
let v = input[ip+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+2] as i32;
a1 += v * weights[((c+1)*27+wp)+2] as i32;
a2 += v * weights[((c+2)*27+wp)+2] as i32;
a3 += v * weights[((c+3)*27+wp)+2] as i32;
}
{ let ip=((y*2+1-1)*9+x*2+2-1)*3; let wp=15;
let mut i=0usize; while i < 0 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+0] as i32;
a1 += v * weights[((c+1)*27+wp)+i+0] as i32;
a2 += v * weights[((c+2)*27+wp)+i+0] as i32;
a3 += v * weights[((c+3)*27+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+1] as i32;
a1 += v * weights[((c+1)*27+wp)+i+1] as i32;
a2 += v * weights[((c+2)*27+wp)+i+1] as i32;
a3 += v * weights[((c+3)*27+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+2] as i32;
a1 += v * weights[((c+1)*27+wp)+i+2] as i32;
a2 += v * weights[((c+2)*27+wp)+i+2] as i32;
a3 += v * weights[((c+3)*27+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+3] as i32;
a1 += v * weights[((c+1)*27+wp)+i+3] as i32;
a2 += v * weights[((c+2)*27+wp)+i+3] as i32;
a3 += v * weights[((c+3)*27+wp)+i+3] as i32;
i+=4; }
let v = input[ip+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+0] as i32;
a1 += v * weights[((c+1)*27+wp)+0] as i32;
a2 += v * weights[((c+2)*27+wp)+0] as i32;
a3 += v * weights[((c+3)*27+wp)+0] as i32;
let v = input[ip+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+1] as i32;
a1 += v * weights[((c+1)*27+wp)+1] as i32;
a2 += v * weights[((c+2)*27+wp)+1] as i32;
a3 += v * weights[((c+3)*27+wp)+1] as i32;
let v = input[ip+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+2] as i32;
a1 += v * weights[((c+1)*27+wp)+2] as i32;
a2 += v * weights[((c+2)*27+wp)+2] as i32;
a3 += v * weights[((c+3)*27+wp)+2] as i32;
}
{ let ip=((y*2+2-1)*9+x*2+0-1)*3; let wp=18;
let mut i=0usize; while i < 0 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+0] as i32;
a1 += v * weights[((c+1)*27+wp)+i+0] as i32;
a2 += v * weights[((c+2)*27+wp)+i+0] as i32;
a3 += v * weights[((c+3)*27+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+1] as i32;
a1 += v * weights[((c+1)*27+wp)+i+1] as i32;
a2 += v * weights[((c+2)*27+wp)+i+1] as i32;
a3 += v * weights[((c+3)*27+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+2] as i32;
a1 += v * weights[((c+1)*27+wp)+i+2] as i32;
a2 += v * weights[((c+2)*27+wp)+i+2] as i32;
a3 += v * weights[((c+3)*27+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+3] as i32;
a1 += v * weights[((c+1)*27+wp)+i+3] as i32;
a2 += v * weights[((c+2)*27+wp)+i+3] as i32;
a3 += v * weights[((c+3)*27+wp)+i+3] as i32;
i+=4; }
let v = input[ip+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+0] as i32;
a1 += v * weights[((c+1)*27+wp)+0] as i32;
a2 += v * weights[((c+2)*27+wp)+0] as i32;
a3 += v * weights[((c+3)*27+wp)+0] as i32;
let v = input[ip+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+1] as i32;
a1 += v * weights[((c+1)*27+wp)+1] as i32;
a2 += v * weights[((c+2)*27+wp)+1] as i32;
a3 += v * weights[((c+3)*27+wp)+1] as i32;
let v = input[ip+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+2] as i32;
a1 += v * weights[((c+1)*27+wp)+2] as i32;
a2 += v * weights[((c+2)*27+wp)+2] as i32;
a3 += v * weights[((c+3)*27+wp)+2] as i32;
}
{ let ip=((y*2+2-1)*9+x*2+1-1)*3; let wp=21;
let mut i=0usize; while i < 0 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+0] as i32;
a1 += v * weights[((c+1)*27+wp)+i+0] as i32;
a2 += v * weights[((c+2)*27+wp)+i+0] as i32;
a3 += v * weights[((c+3)*27+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+1] as i32;
a1 += v * weights[((c+1)*27+wp)+i+1] as i32;
a2 += v * weights[((c+2)*27+wp)+i+1] as i32;
a3 += v * weights[((c+3)*27+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+2] as i32;
a1 += v * weights[((c+1)*27+wp)+i+2] as i32;
a2 += v * weights[((c+2)*27+wp)+i+2] as i32;
a3 += v * weights[((c+3)*27+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+3] as i32;
a1 += v * weights[((c+1)*27+wp)+i+3] as i32;
a2 += v * weights[((c+2)*27+wp)+i+3] as i32;
a3 += v * weights[((c+3)*27+wp)+i+3] as i32;
i+=4; }
let v = input[ip+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+0] as i32;
a1 += v * weights[((c+1)*27+wp)+0] as i32;
a2 += v * weights[((c+2)*27+wp)+0] as i32;
a3 += v * weights[((c+3)*27+wp)+0] as i32;
let v = input[ip+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+1] as i32;
a1 += v * weights[((c+1)*27+wp)+1] as i32;
a2 += v * weights[((c+2)*27+wp)+1] as i32;
a3 += v * weights[((c+3)*27+wp)+1] as i32;
let v = input[ip+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+2] as i32;
a1 += v * weights[((c+1)*27+wp)+2] as i32;
a2 += v * weights[((c+2)*27+wp)+2] as i32;
a3 += v * weights[((c+3)*27+wp)+2] as i32;
}
{ let ip=((y*2+2-1)*9+x*2+2-1)*3; let wp=24;
let mut i=0usize; while i < 0 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+0] as i32;
a1 += v * weights[((c+1)*27+wp)+i+0] as i32;
a2 += v * weights[((c+2)*27+wp)+i+0] as i32;
a3 += v * weights[((c+3)*27+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+1] as i32;
a1 += v * weights[((c+1)*27+wp)+i+1] as i32;
a2 += v * weights[((c+2)*27+wp)+i+1] as i32;
a3 += v * weights[((c+3)*27+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+2] as i32;
a1 += v * weights[((c+1)*27+wp)+i+2] as i32;
a2 += v * weights[((c+2)*27+wp)+i+2] as i32;
a3 += v * weights[((c+3)*27+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+3] as i32;
a1 += v * weights[((c+1)*27+wp)+i+3] as i32;
a2 += v * weights[((c+2)*27+wp)+i+3] as i32;
a3 += v * weights[((c+3)*27+wp)+i+3] as i32;
i+=4; }
let v = input[ip+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+0] as i32;
a1 += v * weights[((c+1)*27+wp)+0] as i32;
a2 += v * weights[((c+2)*27+wp)+0] as i32;
a3 += v * weights[((c+3)*27+wp)+0] as i32;
let v = input[ip+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+1] as i32;
a1 += v * weights[((c+1)*27+wp)+1] as i32;
a2 += v * weights[((c+2)*27+wp)+1] as i32;
a3 += v * weights[((c+3)*27+wp)+1] as i32;
let v = input[ip+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+2] as i32;
a1 += v * weights[((c+1)*27+wp)+2] as i32;
a2 += v * weights[((c+2)*27+wp)+2] as i32;
a3 += v * weights[((c+3)*27+wp)+2] as i32;
}
} else {
{ let iy=(y*2+0) as isize-1; let ix=(x*2+0) as isize-1;
if iy>=0 && iy<7 && ix>=0 && ix<9 {
let ip=(iy as usize*9+ix as usize)*3; let wp=0;
let mut i=0usize; while i < 0 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+0] as i32;
a1 += v * weights[((c+1)*27+wp)+i+0] as i32;
a2 += v * weights[((c+2)*27+wp)+i+0] as i32;
a3 += v * weights[((c+3)*27+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+1] as i32;
a1 += v * weights[((c+1)*27+wp)+i+1] as i32;
a2 += v * weights[((c+2)*27+wp)+i+1] as i32;
a3 += v * weights[((c+3)*27+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+2] as i32;
a1 += v * weights[((c+1)*27+wp)+i+2] as i32;
a2 += v * weights[((c+2)*27+wp)+i+2] as i32;
a3 += v * weights[((c+3)*27+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+3] as i32;
a1 += v * weights[((c+1)*27+wp)+i+3] as i32;
a2 += v * weights[((c+2)*27+wp)+i+3] as i32;
a3 += v * weights[((c+3)*27+wp)+i+3] as i32;
i+=4; }
let v = input[ip+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+0] as i32;
a1 += v * weights[((c+1)*27+wp)+0] as i32;
a2 += v * weights[((c+2)*27+wp)+0] as i32;
a3 += v * weights[((c+3)*27+wp)+0] as i32;
let v = input[ip+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+1] as i32;
a1 += v * weights[((c+1)*27+wp)+1] as i32;
a2 += v * weights[((c+2)*27+wp)+1] as i32;
a3 += v * weights[((c+3)*27+wp)+1] as i32;
let v = input[ip+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+2] as i32;
a1 += v * weights[((c+1)*27+wp)+2] as i32;
a2 += v * weights[((c+2)*27+wp)+2] as i32;
a3 += v * weights[((c+3)*27+wp)+2] as i32;
} }
{ let iy=(y*2+0) as isize-1; let ix=(x*2+1) as isize-1;
if iy>=0 && iy<7 && ix>=0 && ix<9 {
let ip=(iy as usize*9+ix as usize)*3; let wp=3;
let mut i=0usize; while i < 0 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+0] as i32;
a1 += v * weights[((c+1)*27+wp)+i+0] as i32;
a2 += v * weights[((c+2)*27+wp)+i+0] as i32;
a3 += v * weights[((c+3)*27+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+1] as i32;
a1 += v * weights[((c+1)*27+wp)+i+1] as i32;
a2 += v * weights[((c+2)*27+wp)+i+1] as i32;
a3 += v * weights[((c+3)*27+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+2] as i32;
a1 += v * weights[((c+1)*27+wp)+i+2] as i32;
a2 += v * weights[((c+2)*27+wp)+i+2] as i32;
a3 += v * weights[((c+3)*27+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+3] as i32;
a1 += v * weights[((c+1)*27+wp)+i+3] as i32;
a2 += v * weights[((c+2)*27+wp)+i+3] as i32;
a3 += v * weights[((c+3)*27+wp)+i+3] as i32;
i+=4; }
let v = input[ip+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+0] as i32;
a1 += v * weights[((c+1)*27+wp)+0] as i32;
a2 += v * weights[((c+2)*27+wp)+0] as i32;
a3 += v * weights[((c+3)*27+wp)+0] as i32;
let v = input[ip+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+1] as i32;
a1 += v * weights[((c+1)*27+wp)+1] as i32;
a2 += v * weights[((c+2)*27+wp)+1] as i32;
a3 += v * weights[((c+3)*27+wp)+1] as i32;
let v = input[ip+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+2] as i32;
a1 += v * weights[((c+1)*27+wp)+2] as i32;
a2 += v * weights[((c+2)*27+wp)+2] as i32;
a3 += v * weights[((c+3)*27+wp)+2] as i32;
} }
{ let iy=(y*2+0) as isize-1; let ix=(x*2+2) as isize-1;
if iy>=0 && iy<7 && ix>=0 && ix<9 {
let ip=(iy as usize*9+ix as usize)*3; let wp=6;
let mut i=0usize; while i < 0 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+0] as i32;
a1 += v * weights[((c+1)*27+wp)+i+0] as i32;
a2 += v * weights[((c+2)*27+wp)+i+0] as i32;
a3 += v * weights[((c+3)*27+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+1] as i32;
a1 += v * weights[((c+1)*27+wp)+i+1] as i32;
a2 += v * weights[((c+2)*27+wp)+i+1] as i32;
a3 += v * weights[((c+3)*27+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+2] as i32;
a1 += v * weights[((c+1)*27+wp)+i+2] as i32;
a2 += v * weights[((c+2)*27+wp)+i+2] as i32;
a3 += v * weights[((c+3)*27+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+3] as i32;
a1 += v * weights[((c+1)*27+wp)+i+3] as i32;
a2 += v * weights[((c+2)*27+wp)+i+3] as i32;
a3 += v * weights[((c+3)*27+wp)+i+3] as i32;
i+=4; }
let v = input[ip+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+0] as i32;
a1 += v * weights[((c+1)*27+wp)+0] as i32;
a2 += v * weights[((c+2)*27+wp)+0] as i32;
a3 += v * weights[((c+3)*27+wp)+0] as i32;
let v = input[ip+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+1] as i32;
a1 += v * weights[((c+1)*27+wp)+1] as i32;
a2 += v * weights[((c+2)*27+wp)+1] as i32;
a3 += v * weights[((c+3)*27+wp)+1] as i32;
let v = input[ip+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+2] as i32;
a1 += v * weights[((c+1)*27+wp)+2] as i32;
a2 += v * weights[((c+2)*27+wp)+2] as i32;
a3 += v * weights[((c+3)*27+wp)+2] as i32;
} }
{ let iy=(y*2+1) as isize-1; let ix=(x*2+0) as isize-1;
if iy>=0 && iy<7 && ix>=0 && ix<9 {
let ip=(iy as usize*9+ix as usize)*3; let wp=9;
let mut i=0usize; while i < 0 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+0] as i32;
a1 += v * weights[((c+1)*27+wp)+i+0] as i32;
a2 += v * weights[((c+2)*27+wp)+i+0] as i32;
a3 += v * weights[((c+3)*27+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+1] as i32;
a1 += v * weights[((c+1)*27+wp)+i+1] as i32;
a2 += v * weights[((c+2)*27+wp)+i+1] as i32;
a3 += v * weights[((c+3)*27+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+2] as i32;
a1 += v * weights[((c+1)*27+wp)+i+2] as i32;
a2 += v * weights[((c+2)*27+wp)+i+2] as i32;
a3 += v * weights[((c+3)*27+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+3] as i32;
a1 += v * weights[((c+1)*27+wp)+i+3] as i32;
a2 += v * weights[((c+2)*27+wp)+i+3] as i32;
a3 += v * weights[((c+3)*27+wp)+i+3] as i32;
i+=4; }
let v = input[ip+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+0] as i32;
a1 += v * weights[((c+1)*27+wp)+0] as i32;
a2 += v * weights[((c+2)*27+wp)+0] as i32;
a3 += v * weights[((c+3)*27+wp)+0] as i32;
let v = input[ip+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+1] as i32;
a1 += v * weights[((c+1)*27+wp)+1] as i32;
a2 += v * weights[((c+2)*27+wp)+1] as i32;
a3 += v * weights[((c+3)*27+wp)+1] as i32;
let v = input[ip+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+2] as i32;
a1 += v * weights[((c+1)*27+wp)+2] as i32;
a2 += v * weights[((c+2)*27+wp)+2] as i32;
a3 += v * weights[((c+3)*27+wp)+2] as i32;
} }
{ let iy=(y*2+1) as isize-1; let ix=(x*2+1) as isize-1;
if iy>=0 && iy<7 && ix>=0 && ix<9 {
let ip=(iy as usize*9+ix as usize)*3; let wp=12;
let mut i=0usize; while i < 0 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+0] as i32;
a1 += v * weights[((c+1)*27+wp)+i+0] as i32;
a2 += v * weights[((c+2)*27+wp)+i+0] as i32;
a3 += v * weights[((c+3)*27+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+1] as i32;
a1 += v * weights[((c+1)*27+wp)+i+1] as i32;
a2 += v * weights[((c+2)*27+wp)+i+1] as i32;
a3 += v * weights[((c+3)*27+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+2] as i32;
a1 += v * weights[((c+1)*27+wp)+i+2] as i32;
a2 += v * weights[((c+2)*27+wp)+i+2] as i32;
a3 += v * weights[((c+3)*27+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+3] as i32;
a1 += v * weights[((c+1)*27+wp)+i+3] as i32;
a2 += v * weights[((c+2)*27+wp)+i+3] as i32;
a3 += v * weights[((c+3)*27+wp)+i+3] as i32;
i+=4; }
let v = input[ip+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+0] as i32;
a1 += v * weights[((c+1)*27+wp)+0] as i32;
a2 += v * weights[((c+2)*27+wp)+0] as i32;
a3 += v * weights[((c+3)*27+wp)+0] as i32;
let v = input[ip+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+1] as i32;
a1 += v * weights[((c+1)*27+wp)+1] as i32;
a2 += v * weights[((c+2)*27+wp)+1] as i32;
a3 += v * weights[((c+3)*27+wp)+1] as i32;
let v = input[ip+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+2] as i32;
a1 += v * weights[((c+1)*27+wp)+2] as i32;
a2 += v * weights[((c+2)*27+wp)+2] as i32;
a3 += v * weights[((c+3)*27+wp)+2] as i32;
} }
{ let iy=(y*2+1) as isize-1; let ix=(x*2+2) as isize-1;
if iy>=0 && iy<7 && ix>=0 && ix<9 {
let ip=(iy as usize*9+ix as usize)*3; let wp=15;
let mut i=0usize; while i < 0 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+0] as i32;
a1 += v * weights[((c+1)*27+wp)+i+0] as i32;
a2 += v * weights[((c+2)*27+wp)+i+0] as i32;
a3 += v * weights[((c+3)*27+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+1] as i32;
a1 += v * weights[((c+1)*27+wp)+i+1] as i32;
a2 += v * weights[((c+2)*27+wp)+i+1] as i32;
a3 += v * weights[((c+3)*27+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+2] as i32;
a1 += v * weights[((c+1)*27+wp)+i+2] as i32;
a2 += v * weights[((c+2)*27+wp)+i+2] as i32;
a3 += v * weights[((c+3)*27+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+3] as i32;
a1 += v * weights[((c+1)*27+wp)+i+3] as i32;
a2 += v * weights[((c+2)*27+wp)+i+3] as i32;
a3 += v * weights[((c+3)*27+wp)+i+3] as i32;
i+=4; }
let v = input[ip+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+0] as i32;
a1 += v * weights[((c+1)*27+wp)+0] as i32;
a2 += v * weights[((c+2)*27+wp)+0] as i32;
a3 += v * weights[((c+3)*27+wp)+0] as i32;
let v = input[ip+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+1] as i32;
a1 += v * weights[((c+1)*27+wp)+1] as i32;
a2 += v * weights[((c+2)*27+wp)+1] as i32;
a3 += v * weights[((c+3)*27+wp)+1] as i32;
let v = input[ip+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+2] as i32;
a1 += v * weights[((c+1)*27+wp)+2] as i32;
a2 += v * weights[((c+2)*27+wp)+2] as i32;
a3 += v * weights[((c+3)*27+wp)+2] as i32;
} }
{ let iy=(y*2+2) as isize-1; let ix=(x*2+0) as isize-1;
if iy>=0 && iy<7 && ix>=0 && ix<9 {
let ip=(iy as usize*9+ix as usize)*3; let wp=18;
let mut i=0usize; while i < 0 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+0] as i32;
a1 += v * weights[((c+1)*27+wp)+i+0] as i32;
a2 += v * weights[((c+2)*27+wp)+i+0] as i32;
a3 += v * weights[((c+3)*27+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+1] as i32;
a1 += v * weights[((c+1)*27+wp)+i+1] as i32;
a2 += v * weights[((c+2)*27+wp)+i+1] as i32;
a3 += v * weights[((c+3)*27+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+2] as i32;
a1 += v * weights[((c+1)*27+wp)+i+2] as i32;
a2 += v * weights[((c+2)*27+wp)+i+2] as i32;
a3 += v * weights[((c+3)*27+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+3] as i32;
a1 += v * weights[((c+1)*27+wp)+i+3] as i32;
a2 += v * weights[((c+2)*27+wp)+i+3] as i32;
a3 += v * weights[((c+3)*27+wp)+i+3] as i32;
i+=4; }
let v = input[ip+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+0] as i32;
a1 += v * weights[((c+1)*27+wp)+0] as i32;
a2 += v * weights[((c+2)*27+wp)+0] as i32;
a3 += v * weights[((c+3)*27+wp)+0] as i32;
let v = input[ip+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+1] as i32;
a1 += v * weights[((c+1)*27+wp)+1] as i32;
a2 += v * weights[((c+2)*27+wp)+1] as i32;
a3 += v * weights[((c+3)*27+wp)+1] as i32;
let v = input[ip+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+2] as i32;
a1 += v * weights[((c+1)*27+wp)+2] as i32;
a2 += v * weights[((c+2)*27+wp)+2] as i32;
a3 += v * weights[((c+3)*27+wp)+2] as i32;
} }
{ let iy=(y*2+2) as isize-1; let ix=(x*2+1) as isize-1;
if iy>=0 && iy<7 && ix>=0 && ix<9 {
let ip=(iy as usize*9+ix as usize)*3; let wp=21;
let mut i=0usize; while i < 0 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+0] as i32;
a1 += v * weights[((c+1)*27+wp)+i+0] as i32;
a2 += v * weights[((c+2)*27+wp)+i+0] as i32;
a3 += v * weights[((c+3)*27+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+1] as i32;
a1 += v * weights[((c+1)*27+wp)+i+1] as i32;
a2 += v * weights[((c+2)*27+wp)+i+1] as i32;
a3 += v * weights[((c+3)*27+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+2] as i32;
a1 += v * weights[((c+1)*27+wp)+i+2] as i32;
a2 += v * weights[((c+2)*27+wp)+i+2] as i32;
a3 += v * weights[((c+3)*27+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+3] as i32;
a1 += v * weights[((c+1)*27+wp)+i+3] as i32;
a2 += v * weights[((c+2)*27+wp)+i+3] as i32;
a3 += v * weights[((c+3)*27+wp)+i+3] as i32;
i+=4; }
let v = input[ip+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+0] as i32;
a1 += v * weights[((c+1)*27+wp)+0] as i32;
a2 += v * weights[((c+2)*27+wp)+0] as i32;
a3 += v * weights[((c+3)*27+wp)+0] as i32;
let v = input[ip+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+1] as i32;
a1 += v * weights[((c+1)*27+wp)+1] as i32;
a2 += v * weights[((c+2)*27+wp)+1] as i32;
a3 += v * weights[((c+3)*27+wp)+1] as i32;
let v = input[ip+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+2] as i32;
a1 += v * weights[((c+1)*27+wp)+2] as i32;
a2 += v * weights[((c+2)*27+wp)+2] as i32;
a3 += v * weights[((c+3)*27+wp)+2] as i32;
} }
{ let iy=(y*2+2) as isize-1; let ix=(x*2+2) as isize-1;
if iy>=0 && iy<7 && ix>=0 && ix<9 {
let ip=(iy as usize*9+ix as usize)*3; let wp=24;
let mut i=0usize; while i < 0 {
let v = input[ip+i+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+0] as i32;
a1 += v * weights[((c+1)*27+wp)+i+0] as i32;
a2 += v * weights[((c+2)*27+wp)+i+0] as i32;
a3 += v * weights[((c+3)*27+wp)+i+0] as i32;
let v = input[ip+i+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+1] as i32;
a1 += v * weights[((c+1)*27+wp)+i+1] as i32;
a2 += v * weights[((c+2)*27+wp)+i+1] as i32;
a3 += v * weights[((c+3)*27+wp)+i+1] as i32;
let v = input[ip+i+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+2] as i32;
a1 += v * weights[((c+1)*27+wp)+i+2] as i32;
a2 += v * weights[((c+2)*27+wp)+i+2] as i32;
a3 += v * weights[((c+3)*27+wp)+i+2] as i32;
let v = input[ip+i+3] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+i+3] as i32;
a1 += v * weights[((c+1)*27+wp)+i+3] as i32;
a2 += v * weights[((c+2)*27+wp)+i+3] as i32;
a3 += v * weights[((c+3)*27+wp)+i+3] as i32;
i+=4; }
let v = input[ip+0] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+0] as i32;
a1 += v * weights[((c+1)*27+wp)+0] as i32;
a2 += v * weights[((c+2)*27+wp)+0] as i32;
a3 += v * weights[((c+3)*27+wp)+0] as i32;
let v = input[ip+1] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+1] as i32;
a1 += v * weights[((c+1)*27+wp)+1] as i32;
a2 += v * weights[((c+2)*27+wp)+1] as i32;
a3 += v * weights[((c+3)*27+wp)+1] as i32;
let v = input[ip+2] as i32 + -3;
a0 += v * weights[((c+0)*27+wp)+2] as i32;
a1 += v * weights[((c+1)*27+wp)+2] as i32;
a2 += v * weights[((c+2)*27+wp)+2] as i32;
a3 += v * weights[((c+3)*27+wp)+2] as i32;
} }
}
output[(y*5+x)*7+c+0] = (requantize(a0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*5+x)*7+c+1] = (requantize(a1,mult[c+1],shift[c+1])+5).clamp(-111,103) as i8;
output[(y*5+x)*7+c+2] = (requantize(a2,mult[c+2],shift[c+2])+5).clamp(-111,103) as i8;
output[(y*5+x)*7+c+3] = (requantize(a3,mult[c+3],shift[c+3])+5).clamp(-111,103) as i8;
c+=4; }
while c<7 {
let mut a0=bias[c+0];
if y*2>=1 && y*2+2<8 && x*2>=1 && x*2+2<10 {
{ let ip=((y*2+0-1)*9+x*2+0-1)*3; let wp=0;
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
}
{ let ip=((y*2+0-1)*9+x*2+1-1)*3; let wp=3;
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
}
{ let ip=((y*2+0-1)*9+x*2+2-1)*3; let wp=6;
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
}
{ let ip=((y*2+1-1)*9+x*2+0-1)*3; let wp=9;
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
}
{ let ip=((y*2+1-1)*9+x*2+1-1)*3; let wp=12;
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
}
{ let ip=((y*2+1-1)*9+x*2+2-1)*3; let wp=15;
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
}
{ let ip=((y*2+2-1)*9+x*2+0-1)*3; let wp=18;
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
}
{ let ip=((y*2+2-1)*9+x*2+1-1)*3; let wp=21;
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
}
{ let ip=((y*2+2-1)*9+x*2+2-1)*3; let wp=24;
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
}
} else {
{ let iy=(y*2+0) as isize-1; let ix=(x*2+0) as isize-1;
if iy>=0 && iy<7 && ix>=0 && ix<9 {
let ip=(iy as usize*9+ix as usize)*3; let wp=0;
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
{ let iy=(y*2+0) as isize-1; let ix=(x*2+1) as isize-1;
if iy>=0 && iy<7 && ix>=0 && ix<9 {
let ip=(iy as usize*9+ix as usize)*3; let wp=3;
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
{ let iy=(y*2+0) as isize-1; let ix=(x*2+2) as isize-1;
if iy>=0 && iy<7 && ix>=0 && ix<9 {
let ip=(iy as usize*9+ix as usize)*3; let wp=6;
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
{ let iy=(y*2+1) as isize-1; let ix=(x*2+0) as isize-1;
if iy>=0 && iy<7 && ix>=0 && ix<9 {
let ip=(iy as usize*9+ix as usize)*3; let wp=9;
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
{ let iy=(y*2+1) as isize-1; let ix=(x*2+1) as isize-1;
if iy>=0 && iy<7 && ix>=0 && ix<9 {
let ip=(iy as usize*9+ix as usize)*3; let wp=12;
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
{ let iy=(y*2+1) as isize-1; let ix=(x*2+2) as isize-1;
if iy>=0 && iy<7 && ix>=0 && ix<9 {
let ip=(iy as usize*9+ix as usize)*3; let wp=15;
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
{ let iy=(y*2+2) as isize-1; let ix=(x*2+0) as isize-1;
if iy>=0 && iy<7 && ix>=0 && ix<9 {
let ip=(iy as usize*9+ix as usize)*3; let wp=18;
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
{ let iy=(y*2+2) as isize-1; let ix=(x*2+1) as isize-1;
if iy>=0 && iy<7 && ix>=0 && ix<9 {
let ip=(iy as usize*9+ix as usize)*3; let wp=21;
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
{ let iy=(y*2+2) as isize-1; let ix=(x*2+2) as isize-1;
if iy>=0 && iy<7 && ix>=0 && ix<9 {
let ip=(iy as usize*9+ix as usize)*3; let wp=24;
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
}
output[(y*5+x)*7+c+0] = (requantize(a0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
c+=1; }
} } }
/// # Safety
/// Pointers must be aligned, valid for lengths 189, 189, 7, 7, 7, 140 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn kernel(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,189),core::slice::from_raw_parts(w,189),core::slice::from_raw_parts(b,7),core::slice::from_raw_parts(m,7),core::slice::from_raw_parts(s,7),core::slice::from_raw_parts_mut(o,140)); }
