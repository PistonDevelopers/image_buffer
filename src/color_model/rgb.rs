//! sRGB colors

use num_traits::{Float, NumCast};

use traits::{Primitive, ChannelMax};
use super::{Rgb, Rgba, Gray, GrayA, Xyz, XyzA};
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

impl<T: Primitive + ChannelMax> From<Rgb<T>> for Rgba<T> {
    fn from(other: Rgb<T>) -> Self {
        let rgb = other.0;
        Rgba([rgb[0], rgb[1], rgb[2], T::channel_max()])
    }
}

impl<T: Primitive> From<GrayA<T>> for Rgba<T> {
    fn from(other: GrayA<T>) -> Self {
        let luma_a = other.0;
        Rgba([luma_a[0], luma_a[0], luma_a[0], luma_a[1]])
    }
}

impl<T: Primitive + ChannelMax> From<Gray<T>> for Rgba<T> {
    fn from(other: Gray<T>) -> Self {
        let luma = other.0[0];
        Rgba([luma, luma, luma, T::channel_max()])
    }
}

impl From<XyzA<f32>> for Rgba<f32> {
    fn from(other: XyzA<f32>) -> Self {
        let x = other.0[0];
        let y = other.0[1];
        let z = other.0[2];
        let a = other.0[3];
        Rgba([xyz_to_r(x, y, z), xyz_to_g(x, y, z), xyz_to_b(x, y, z), a])
    }
}

impl From<XyzA<f32>> for Rgba<u8> {
    fn from(other: XyzA<f32>) -> Self {
        let rgba: Rgba<f32> = other.into();
        rgba.into()
    }
}

impl From<XyzA<f32>> for Rgba<u16> {
    fn from(other: XyzA<f32>) -> Self {
        let rgb: Rgba<f32> = other.into();
        rgb.into()
    }
}

// Gamma expansion and compression

impl From<Rgba<u8>> for Rgba<f32> {
    fn from(other: Rgba<u8>) -> Self {
        let a = other.0[3];
        let rgb: Rgb<u8> = other.into();
        let rgb: Rgb<f32> = rgb.into();
        let mut rgba: Rgba<f32> = rgb.into();
        rgba.0[3] = rescale(a);
        rgba
    }
}

impl From<Rgba<u16>> for Rgba<f32> {
    fn from(other: Rgba<u16>) -> Self {
        let a = other.0[3];
        let rgb: Rgb<u16> = other.into();
        let rgb: Rgb<f32> = rgb.into();
        let mut rgba: Rgba<f32> = rgb.into();
        rgba.0[3] = rescale(a);
        rgba
    }
}

impl From<Rgba<f32>> for Rgba<u8> {
    fn from(other: Rgba<f32>) -> Self {
        let a = other.0[3];
        let rgb: Rgb<f32> = other.into();
        let rgb: Rgb<u8> = rgb.into();
        let mut rgba: Rgba<u8> = rgb.into();
        rgba.0[3] = rescale(a);
        rgba
    }
}

impl From<Rgba<f32>> for Rgba<u16> {
    fn from(other: Rgba<f32>) -> Self {
        let a = other.0[3];
        let rgb: Rgb<f32> = other.into();
        let rgb: Rgb<u16> = rgb.into();
        let mut rgba: Rgba<u16> = rgb.into();
        rgba.0[3] = rescale(a);
        rgba
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
        assert_eq!(val.0[2], 1.0f32);
        //let val: Rgb<u16> = val.into();
        //assert_eq!(val.0[2], 0xFFFFu16);
        let val: Rgba<u8> = VAL_RGB_U8.into();
        assert_eq!(val.0[3], 255);
        let val: Rgb<u8> = val.into();
        assert_eq!(val.0[1], 251);
    }

}
