//! sRGB colors

use num_traits::{NumCast, Unsigned};

use traits::Primitive;
use super::{Rgb, Rgba, Gray, GrayA};
use math::clamp;

/// Converts sRGB to the X component of CIE 1931.
pub fn xyz_to_r(x: f32, y: f32, z: f32) -> f32 {
    3.2406 * x - 1.5372 * y - 0.4986 * z
}

/// Converts sRGB to the Y component of CIE 1931.
pub fn xyz_to_g(x: f32, y: f32, z: f32) -> f32 {
    -0.9689 * x + 1.8758 * y + 0.0415 * z
}

/// Converts sRGB to the Z component of CIE 1931.
pub fn xyz_to_b(x: f32, y: f32, z: f32) -> f32 {
    0.0557 * x - 0.2040 * y + 1.0570 * z
}

/// Gamma expansion as defined for sRGB.
///
/// Assumes `c` to be an discretized integer value.
pub fn srgb_expand_gamma<T: Primitive + Unsigned>(c: T) -> f32 {
    // Scale to 1.0
    let c = c.to_f32().unwrap() * (1.0 / T::max_value().to_f32().unwrap());
    // sRGB gamma correction
    if c < 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Gamma compression as defined for sRGB.
///
/// Assumes `c` to be scaled to 1.0.
pub fn srgb_compress_gamma<T: Primitive + Unsigned>(c: f32) -> T {
    // Inverse gamma correction
    let c = if c < 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    };
    let max = T::max_value().to_f32().unwrap();
    NumCast::from(clamp(c * max, 0.0, max).round()).unwrap()
}

impl<T: Primitive> From<Rgb<T>> for Rgba<T> {
    fn from(other: Rgb<T>) -> Self {
        let rgb = other.0;
        Rgba([rgb[0], rgb[1], rgb[2], T::max_value()])
    }
}

impl<T: Primitive> From<GrayA<T>> for Rgba<T> {
    fn from(other: GrayA<T>) -> Self {
        let luma_a = other.0;
        Rgba([luma_a[0], luma_a[0], luma_a[0], luma_a[1]])
    }
}

impl<T: Primitive> From<Gray<T>> for Rgba<T> {
    fn from(other: Gray<T>) -> Self {
        let luma = other.0[0];
        Rgba([luma, luma, luma, T::max_value()])
    }
}


// From for RGB

impl<T: Primitive> From<Rgba<T>> for Rgb<T> {
    fn from(other: Rgba<T>) -> Self {
        let rgb_a = other.0;
        Rgb([rgb_a[0], rgb_a[1], rgb_a[2]])

    }
}

impl<T: Primitive> From<GrayA<T>> for Rgb<T> {
    fn from(other: GrayA<T>) -> Self {
        let luma = other.0[0];
        Rgb([luma, luma, luma])
    }
}

impl<T: Primitive> From<Gray<T>> for Rgb<T> {
    fn from(other: Gray<T>) -> Self {
        let luma = other.0[0];
        Rgb([luma, luma, luma])
    }
}

// Gamma expansion and compression

impl From<Rgb<u8>> for Rgb<f32> {
    fn from(other: Rgb<u8>) -> Self {
        let rgb = other.0;
        Rgb([srgb_expand_gamma(rgb[0]), srgb_expand_gamma(rgb[1]), srgb_expand_gamma(rgb[2])])
    }
}

impl From<Rgb<u16>> for Rgb<f32> {
    fn from(other: Rgb<u16>) -> Self {
        let rgb = other.0;
        Rgb([srgb_expand_gamma(rgb[0]), srgb_expand_gamma(rgb[1]), srgb_expand_gamma(rgb[2])])
    }
}

impl From<Rgb<f32>> for Rgb<u8> {
    fn from(other: Rgb<f32>) -> Self {
        let rgb = other.0;
        Rgb([srgb_compress_gamma(rgb[0]), srgb_compress_gamma(rgb[1]), srgb_compress_gamma(rgb[2])])
    }
}

impl From<Rgb<f32>> for Rgb<u16> {
    fn from(other: Rgb<f32>) -> Self {
        let rgb = other.0;
        Rgb([srgb_compress_gamma(rgb[0]), srgb_compress_gamma(rgb[1]), srgb_compress_gamma(rgb[2])])
    }
}
