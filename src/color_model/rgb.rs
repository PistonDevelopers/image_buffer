//! sRGB colors

use num_traits::NumCast;

use traits::{Primitive, ChannelMax};
use super::{Rgb, Gray, Xyz};
use math::clamp;

/// Converts CIE 1931 XYZ to the R component of sRGB.
pub fn xyz_to_r(x: f32, y: f32, z: f32) -> f32 {
    3.2406 * x - 1.5372 * y - 0.4986 * z
}

/// Converts CIE 1931 XYZ to the G component of sRGB.
pub fn xyz_to_g(x: f32, y: f32, z: f32) -> f32 {
    -0.9689 * x + 1.8758 * y + 0.0415 * z
}

/// Converts CIE 1931 XYZ to the B component of sRGB.
pub fn xyz_to_b(x: f32, y: f32, z: f32) -> f32 {
    0.0557 * x - 0.2040 * y + 1.0570 * z
}

#[inline]
fn rescale<T: Primitive + ChannelMax, V: Primitive + ChannelMax>(a: T) -> V {
    let max_t_in_v: V = NumCast::from(T::channel_max()).unwrap();
    let max_v = V::channel_max();
    if max_v > max_t_in_v {
        // V is an integer
        // TODO: Use round here, maybe use a trait `Rescale`?
        let max = NumCast::from(max_v).unwrap();
        NumCast::from(clamp(a * max, T::zero(), max)).unwrap()
    } else {
        // V is a float
        <V as NumCast>::from(a).unwrap() * (V::one() / max_t_in_v)
    }
}

/// Gamma expansion as defined for sRGB.
///
/// Assumes `c` to be an discretized integer value.
pub fn srgb_expand_gamma<T: Primitive + ChannelMax>(c: T) -> f32 {
    // Scale to 1.0
    let c: f32 = rescale(c);
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
pub fn srgb_compress_gamma<T: Primitive + ChannelMax>(c: f32) -> T {
    // Inverse gamma correction
    rescale(if c < 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    })
}

// From for RGB

impl<T: Primitive> From<Gray<T>> for Rgb<T> {
    fn from(other: Gray<T>) -> Self {
        let luma = other.0[0];
        Rgb([luma, luma, luma])
    }
}

impl From<Xyz<f32>> for Rgb<f32> {
    fn from(other: Xyz<f32>) -> Self {
        let x = other.0[0];
        let y = other.0[1];
        let z = other.0[2];
        Rgb([xyz_to_r(x, y, z), xyz_to_g(x, y, z), xyz_to_b(x, y, z)])
    }
}

impl From<Xyz<f32>> for Rgb<u8> {
    fn from(other: Xyz<f32>) -> Self {
        let rgb: Rgb<f32> = other.into();
        rgb.into()
    }
}

impl From<Xyz<f32>> for Rgb<u16> {
    fn from(other: Xyz<f32>) -> Self {
        let rgb: Rgb<f32> = other.into();
        rgb.into()
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

// Gamma expansion and compression

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

#[cfg(test)]
mod tests {
    use super::super::*;

    static VAL_RGB_U8: Rgb<u8> = Rgb([241, 251, 0xFF]);

    #[test]
    fn test_rescale() {
        assert_eq!(1.0f32, super::rescale(255u8));
        assert_eq!(255u8, super::rescale(1.0f32));
        assert_eq!(0xFFFFu16, super::rescale(1.0f32));
    }

    #[test]
    fn test_rgb_conversions() {
        let val: Rgb<f32> = VAL_RGB_U8.into();
        assert_eq!(val.as_ref()[2], 1.0f32);
        // let val: Rgb<u16> = val.into();
        // assert_eq!(val.0[2], 0xFFFFu16);
        let val: Rgba<u8> = VAL_RGB_U8.into();
        assert_eq!(val.as_ref()[3], 255);
        let val: Rgb<u8> = val.into();
        assert_eq!(val.as_ref()[1], 251);
    }

}
