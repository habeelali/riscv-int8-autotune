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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_at_ties_and_over_large_proven_domain() {
        let mut count = 0;
        for x in [
            -(1 << 30),
            -(1 << 30) + 1,
            -100001,
            -33,
            -17,
            -9,
            -5,
            -3,
            -2,
            -1,
            0,
            1,
            2,
            3,
            5,
            9,
            17,
            33,
            100001,
            (1 << 30) - 1,
            1 << 30,
        ] {
            for m in [0, 1, 1234567, 1 << 29, 1 << 30, i32::MAX] {
                for shift in -30..=-1 {
                    assert_eq!(
                        requantize(x, m, shift),
                        kernel_core::requantize(x, m, shift)
                    );
                    count += 1;
                }
            }
        }
        let mut state = 20261018u32;
        for _ in 0..100_000 {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            let x = (state as i32) / 2;
            let m = (state.rotate_left(13) & 0x7fff_ffff) as i32;
            let shift = -1 - (state % 30) as i32;
            assert_eq!(
                requantize(x, m, shift),
                kernel_core::requantize(x, m, shift)
            );
            count += 1;
        }
        assert_eq!(count, 103780);
    }
}
