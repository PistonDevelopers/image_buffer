use traits::{Primitive, ChannelMax};
use super::{Rgb, Gray, cie, rgb};

/// sRGB to Y conversion for integer values
fn srgb_to_luminance<T: Primitive + ChannelMax, V: Primitive + ChannelMax>(rgb: Rgb<T>) -> V
    where super::Rgb<f32>: From<super::Rgb<T>>
{
    let Rgb::<f32>(rgb) = rgb.into();
    rgb::srgb_compress_gamma(cie::rgb_to_y(rgb[0], rgb[1], rgb[2]))
}

// From for Gray

impl<T: Primitive + ChannelMax> From<Rgb<T>> for Gray<T>
    where super::Rgb<f32>: From<super::Rgb<T>>
{
    fn from(other: Rgb<T>) -> Self {
        Gray([srgb_to_luminance(other)])
    }
}
