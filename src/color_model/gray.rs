use traits::{Primitive, ChannelMax};
use super::{Rgb, Rgba, Gray, GrayA, cie, rgb};

/// sRGB to Y conversion for integer values
fn srgb_to_luminance<T: Primitive + ChannelMax, V: Primitive + ChannelMax>(rgb: Rgb<T>) -> V
    where super::Rgb<f32>: From<super::Rgb<T>>
{
    let Rgb::<f32>(rgb) = rgb.into();
    rgb::srgb_compress_gamma(cie::rgb_to_y(rgb[0], rgb[1], rgb[2]))
}

// From for Gray

impl<T: Primitive + ChannelMax> From<Rgba<T>> for Gray<T>
    where super::Rgb<f32>: From<super::Rgb<T>>
{
    fn from(other: Rgba<T>) -> Self {
        Gray([srgb_to_luminance(other.into())])
    }
}

impl<T: Primitive + ChannelMax> From<Rgb<T>> for Gray<T>
    where super::Rgb<f32>: From<super::Rgb<T>>
{
    fn from(other: Rgb<T>) -> Self {
        Gray([srgb_to_luminance(other)])
    }
}

impl<T: Primitive> From<GrayA<T>> for Gray<T> {
    fn from(other: GrayA<T>) -> Self {
        Gray([other.0[0]])
    }
}

// From for LumA

impl<T: Primitive + ChannelMax> From<Rgba<T>> for GrayA<T>
    where super::Rgb<f32>: From<super::Rgb<T>>
{
    fn from(other: Rgba<T>) -> Self {
        GrayA([srgb_to_luminance(other.into()), T::channel_max()])
    }
}

impl<T: Primitive + ChannelMax> From<Rgb<T>> for GrayA<T>
    where super::Rgb<f32>: From<super::Rgb<T>>
{
    fn from(other: Rgb<T>) -> Self {
        GrayA([srgb_to_luminance(other), T::channel_max()])
    }
}

impl<T: Primitive + ChannelMax> From<Gray<T>> for GrayA<T> {
    fn from(other: Gray<T>) -> Self {
        GrayA([other.0[0], T::channel_max()])
    }
}
