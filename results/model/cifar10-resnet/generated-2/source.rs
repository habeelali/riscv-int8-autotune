#![no_std]
mod layer_0 {

// Generated for resnet_layer_0 with b8_u1_s0_im2col_qpos. Scratch: 54 bytes (local stack partial im2col, included in watermark).
// Input/weight/output lengths: 3072, 432, 16384.
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
assert!(input.len()>=3072 && weights.len()>=432 && output.len()>=16384);
assert!(bias.len()>=16 && mult.len()>=16 && shift.len()>=16);
// Quantization ranges and accumulator bound are proven by the model packer.
let mut column=[0i16;27];
for y in 0..32 { for x in 0..32 {
for ky in 0..3 { for kx in 0..3 { let iy=(y*1+ky) as isize-1; let ix=(x*1+kx) as isize-1; for i in 0..3 {
column[(ky*3+kx)*3+i]=if iy>=0 && iy<32 && ix>=0 && ix<32 {(input[(iy as usize*32+ix as usize)*3+i] as i32+128) as i16}else{0}; } } }
let mut c=0usize; while c<16 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
let mut a4=bias[c+4];
let mut a5=bias[c+5];
let mut a6=bias[c+6];
let mut a7=bias[c+7];
let mut i=0usize; while i<27 {
let v=column[i+0] as i32;
a0 += v * weights[(c+0)*27+i+0] as i32;
a1 += v * weights[(c+1)*27+i+0] as i32;
a2 += v * weights[(c+2)*27+i+0] as i32;
a3 += v * weights[(c+3)*27+i+0] as i32;
a4 += v * weights[(c+4)*27+i+0] as i32;
a5 += v * weights[(c+5)*27+i+0] as i32;
a6 += v * weights[(c+6)*27+i+0] as i32;
a7 += v * weights[(c+7)*27+i+0] as i32;
i+=1; }
output[(y*32+x)*16+c+0] = (requantize(a0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*32+x)*16+c+1] = (requantize(a1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*32+x)*16+c+2] = (requantize(a2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*32+x)*16+c+3] = (requantize(a3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
output[(y*32+x)*16+c+4] = (requantize(a4,mult[c+4],shift[c+4])+-128).clamp(-128,127) as i8;
output[(y*32+x)*16+c+5] = (requantize(a5,mult[c+5],shift[c+5])+-128).clamp(-128,127) as i8;
output[(y*32+x)*16+c+6] = (requantize(a6,mult[c+6],shift[c+6])+-128).clamp(-128,127) as i8;
output[(y*32+x)*16+c+7] = (requantize(a7,mult[c+7],shift[c+7])+-128).clamp(-128,127) as i8;
c+=8; }
} } }
/// # Safety
/// Parameters must satisfy the bundled host-validated quantization and accumulator-bound manifest.
/// Pointers must be aligned, valid for lengths 3072, 432, 16, 16, 16, 16384 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_0(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,3072),core::slice::from_raw_parts(w,432),core::slice::from_raw_parts(b,16),core::slice::from_raw_parts(m,16),core::slice::from_raw_parts(s,16),core::slice::from_raw_parts_mut(o,16384)); }

}
mod layer_1 {

// Generated for resnet_layer_1 with b8_u1_s0_im2col_qpos. Scratch: 288 bytes (local stack partial im2col, included in watermark).
// Input/weight/output lengths: 16384, 2304, 16384.
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
assert!(input.len()>=16384 && weights.len()>=2304 && output.len()>=16384);
assert!(bias.len()>=16 && mult.len()>=16 && shift.len()>=16);
// Quantization ranges and accumulator bound are proven by the model packer.
let mut column=[0i16;144];
for y in 0..32 { for x in 0..32 {
for ky in 0..3 { for kx in 0..3 { let iy=(y*1+ky) as isize-1; let ix=(x*1+kx) as isize-1; for i in 0..16 {
column[(ky*3+kx)*16+i]=if iy>=0 && iy<32 && ix>=0 && ix<32 {(input[(iy as usize*32+ix as usize)*16+i] as i32+128) as i16}else{0}; } } }
let mut c=0usize; while c<16 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
let mut a4=bias[c+4];
let mut a5=bias[c+5];
let mut a6=bias[c+6];
let mut a7=bias[c+7];
let mut i=0usize; while i<144 {
let v=column[i+0] as i32;
a0 += v * weights[(c+0)*144+i+0] as i32;
a1 += v * weights[(c+1)*144+i+0] as i32;
a2 += v * weights[(c+2)*144+i+0] as i32;
a3 += v * weights[(c+3)*144+i+0] as i32;
a4 += v * weights[(c+4)*144+i+0] as i32;
a5 += v * weights[(c+5)*144+i+0] as i32;
a6 += v * weights[(c+6)*144+i+0] as i32;
a7 += v * weights[(c+7)*144+i+0] as i32;
i+=1; }
output[(y*32+x)*16+c+0] = (requantize(a0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*32+x)*16+c+1] = (requantize(a1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*32+x)*16+c+2] = (requantize(a2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*32+x)*16+c+3] = (requantize(a3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
output[(y*32+x)*16+c+4] = (requantize(a4,mult[c+4],shift[c+4])+-128).clamp(-128,127) as i8;
output[(y*32+x)*16+c+5] = (requantize(a5,mult[c+5],shift[c+5])+-128).clamp(-128,127) as i8;
output[(y*32+x)*16+c+6] = (requantize(a6,mult[c+6],shift[c+6])+-128).clamp(-128,127) as i8;
output[(y*32+x)*16+c+7] = (requantize(a7,mult[c+7],shift[c+7])+-128).clamp(-128,127) as i8;
c+=8; }
} } }
/// # Safety
/// Parameters must satisfy the bundled host-validated quantization and accumulator-bound manifest.
/// Pointers must be aligned, valid for lengths 16384, 2304, 16, 16, 16, 16384 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_1(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,16384),core::slice::from_raw_parts(w,2304),core::slice::from_raw_parts(b,16),core::slice::from_raw_parts(m,16),core::slice::from_raw_parts(s,16),core::slice::from_raw_parts_mut(o,16384)); }

}
mod layer_2 {

// Generated for resnet_layer_2 with b8_u1_s0_im2col_qpos. Scratch: 288 bytes (local stack partial im2col, included in watermark).
// Input/weight/output lengths: 16384, 2304, 16384.
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
assert!(input.len()>=16384 && weights.len()>=2304 && output.len()>=16384);
assert!(bias.len()>=16 && mult.len()>=16 && shift.len()>=16);
// Quantization ranges and accumulator bound are proven by the model packer.
let mut column=[0i16;144];
for y in 0..32 { for x in 0..32 {
for ky in 0..3 { for kx in 0..3 { let iy=(y*1+ky) as isize-1; let ix=(x*1+kx) as isize-1; for i in 0..16 {
column[(ky*3+kx)*16+i]=if iy>=0 && iy<32 && ix>=0 && ix<32 {(input[(iy as usize*32+ix as usize)*16+i] as i32+128) as i16}else{0}; } } }
let mut c=0usize; while c<16 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
let mut a4=bias[c+4];
let mut a5=bias[c+5];
let mut a6=bias[c+6];
let mut a7=bias[c+7];
let mut i=0usize; while i<144 {
let v=column[i+0] as i32;
a0 += v * weights[(c+0)*144+i+0] as i32;
a1 += v * weights[(c+1)*144+i+0] as i32;
a2 += v * weights[(c+2)*144+i+0] as i32;
a3 += v * weights[(c+3)*144+i+0] as i32;
a4 += v * weights[(c+4)*144+i+0] as i32;
a5 += v * weights[(c+5)*144+i+0] as i32;
a6 += v * weights[(c+6)*144+i+0] as i32;
a7 += v * weights[(c+7)*144+i+0] as i32;
i+=1; }
output[(y*32+x)*16+c+0] = (requantize(a0,mult[c+0],shift[c+0])+4).clamp(-128,127) as i8;
output[(y*32+x)*16+c+1] = (requantize(a1,mult[c+1],shift[c+1])+4).clamp(-128,127) as i8;
output[(y*32+x)*16+c+2] = (requantize(a2,mult[c+2],shift[c+2])+4).clamp(-128,127) as i8;
output[(y*32+x)*16+c+3] = (requantize(a3,mult[c+3],shift[c+3])+4).clamp(-128,127) as i8;
output[(y*32+x)*16+c+4] = (requantize(a4,mult[c+4],shift[c+4])+4).clamp(-128,127) as i8;
output[(y*32+x)*16+c+5] = (requantize(a5,mult[c+5],shift[c+5])+4).clamp(-128,127) as i8;
output[(y*32+x)*16+c+6] = (requantize(a6,mult[c+6],shift[c+6])+4).clamp(-128,127) as i8;
output[(y*32+x)*16+c+7] = (requantize(a7,mult[c+7],shift[c+7])+4).clamp(-128,127) as i8;
c+=8; }
} } }
/// # Safety
/// Parameters must satisfy the bundled host-validated quantization and accumulator-bound manifest.
/// Pointers must be aligned, valid for lengths 16384, 2304, 16, 16, 16, 16384 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_2(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,16384),core::slice::from_raw_parts(w,2304),core::slice::from_raw_parts(b,16),core::slice::from_raw_parts(m,16),core::slice::from_raw_parts(s,16),core::slice::from_raw_parts_mut(o,16384)); }

}
mod layer_4 {

// Generated for resnet_layer_4 with b8_u1_s0_im2col_qpos. Scratch: 288 bytes (local stack partial im2col, included in watermark).
// Input/weight/output lengths: 17424, 4608, 8192.
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
assert!(input.len()>=17424 && weights.len()>=4608 && output.len()>=8192);
assert!(bias.len()>=32 && mult.len()>=32 && shift.len()>=32);
// Quantization ranges and accumulator bound are proven by the model packer.
let mut column=[0i16;144];
for y in 0..16 { for x in 0..16 {
for ky in 0..3 { for kx in 0..3 { let iy=(y*2+ky) as isize-0; let ix=(x*2+kx) as isize-0; for i in 0..16 {
column[(ky*3+kx)*16+i]=if iy>=0 && iy<33 && ix>=0 && ix<33 {(input[(iy as usize*33+ix as usize)*16+i] as i32+0) as i16}else{0}; } } }
let mut c=0usize; while c<32 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
let mut a4=bias[c+4];
let mut a5=bias[c+5];
let mut a6=bias[c+6];
let mut a7=bias[c+7];
let mut i=0usize; while i<144 {
let v=column[i+0] as i32;
a0 += v * weights[(c+0)*144+i+0] as i32;
a1 += v * weights[(c+1)*144+i+0] as i32;
a2 += v * weights[(c+2)*144+i+0] as i32;
a3 += v * weights[(c+3)*144+i+0] as i32;
a4 += v * weights[(c+4)*144+i+0] as i32;
a5 += v * weights[(c+5)*144+i+0] as i32;
a6 += v * weights[(c+6)*144+i+0] as i32;
a7 += v * weights[(c+7)*144+i+0] as i32;
i+=1; }
output[(y*16+x)*32+c+0] = (requantize(a0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*16+x)*32+c+1] = (requantize(a1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*16+x)*32+c+2] = (requantize(a2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*16+x)*32+c+3] = (requantize(a3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
output[(y*16+x)*32+c+4] = (requantize(a4,mult[c+4],shift[c+4])+-128).clamp(-128,127) as i8;
output[(y*16+x)*32+c+5] = (requantize(a5,mult[c+5],shift[c+5])+-128).clamp(-128,127) as i8;
output[(y*16+x)*32+c+6] = (requantize(a6,mult[c+6],shift[c+6])+-128).clamp(-128,127) as i8;
output[(y*16+x)*32+c+7] = (requantize(a7,mult[c+7],shift[c+7])+-128).clamp(-128,127) as i8;
c+=8; }
} } }
/// # Safety
/// Parameters must satisfy the bundled host-validated quantization and accumulator-bound manifest.
/// Pointers must be aligned, valid for lengths 17424, 4608, 32, 32, 32, 8192 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_4(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,17424),core::slice::from_raw_parts(w,4608),core::slice::from_raw_parts(b,32),core::slice::from_raw_parts(m,32),core::slice::from_raw_parts(s,32),core::slice::from_raw_parts_mut(o,8192)); }

}
mod layer_5 {

// Generated for resnet_layer_5 with b8_u1_s0_im2col_qpos. Scratch: 576 bytes (local stack partial im2col, included in watermark).
// Input/weight/output lengths: 8192, 9216, 8192.
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
assert!(input.len()>=8192 && weights.len()>=9216 && output.len()>=8192);
assert!(bias.len()>=32 && mult.len()>=32 && shift.len()>=32);
// Quantization ranges and accumulator bound are proven by the model packer.
let mut column=[0i16;288];
for y in 0..16 { for x in 0..16 {
for ky in 0..3 { for kx in 0..3 { let iy=(y*1+ky) as isize-1; let ix=(x*1+kx) as isize-1; for i in 0..32 {
column[(ky*3+kx)*32+i]=if iy>=0 && iy<16 && ix>=0 && ix<16 {(input[(iy as usize*16+ix as usize)*32+i] as i32+128) as i16}else{0}; } } }
let mut c=0usize; while c<32 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
let mut a4=bias[c+4];
let mut a5=bias[c+5];
let mut a6=bias[c+6];
let mut a7=bias[c+7];
let mut i=0usize; while i<288 {
let v=column[i+0] as i32;
a0 += v * weights[(c+0)*288+i+0] as i32;
a1 += v * weights[(c+1)*288+i+0] as i32;
a2 += v * weights[(c+2)*288+i+0] as i32;
a3 += v * weights[(c+3)*288+i+0] as i32;
a4 += v * weights[(c+4)*288+i+0] as i32;
a5 += v * weights[(c+5)*288+i+0] as i32;
a6 += v * weights[(c+6)*288+i+0] as i32;
a7 += v * weights[(c+7)*288+i+0] as i32;
i+=1; }
output[(y*16+x)*32+c+0] = (requantize(a0,mult[c+0],shift[c+0])+4).clamp(-128,127) as i8;
output[(y*16+x)*32+c+1] = (requantize(a1,mult[c+1],shift[c+1])+4).clamp(-128,127) as i8;
output[(y*16+x)*32+c+2] = (requantize(a2,mult[c+2],shift[c+2])+4).clamp(-128,127) as i8;
output[(y*16+x)*32+c+3] = (requantize(a3,mult[c+3],shift[c+3])+4).clamp(-128,127) as i8;
output[(y*16+x)*32+c+4] = (requantize(a4,mult[c+4],shift[c+4])+4).clamp(-128,127) as i8;
output[(y*16+x)*32+c+5] = (requantize(a5,mult[c+5],shift[c+5])+4).clamp(-128,127) as i8;
output[(y*16+x)*32+c+6] = (requantize(a6,mult[c+6],shift[c+6])+4).clamp(-128,127) as i8;
output[(y*16+x)*32+c+7] = (requantize(a7,mult[c+7],shift[c+7])+4).clamp(-128,127) as i8;
c+=8; }
} } }
/// # Safety
/// Parameters must satisfy the bundled host-validated quantization and accumulator-bound manifest.
/// Pointers must be aligned, valid for lengths 8192, 9216, 32, 32, 32, 8192 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_5(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,8192),core::slice::from_raw_parts(w,9216),core::slice::from_raw_parts(b,32),core::slice::from_raw_parts(m,32),core::slice::from_raw_parts(s,32),core::slice::from_raw_parts_mut(o,8192)); }

}
mod layer_6 {

// Generated for resnet_layer_6 with b8_u1_s0_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 16384, 512, 8192.
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
assert!(input.len()>=16384 && weights.len()>=512 && output.len()>=8192);
assert!(bias.len()>=32 && mult.len()>=32 && shift.len()>=32);
// Quantization ranges and accumulator bound are proven by the model packer.
for y in 0..16 { for x in 0..16 {
let mut c=0usize; while c<32 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
let mut a4=bias[c+4];
let mut a5=bias[c+5];
let mut a6=bias[c+6];
let mut a7=bias[c+7];
let by=(y*2) as isize-0; let bx=(x*2) as isize-0;
let ys=0isize.max(-by) as usize; let ye=(1isize).min(32isize-by) as usize;
let xs=0isize.max(-bx) as usize; let xe=(1isize).min(32isize-bx) as usize;
for ky in ys..ye { for kx in xs..xe {
let ip=(((by+ky as isize) as usize)*32+(bx+kx as isize) as usize)*16; let wp=(ky*1+kx)*16;
let mut i=0usize; while i < 16 {
let v = input[ip+i+0] as i32 + 0;
a0 += v * weights[((c+0)*16+wp)+i+0] as i32;
a1 += v * weights[((c+1)*16+wp)+i+0] as i32;
a2 += v * weights[((c+2)*16+wp)+i+0] as i32;
a3 += v * weights[((c+3)*16+wp)+i+0] as i32;
a4 += v * weights[((c+4)*16+wp)+i+0] as i32;
a5 += v * weights[((c+5)*16+wp)+i+0] as i32;
a6 += v * weights[((c+6)*16+wp)+i+0] as i32;
a7 += v * weights[((c+7)*16+wp)+i+0] as i32;
i+=1; }
} }
output[(y*16+x)*32+c+0] = (requantize(a0,mult[c+0],shift[c+0])+-17).clamp(-128,127) as i8;
output[(y*16+x)*32+c+1] = (requantize(a1,mult[c+1],shift[c+1])+-17).clamp(-128,127) as i8;
output[(y*16+x)*32+c+2] = (requantize(a2,mult[c+2],shift[c+2])+-17).clamp(-128,127) as i8;
output[(y*16+x)*32+c+3] = (requantize(a3,mult[c+3],shift[c+3])+-17).clamp(-128,127) as i8;
output[(y*16+x)*32+c+4] = (requantize(a4,mult[c+4],shift[c+4])+-17).clamp(-128,127) as i8;
output[(y*16+x)*32+c+5] = (requantize(a5,mult[c+5],shift[c+5])+-17).clamp(-128,127) as i8;
output[(y*16+x)*32+c+6] = (requantize(a6,mult[c+6],shift[c+6])+-17).clamp(-128,127) as i8;
output[(y*16+x)*32+c+7] = (requantize(a7,mult[c+7],shift[c+7])+-17).clamp(-128,127) as i8;
c+=8; }
} } }
/// # Safety
/// Parameters must satisfy the bundled host-validated quantization and accumulator-bound manifest.
/// Pointers must be aligned, valid for lengths 16384, 512, 32, 32, 32, 8192 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_6(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,16384),core::slice::from_raw_parts(w,512),core::slice::from_raw_parts(b,32),core::slice::from_raw_parts(m,32),core::slice::from_raw_parts(s,32),core::slice::from_raw_parts_mut(o,8192)); }

}
mod layer_8 {

// Generated for resnet_layer_8 with b8_u1_s0_im2col_qpos. Scratch: 576 bytes (local stack partial im2col, included in watermark).
// Input/weight/output lengths: 9248, 18432, 4096.
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
assert!(input.len()>=9248 && weights.len()>=18432 && output.len()>=4096);
assert!(bias.len()>=64 && mult.len()>=64 && shift.len()>=64);
// Quantization ranges and accumulator bound are proven by the model packer.
let mut column=[0i16;288];
for y in 0..8 { for x in 0..8 {
for ky in 0..3 { for kx in 0..3 { let iy=(y*2+ky) as isize-0; let ix=(x*2+kx) as isize-0; for i in 0..32 {
column[(ky*3+kx)*32+i]=if iy>=0 && iy<17 && ix>=0 && ix<17 {(input[(iy as usize*17+ix as usize)*32+i] as i32+0) as i16}else{0}; } } }
let mut c=0usize; while c<64 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
let mut a4=bias[c+4];
let mut a5=bias[c+5];
let mut a6=bias[c+6];
let mut a7=bias[c+7];
let mut i=0usize; while i<288 {
let v=column[i+0] as i32;
a0 += v * weights[(c+0)*288+i+0] as i32;
a1 += v * weights[(c+1)*288+i+0] as i32;
a2 += v * weights[(c+2)*288+i+0] as i32;
a3 += v * weights[(c+3)*288+i+0] as i32;
a4 += v * weights[(c+4)*288+i+0] as i32;
a5 += v * weights[(c+5)*288+i+0] as i32;
a6 += v * weights[(c+6)*288+i+0] as i32;
a7 += v * weights[(c+7)*288+i+0] as i32;
i+=1; }
output[(y*8+x)*64+c+0] = (requantize(a0,mult[c+0],shift[c+0])+-128).clamp(-128,127) as i8;
output[(y*8+x)*64+c+1] = (requantize(a1,mult[c+1],shift[c+1])+-128).clamp(-128,127) as i8;
output[(y*8+x)*64+c+2] = (requantize(a2,mult[c+2],shift[c+2])+-128).clamp(-128,127) as i8;
output[(y*8+x)*64+c+3] = (requantize(a3,mult[c+3],shift[c+3])+-128).clamp(-128,127) as i8;
output[(y*8+x)*64+c+4] = (requantize(a4,mult[c+4],shift[c+4])+-128).clamp(-128,127) as i8;
output[(y*8+x)*64+c+5] = (requantize(a5,mult[c+5],shift[c+5])+-128).clamp(-128,127) as i8;
output[(y*8+x)*64+c+6] = (requantize(a6,mult[c+6],shift[c+6])+-128).clamp(-128,127) as i8;
output[(y*8+x)*64+c+7] = (requantize(a7,mult[c+7],shift[c+7])+-128).clamp(-128,127) as i8;
c+=8; }
} } }
/// # Safety
/// Parameters must satisfy the bundled host-validated quantization and accumulator-bound manifest.
/// Pointers must be aligned, valid for lengths 9248, 18432, 64, 64, 64, 4096 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_8(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,9248),core::slice::from_raw_parts(w,18432),core::slice::from_raw_parts(b,64),core::slice::from_raw_parts(m,64),core::slice::from_raw_parts(s,64),core::slice::from_raw_parts_mut(o,4096)); }

}
mod layer_9 {

// Generated for resnet_layer_9 with b8_u1_s0_im2col_qpos. Scratch: 1152 bytes (local stack partial im2col, included in watermark).
// Input/weight/output lengths: 4096, 36864, 4096.
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
assert!(input.len()>=4096 && weights.len()>=36864 && output.len()>=4096);
assert!(bias.len()>=64 && mult.len()>=64 && shift.len()>=64);
// Quantization ranges and accumulator bound are proven by the model packer.
let mut column=[0i16;576];
for y in 0..8 { for x in 0..8 {
for ky in 0..3 { for kx in 0..3 { let iy=(y*1+ky) as isize-1; let ix=(x*1+kx) as isize-1; for i in 0..64 {
column[(ky*3+kx)*64+i]=if iy>=0 && iy<8 && ix>=0 && ix<8 {(input[(iy as usize*8+ix as usize)*64+i] as i32+128) as i16}else{0}; } } }
let mut c=0usize; while c<64 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
let mut a4=bias[c+4];
let mut a5=bias[c+5];
let mut a6=bias[c+6];
let mut a7=bias[c+7];
let mut i=0usize; while i<576 {
let v=column[i+0] as i32;
a0 += v * weights[(c+0)*576+i+0] as i32;
a1 += v * weights[(c+1)*576+i+0] as i32;
a2 += v * weights[(c+2)*576+i+0] as i32;
a3 += v * weights[(c+3)*576+i+0] as i32;
a4 += v * weights[(c+4)*576+i+0] as i32;
a5 += v * weights[(c+5)*576+i+0] as i32;
a6 += v * weights[(c+6)*576+i+0] as i32;
a7 += v * weights[(c+7)*576+i+0] as i32;
i+=1; }
output[(y*8+x)*64+c+0] = (requantize(a0,mult[c+0],shift[c+0])+-2).clamp(-128,127) as i8;
output[(y*8+x)*64+c+1] = (requantize(a1,mult[c+1],shift[c+1])+-2).clamp(-128,127) as i8;
output[(y*8+x)*64+c+2] = (requantize(a2,mult[c+2],shift[c+2])+-2).clamp(-128,127) as i8;
output[(y*8+x)*64+c+3] = (requantize(a3,mult[c+3],shift[c+3])+-2).clamp(-128,127) as i8;
output[(y*8+x)*64+c+4] = (requantize(a4,mult[c+4],shift[c+4])+-2).clamp(-128,127) as i8;
output[(y*8+x)*64+c+5] = (requantize(a5,mult[c+5],shift[c+5])+-2).clamp(-128,127) as i8;
output[(y*8+x)*64+c+6] = (requantize(a6,mult[c+6],shift[c+6])+-2).clamp(-128,127) as i8;
output[(y*8+x)*64+c+7] = (requantize(a7,mult[c+7],shift[c+7])+-2).clamp(-128,127) as i8;
c+=8; }
} } }
/// # Safety
/// Parameters must satisfy the bundled host-validated quantization and accumulator-bound manifest.
/// Pointers must be aligned, valid for lengths 4096, 36864, 64, 64, 64, 4096 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_9(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,4096),core::slice::from_raw_parts(w,36864),core::slice::from_raw_parts(b,64),core::slice::from_raw_parts(m,64),core::slice::from_raw_parts(s,64),core::slice::from_raw_parts_mut(o,4096)); }

}
mod layer_10 {

// Generated for resnet_layer_10 with b8_u1_s0_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 8192, 2048, 4096.
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
assert!(input.len()>=8192 && weights.len()>=2048 && output.len()>=4096);
assert!(bias.len()>=64 && mult.len()>=64 && shift.len()>=64);
// Quantization ranges and accumulator bound are proven by the model packer.
for y in 0..8 { for x in 0..8 {
let mut c=0usize; while c<64 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
let mut a4=bias[c+4];
let mut a5=bias[c+5];
let mut a6=bias[c+6];
let mut a7=bias[c+7];
let by=(y*2) as isize-0; let bx=(x*2) as isize-0;
let ys=0isize.max(-by) as usize; let ye=(1isize).min(16isize-by) as usize;
let xs=0isize.max(-bx) as usize; let xe=(1isize).min(16isize-bx) as usize;
for ky in ys..ye { for kx in xs..xe {
let ip=(((by+ky as isize) as usize)*16+(bx+kx as isize) as usize)*32; let wp=(ky*1+kx)*32;
let mut i=0usize; while i < 32 {
let v = input[ip+i+0] as i32 + 0;
a0 += v * weights[((c+0)*32+wp)+i+0] as i32;
a1 += v * weights[((c+1)*32+wp)+i+0] as i32;
a2 += v * weights[((c+2)*32+wp)+i+0] as i32;
a3 += v * weights[((c+3)*32+wp)+i+0] as i32;
a4 += v * weights[((c+4)*32+wp)+i+0] as i32;
a5 += v * weights[((c+5)*32+wp)+i+0] as i32;
a6 += v * weights[((c+6)*32+wp)+i+0] as i32;
a7 += v * weights[((c+7)*32+wp)+i+0] as i32;
i+=1; }
} }
output[(y*8+x)*64+c+0] = (requantize(a0,mult[c+0],shift[c+0])+38).clamp(-128,127) as i8;
output[(y*8+x)*64+c+1] = (requantize(a1,mult[c+1],shift[c+1])+38).clamp(-128,127) as i8;
output[(y*8+x)*64+c+2] = (requantize(a2,mult[c+2],shift[c+2])+38).clamp(-128,127) as i8;
output[(y*8+x)*64+c+3] = (requantize(a3,mult[c+3],shift[c+3])+38).clamp(-128,127) as i8;
output[(y*8+x)*64+c+4] = (requantize(a4,mult[c+4],shift[c+4])+38).clamp(-128,127) as i8;
output[(y*8+x)*64+c+5] = (requantize(a5,mult[c+5],shift[c+5])+38).clamp(-128,127) as i8;
output[(y*8+x)*64+c+6] = (requantize(a6,mult[c+6],shift[c+6])+38).clamp(-128,127) as i8;
output[(y*8+x)*64+c+7] = (requantize(a7,mult[c+7],shift[c+7])+38).clamp(-128,127) as i8;
c+=8; }
} } }
/// # Safety
/// Parameters must satisfy the bundled host-validated quantization and accumulator-bound manifest.
/// Pointers must be aligned, valid for lengths 8192, 2048, 64, 64, 64, 4096 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_10(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,8192),core::slice::from_raw_parts(w,2048),core::slice::from_raw_parts(b,64),core::slice::from_raw_parts(m,64),core::slice::from_raw_parts(s,64),core::slice::from_raw_parts_mut(o,4096)); }

}
mod layer_14 {

// Generated for resnet_layer_14 with b8_u1_s0_qpos. Scratch: 0 bytes.
// Input/weight/output lengths: 64, 640, 10.
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
assert!(input.len()>=64 && weights.len()>=640 && output.len()>=10);
assert!(bias.len()>=10 && mult.len()>=10 && shift.len()>=10);
// Quantization ranges and accumulator bound are proven by the model packer.
for y in 0..1 { for x in 0..1 {
let mut c=0usize; while c<8 {
let mut a0=bias[c+0];
let mut a1=bias[c+1];
let mut a2=bias[c+2];
let mut a3=bias[c+3];
let mut a4=bias[c+4];
let mut a5=bias[c+5];
let mut a6=bias[c+6];
let mut a7=bias[c+7];
let by=(y*1) as isize-0; let bx=(x*1) as isize-0;
let ys=0isize.max(-by) as usize; let ye=(1isize).min(1isize-by) as usize;
let xs=0isize.max(-bx) as usize; let xe=(1isize).min(1isize-bx) as usize;
for ky in ys..ye { for kx in xs..xe {
let ip=(((by+ky as isize) as usize)*1+(bx+kx as isize) as usize)*64; let wp=(ky*1+kx)*64;
let mut i=0usize; while i < 64 {
let v = input[ip+i+0] as i32 + 0;
a0 += v * weights[((c+0)*64+wp)+i+0] as i32;
a1 += v * weights[((c+1)*64+wp)+i+0] as i32;
a2 += v * weights[((c+2)*64+wp)+i+0] as i32;
a3 += v * weights[((c+3)*64+wp)+i+0] as i32;
a4 += v * weights[((c+4)*64+wp)+i+0] as i32;
a5 += v * weights[((c+5)*64+wp)+i+0] as i32;
a6 += v * weights[((c+6)*64+wp)+i+0] as i32;
a7 += v * weights[((c+7)*64+wp)+i+0] as i32;
i+=1; }
} }
output[(y*1+x)*10+c+0] = (requantize(a0,mult[c+0],shift[c+0])+24).clamp(-128,127) as i8;
output[(y*1+x)*10+c+1] = (requantize(a1,mult[c+1],shift[c+1])+24).clamp(-128,127) as i8;
output[(y*1+x)*10+c+2] = (requantize(a2,mult[c+2],shift[c+2])+24).clamp(-128,127) as i8;
output[(y*1+x)*10+c+3] = (requantize(a3,mult[c+3],shift[c+3])+24).clamp(-128,127) as i8;
output[(y*1+x)*10+c+4] = (requantize(a4,mult[c+4],shift[c+4])+24).clamp(-128,127) as i8;
output[(y*1+x)*10+c+5] = (requantize(a5,mult[c+5],shift[c+5])+24).clamp(-128,127) as i8;
output[(y*1+x)*10+c+6] = (requantize(a6,mult[c+6],shift[c+6])+24).clamp(-128,127) as i8;
output[(y*1+x)*10+c+7] = (requantize(a7,mult[c+7],shift[c+7])+24).clamp(-128,127) as i8;
c+=8; }
while c<10 {
let mut a0=bias[c+0];
let by=(y*1) as isize-0; let bx=(x*1) as isize-0;
let ys=0isize.max(-by) as usize; let ye=(1isize).min(1isize-by) as usize;
let xs=0isize.max(-bx) as usize; let xe=(1isize).min(1isize-bx) as usize;
for ky in ys..ye { for kx in xs..xe {
let ip=(((by+ky as isize) as usize)*1+(bx+kx as isize) as usize)*64; let wp=(ky*1+kx)*64;
let mut i=0usize; while i < 64 {
let v = input[ip+i+0] as i32 + 0;
a0 += v * weights[((c+0)*64+wp)+i+0] as i32;
i+=1; }
} }
output[(y*1+x)*10+c+0] = (requantize(a0,mult[c+0],shift[c+0])+24).clamp(-128,127) as i8;
c+=1; }
} } }
/// # Safety
/// Parameters must satisfy the bundled host-validated quantization and accumulator-bound manifest.
/// Pointers must be aligned, valid for lengths 64, 640, 10, 10, 10, 10 respectively.
/// Output must not overlap inputs. Quantization contract is documented above.
#[no_mangle]
pub unsafe extern "C" fn layer_14(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
run(core::slice::from_raw_parts(i,64),core::slice::from_raw_parts(w,640),core::slice::from_raw_parts(b,10),core::slice::from_raw_parts(m,10),core::slice::from_raw_parts(s,10),core::slice::from_raw_parts_mut(o,10)); }

}
#[panic_handler] fn panic(_: &core::panic::PanicInfo) -> ! { loop { core::hint::spin_loop(); } }
