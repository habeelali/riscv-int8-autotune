#![no_std]
// Generated for depthwise_small with b8_u1_s1_interior. Scratch: 0 bytes.
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
let mut c=0usize; while c<0 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
let mut a4=bias[c+4];
let mut a5=bias[c+5];
let mut a6=bias[c+6];
let mut a7=bias[c+7];
if y*2>=1 && y*2+2<10 && x*2>=1 && x*2+2<8 {
{ let ip=((y*2+0-1)*7+x*2+0-1)*5; let wp=0;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + -3) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + -3) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + -3) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + -3) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + -3) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + -3) * weights[wp+c+7] as i32;
}
{ let ip=((y*2+0-1)*7+x*2+1-1)*5; let wp=5;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + -3) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + -3) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + -3) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + -3) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + -3) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + -3) * weights[wp+c+7] as i32;
}
{ let ip=((y*2+0-1)*7+x*2+2-1)*5; let wp=10;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + -3) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + -3) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + -3) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + -3) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + -3) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + -3) * weights[wp+c+7] as i32;
}
{ let ip=((y*2+1-1)*7+x*2+0-1)*5; let wp=15;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + -3) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + -3) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + -3) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + -3) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + -3) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + -3) * weights[wp+c+7] as i32;
}
{ let ip=((y*2+1-1)*7+x*2+1-1)*5; let wp=20;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + -3) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + -3) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + -3) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + -3) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + -3) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + -3) * weights[wp+c+7] as i32;
}
{ let ip=((y*2+1-1)*7+x*2+2-1)*5; let wp=25;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + -3) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + -3) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + -3) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + -3) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + -3) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + -3) * weights[wp+c+7] as i32;
}
{ let ip=((y*2+2-1)*7+x*2+0-1)*5; let wp=30;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + -3) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + -3) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + -3) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + -3) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + -3) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + -3) * weights[wp+c+7] as i32;
}
{ let ip=((y*2+2-1)*7+x*2+1-1)*5; let wp=35;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + -3) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + -3) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + -3) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + -3) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + -3) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + -3) * weights[wp+c+7] as i32;
}
{ let ip=((y*2+2-1)*7+x*2+2-1)*5; let wp=40;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + -3) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + -3) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + -3) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + -3) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + -3) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + -3) * weights[wp+c+7] as i32;
}
} else {
{ let iy=(y*2+0) as isize-1; let ix=(x*2+0) as isize-1;
if iy>=0 && iy<9 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*5; let wp=0;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + -3) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + -3) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + -3) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + -3) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + -3) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + -3) * weights[wp+c+7] as i32;
} }
{ let iy=(y*2+0) as isize-1; let ix=(x*2+1) as isize-1;
if iy>=0 && iy<9 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*5; let wp=5;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + -3) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + -3) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + -3) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + -3) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + -3) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + -3) * weights[wp+c+7] as i32;
} }
{ let iy=(y*2+0) as isize-1; let ix=(x*2+2) as isize-1;
if iy>=0 && iy<9 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*5; let wp=10;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + -3) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + -3) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + -3) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + -3) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + -3) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + -3) * weights[wp+c+7] as i32;
} }
{ let iy=(y*2+1) as isize-1; let ix=(x*2+0) as isize-1;
if iy>=0 && iy<9 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*5; let wp=15;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + -3) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + -3) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + -3) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + -3) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + -3) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + -3) * weights[wp+c+7] as i32;
} }
{ let iy=(y*2+1) as isize-1; let ix=(x*2+1) as isize-1;
if iy>=0 && iy<9 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*5; let wp=20;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + -3) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + -3) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + -3) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + -3) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + -3) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + -3) * weights[wp+c+7] as i32;
} }
{ let iy=(y*2+1) as isize-1; let ix=(x*2+2) as isize-1;
if iy>=0 && iy<9 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*5; let wp=25;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + -3) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + -3) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + -3) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + -3) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + -3) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + -3) * weights[wp+c+7] as i32;
} }
{ let iy=(y*2+2) as isize-1; let ix=(x*2+0) as isize-1;
if iy>=0 && iy<9 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*5; let wp=30;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + -3) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + -3) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + -3) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + -3) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + -3) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + -3) * weights[wp+c+7] as i32;
} }
{ let iy=(y*2+2) as isize-1; let ix=(x*2+1) as isize-1;
if iy>=0 && iy<9 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*5; let wp=35;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + -3) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + -3) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + -3) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + -3) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + -3) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + -3) * weights[wp+c+7] as i32;
} }
{ let iy=(y*2+2) as isize-1; let ix=(x*2+2) as isize-1;
if iy>=0 && iy<9 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*5; let wp=40;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
a1 += (input[ip+c+1] as i32 + -3) * weights[wp+c+1] as i32;
a2 += (input[ip+c+2] as i32 + -3) * weights[wp+c+2] as i32;
a3 += (input[ip+c+3] as i32 + -3) * weights[wp+c+3] as i32;
a4 += (input[ip+c+4] as i32 + -3) * weights[wp+c+4] as i32;
a5 += (input[ip+c+5] as i32 + -3) * weights[wp+c+5] as i32;
a6 += (input[ip+c+6] as i32 + -3) * weights[wp+c+6] as i32;
a7 += (input[ip+c+7] as i32 + -3) * weights[wp+c+7] as i32;
} }
}
output[(y*4+x)*5+c+0] = (requantize(a0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
output[(y*4+x)*5+c+1] = (requantize(a1,mult[c+1],shift[c+1])+5).clamp(-111,103) as i8;
output[(y*4+x)*5+c+2] = (requantize(a2,mult[c+2],shift[c+2])+5).clamp(-111,103) as i8;
output[(y*4+x)*5+c+3] = (requantize(a3,mult[c+3],shift[c+3])+5).clamp(-111,103) as i8;
output[(y*4+x)*5+c+4] = (requantize(a4,mult[c+4],shift[c+4])+5).clamp(-111,103) as i8;
output[(y*4+x)*5+c+5] = (requantize(a5,mult[c+5],shift[c+5])+5).clamp(-111,103) as i8;
output[(y*4+x)*5+c+6] = (requantize(a6,mult[c+6],shift[c+6])+5).clamp(-111,103) as i8;
output[(y*4+x)*5+c+7] = (requantize(a7,mult[c+7],shift[c+7])+5).clamp(-111,103) as i8;
c+=8; }
while c<5 {
let mut a0=bias[c+0];
if y*2>=1 && y*2+2<10 && x*2>=1 && x*2+2<8 {
{ let ip=((y*2+0-1)*7+x*2+0-1)*5; let wp=0;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
}
{ let ip=((y*2+0-1)*7+x*2+1-1)*5; let wp=5;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
}
{ let ip=((y*2+0-1)*7+x*2+2-1)*5; let wp=10;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
}
{ let ip=((y*2+1-1)*7+x*2+0-1)*5; let wp=15;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
}
{ let ip=((y*2+1-1)*7+x*2+1-1)*5; let wp=20;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
}
{ let ip=((y*2+1-1)*7+x*2+2-1)*5; let wp=25;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
}
{ let ip=((y*2+2-1)*7+x*2+0-1)*5; let wp=30;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
}
{ let ip=((y*2+2-1)*7+x*2+1-1)*5; let wp=35;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
}
{ let ip=((y*2+2-1)*7+x*2+2-1)*5; let wp=40;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
}
} else {
{ let iy=(y*2+0) as isize-1; let ix=(x*2+0) as isize-1;
if iy>=0 && iy<9 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*5; let wp=0;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+0) as isize-1; let ix=(x*2+1) as isize-1;
if iy>=0 && iy<9 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*5; let wp=5;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+0) as isize-1; let ix=(x*2+2) as isize-1;
if iy>=0 && iy<9 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*5; let wp=10;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+1) as isize-1; let ix=(x*2+0) as isize-1;
if iy>=0 && iy<9 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*5; let wp=15;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+1) as isize-1; let ix=(x*2+1) as isize-1;
if iy>=0 && iy<9 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*5; let wp=20;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+1) as isize-1; let ix=(x*2+2) as isize-1;
if iy>=0 && iy<9 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*5; let wp=25;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+2) as isize-1; let ix=(x*2+0) as isize-1;
if iy>=0 && iy<9 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*5; let wp=30;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+2) as isize-1; let ix=(x*2+1) as isize-1;
if iy>=0 && iy<9 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*5; let wp=35;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
} }
{ let iy=(y*2+2) as isize-1; let ix=(x*2+2) as isize-1;
if iy>=0 && iy<9 && ix>=0 && ix<7 {
let ip=(iy as usize*7+ix as usize)*5; let wp=40;
a0 += (input[ip+c+0] as i32 + -3) * weights[wp+c+0] as i32;
} }
}
output[(y*4+x)*5+c+0] = (requantize(a0,mult[c+0],shift[c+0])+5).clamp(-111,103) as i8;
c+=1; }
} } }
/// # Safety
/// Pointers must be aligned, valid for lengths 315, 45, 5, 5, 5, 100 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn kernel(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,315),core::slice::from_raw_parts(w,45),core::slice::from_raw_parts(b,5),core::slice::from_raw_parts(m,5),core::slice::from_raw_parts(s,5),core::slice::from_raw_parts_mut(o,100)); }
