#![no_std]
//! Scalar INT8 operators. NHWC input/output; OHWI standard and HWC depthwise weights.
//! Kernels allocate no heap and need no scratch. The caller owns all tensor storage.

#[derive(Clone, Copy, Debug)]
pub struct Params {
    pub height: usize,
    pub width: usize,
    pub input_channels: usize,
    pub output_channels: usize,
    pub kernel: usize,
    pub stride: usize,
    pub padding: usize,
    pub depthwise: bool,
    pub input_offset: i32,
    pub output_offset: i32,
    pub activation_min: i32,
    pub activation_max: i32,
}
impl Params {
    pub fn output_height(self) -> usize {
        (self.height + 2 * self.padding - self.kernel) / self.stride + 1
    }
    pub fn output_width(self) -> usize {
        (self.width + 2 * self.padding - self.kernel) / self.stride + 1
    }
    pub fn input_len(self) -> usize {
        self.height * self.width * self.input_channels
    }
    pub fn weights_len(self) -> usize {
        self.kernel
            * self.kernel
            * self.input_channels
            * if self.depthwise {
                1
            } else {
                self.output_channels
            }
    }
    pub fn output_len(self) -> usize {
        self.output_height() * self.output_width() * self.output_channels
    }
}

/// ESP-NN default two-rounding requantization. Shifts must lie in -31..=30;
/// left-shifted accumulators must fit INT32, as required by the C baseline.
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

/// Generic correct reference: runtime dimensions, scalar loops, fused epilogue.
/// Panics on invalid buffer sizes or unsupported depthwise channel multiplier.
/// Accumulation must fit INT32. Inputs, weights and output cannot alias in safe Rust.
#[inline(never)]
pub fn generic(
    p: &Params,
    input: &[i8],
    weights: &[i8],
    bias: &[i32],
    mult: &[i32],
    shift: &[i32],
    output: &mut [i8],
) {
    assert!(input.len() >= p.input_len() && weights.len() >= p.weights_len());
    assert!(output.len() >= p.output_len());
    assert!(
        bias.len() >= p.output_channels
            && mult.len() >= p.output_channels
            && shift.len() >= p.output_channels
    );
    assert!(!p.depthwise || p.input_channels == p.output_channels);
    for y in 0..p.output_height() {
        for x in 0..p.output_width() {
            for c in 0..p.output_channels {
                let mut acc = bias[c];
                for ky in 0..p.kernel {
                    let iy = (y * p.stride + ky) as isize - p.padding as isize;
                    for kx in 0..p.kernel {
                        let ix = (x * p.stride + kx) as isize - p.padding as isize;
                        if iy < 0 || iy >= p.height as isize || ix < 0 || ix >= p.width as isize {
                            continue;
                        }
                        let pos = (iy as usize * p.width + ix as usize) * p.input_channels;
                        if p.depthwise {
                            let wi = (ky * p.kernel + kx) * p.input_channels + c;
                            acc += (i32::from(input[pos + c]) + p.input_offset)
                                * i32::from(weights[wi]);
                        } else {
                            let wi = ((c * p.kernel + ky) * p.kernel + kx) * p.input_channels;
                            for i in 0..p.input_channels {
                                acc += (i32::from(input[pos + i]) + p.input_offset)
                                    * i32::from(weights[wi + i]);
                            }
                        }
                    }
                }
                let val = requantize(acc, mult[c], shift[c]) + p.output_offset;
                output[(y * p.output_width() + x) * p.output_channels + c] =
                    val.clamp(p.activation_min, p.activation_max) as i8;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rounding_and_saturation() {
        assert_eq!(requantize(i32::MIN, i32::MIN, 0), i32::MAX);
        for (x, expected) in [(-5, -2), (-3, -1), (-1, 0), (1, 1), (3, 2), (5, 3)] {
            assert_eq!(requantize(x, 1 << 30, 0), expected);
        }
        assert_eq!(requantize(-8, 1 << 30, -1), -2);
        assert_eq!(requantize(3, 1 << 30, 1), 3);
    }
    #[test]
    fn offset_padding_and_clamp() {
        let p = Params {
            height: 1,
            width: 1,
            input_channels: 1,
            output_channels: 1,
            kernel: 3,
            stride: 1,
            padding: 1,
            depthwise: true,
            input_offset: 3,
            output_offset: 5,
            activation_min: -10,
            activation_max: 10,
        };
        let mut o = [0];
        generic(&p, &[1], &[1; 9], &[0], &[1 << 30], &[0], &mut o);
        assert_eq!(o, [7]);
        generic(&p, &[127], &[127; 9], &[0], &[1 << 30], &[0], &mut o);
        assert_eq!(o, [10]);
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! { loop { core::hint::spin_loop(); } }
/// # Safety
/// All tensors have the lengths and non-aliasing guarantees in the workload.
#[no_mangle]
pub unsafe extern "C" fn kernel(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {
 let p=core::hint::black_box(Params { height: 1, width: 1, input_channels: 31, output_channels: 13, kernel: 1, stride: 1, padding: 0, input_offset: -3, output_offset: 5, activation_min: -111, activation_max: 103, depthwise: false });
 generic(&p, core::slice::from_raw_parts(i,p.input_len()),core::slice::from_raw_parts(w,p.weights_len()),core::slice::from_raw_parts(b,p.output_channels),core::slice::from_raw_parts(m,p.output_channels),core::slice::from_raw_parts(s,p.output_channels),core::slice::from_raw_parts_mut(o,p.output_len()));
}
