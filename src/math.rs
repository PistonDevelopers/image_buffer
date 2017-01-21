//! Utility functions.

/// Clamp the value to a given range.
#[inline]
pub fn clamp<N: PartialOrd>(a: N, min: N, max: N) -> N {
    if a < min {
        return min;
    }
    if a > max {
        return max;
    }
    a
}
