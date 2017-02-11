use super::{Rgb, Xyz};

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

impl From<Rgb<f32>> for Xyz<f32> {
    fn from(other: Rgb<f32>) -> Self {
        let r = other.0[0];
        let g = other.0[1];
        let b = other.0[2];
        Xyz([rgb_to_x(r, g, b), rgb_to_y(r, g, b), rgb_to_z(r, g, b)])
    }
}
