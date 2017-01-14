use num_traits::{NumCast, Unsigned};

use traits::Primitive;
use super::{Rgb, Rgba, Gray, GrayA};


/// Clamp the value to a given range.
pub fn clamp<N: PartialOrd>(a: N, min: N, max: N) -> N {
    if a < min {
        return min;
    }
    if a > max {
        return max;
    }
    a
}

/// Performs the sRGB gamma correction.
///
/// Assumes `c` to be an discretized integer value.
fn srgb_to_linear<T: Primitive + Unsigned>(c: T) -> f32 {
    // Scale to 1.0
    let c = c.to_f32().unwrap() * (1.0 / T::max_value().to_f32().unwrap());
    // sRGB gamma correction
    if c < 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Performs the inverse sRGB gamma correction.
///
/// Assumes `c` to be scaled to 1.0.
fn linear_to_srgb<T: Primitive + Unsigned>(c: f32) -> T {
    // Inverse gamma correction
    let c = if c < 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    };
    let max = T::max_value().to_f32().unwrap();
    NumCast::from(clamp(c * max, 0.0, max).round()).unwrap()
}

/// sRGB to Y conversion for integer values
fn rgb_to_luminance<T: Primitive + Unsigned, V: Primitive + Unsigned>(r: T, g: T, b: T) -> V {
    let r = srgb_to_linear(r);
    let g = srgb_to_linear(g);
    let b = srgb_to_linear(b);
    let l = 0.2126f32 * r + 0.7152f32 * g + 0.0722f32 * b;
    linear_to_srgb(l)
}

// From for Gray

impl<T: Primitive + Unsigned> From<Rgba<T>> for Gray<T> {
    fn from(other: Rgba<T>) -> Self {
        let rgba = other.0;
        Gray([rgb_to_luminance(rgba[0], rgba[1], rgba[2])])
    }
}

impl<T: Primitive + Unsigned> From<Rgb<T>> for Gray<T> {
    fn from(other: Rgb<T>) -> Self {
        let rgb = other.0;
        Gray([rgb_to_luminance(rgb[0], rgb[1], rgb[2])])
    }
}

impl<T: Primitive> From<GrayA<T>> for Gray<T> {
    fn from(other: GrayA<T>) -> Self {
        Gray([other.0[0]])
    }
}

// From for LumA

impl<T: Primitive + Unsigned> From<Rgba<T>> for GrayA<T> {
    fn from(other: Rgba<T>) -> Self {
        let rgba = other.0;
        GrayA([rgb_to_luminance(rgba[0], rgba[1], rgba[2]), rgba[3]])
    }
}

impl<T: Primitive + Unsigned> From<Rgb<T>> for GrayA<T> {
    fn from(other: Rgb<T>) -> Self {
        let rgb = other.0;
        GrayA([rgb_to_luminance(rgb[0], rgb[1], rgb[2]), T::max_value()])
    }
}

impl<T: Primitive> From<Gray<T>> for GrayA<T> {
    fn from(other: Gray<T>) -> Self {
        GrayA([other.0[0], T::max_value()])
    }
}
