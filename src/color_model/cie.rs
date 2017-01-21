use super::{Rgb, Rgba, Xyz, XyzA};

/// Converts sRGB to the X component of CIE 1931.
pub fn rgb_to_x(r: f32, g: f32, b: f32) -> f32 {
    0.4124 * r + 0.3576 * g + 0.1805 * b
}

/// Converts sRGB to the Y component of CIE 1931.
pub fn rgb_to_y(r: f32, g: f32, b: f32) -> f32 {
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// Converts sRGB to the Z component of CIE 1931.
pub fn rgb_to_z(r: f32, g: f32, b: f32) -> f32 {
    0.0193 * r + 0.1192 * g + 0.9505 * b
}

// From for Xyz

impl From<XyzA<f32>> for Xyz<f32> {
    fn from(other: XyzA<f32>) -> Self {
        let xyza = other.0;
        Xyz([xyza[0], xyza[1], xyza[2]])
    }
}

impl From<Rgb<f32>> for Xyz<f32> {
    fn from(other: Rgb<f32>) -> Self {
        let r = other.0[0];
        let g = other.0[1];
        let b = other.0[2];
        Xyz([rgb_to_x(r, g, b), rgb_to_y(r, g, b), rgb_to_z(r, g, b)])
    }
}

// From for XyzA

impl From<Xyz<f32>> for XyzA<f32> {
    fn from(other: Xyz<f32>) -> Self {
        let xyz = other.0;
        XyzA([xyz[0], xyz[1], xyz[2], 1.0])
    }
}

impl From<Rgba<f32>> for XyzA<f32> {
    fn from(other: Rgba<f32>) -> Self {
        let r = other.0[0];
        let g = other.0[1];
        let b = other.0[2];
        let a = other.0[3];
        XyzA([rgb_to_x(r, g, b), rgb_to_y(r, g, b), rgb_to_z(r, g, b), a])
    }
}
