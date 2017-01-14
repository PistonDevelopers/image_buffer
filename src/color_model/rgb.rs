use traits::Primitive;
use super::{Rgb, Rgba, Gray, GrayA};

// From for RGBA

impl<T: Primitive> From<Rgb<T>> for Rgba<T> {
    fn from(other: Rgb<T>) -> Self {
        let rgb = other.0;
        Rgba([
            rgb[0],
            rgb[1],
            rgb[2],
            T::max_value(),
        ])
    }
}
impl<T: Primitive> From<GrayA<T>> for Rgba<T> {
    fn from(other: GrayA<T>) -> Self {
        let luma_a = other.0;
        Rgba([
            luma_a[0],
            luma_a[0],
            luma_a[0],
            luma_a[1],
        ])
    }
}

impl<T: Primitive> From<Gray<T>> for Rgba<T> {
    fn from(other: Gray<T>) -> Self {
        let luma = other.0[0];
        Rgba([
            luma,
            luma,
            luma,
            T::max_value(),
        ])
    }
}


// From for RGB

impl<T: Primitive> From<Rgba<T>> for Rgb<T> {
    fn from(other: Rgba<T>) -> Self {
        let rgb_a = other.0;
        Rgb([
            rgb_a[0],
            rgb_a[1],
            rgb_a[2]
        ])

    }
}

impl<T: Primitive> From<GrayA<T>> for Rgb<T> {
    fn from(other: GrayA<T>) -> Self {
        let luma = other.0[0];
        Rgb([
            luma,
            luma,
            luma,
        ])
    }
}

impl<T: Primitive> From<Gray<T>> for Rgb<T> {
    fn from(other: Gray<T>) -> Self {
        let luma = other.0[0];
        Rgb([
            luma,
            luma,
            luma,
        ])
    }
}
