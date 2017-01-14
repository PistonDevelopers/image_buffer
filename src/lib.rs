#![feature(optin_builtin_traits)]

extern crate num_traits;

mod buffer;
mod color_model;
mod traits;

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
pub use traits::{
	Pixel,
	Primitive
};

pub mod color {
	pub use color_model::*;
}
