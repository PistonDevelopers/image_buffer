extern crate num_traits;

mod buffer;
mod color_model;
mod traits;

#[cfg_attr(rustfmt, rustfmt_skip)]
pub use buffer::{
	ImageBuffer,
	Pixels,
	PixelsMut,
	EnumeratePixels,
	EnumeratePixelsMut,
	RgbImage,
	RgbaImage,
	GrayImage,
	GrayAlphaImage,
};
#[cfg_attr(rustfmt, rustfmt_skip)]
pub use traits::{
	Pixel,
	Primitive
};

pub mod color {
    pub use color_model::*;
}
