//! Image buffer abstractions
//!
//! # Image buffer
//!
//! `image_buffer` provides an `ImageBuffer` which helps to access the pixels of an image. It
//! provides iterators over the pixels of the image (via `ImageBuffer::pixels[_mut]` and
//! `ImageBuffer::enumerate_pixels[_mut]`). Furthermore it implements `Index` and `IndexMut`
//! which takes a tuple `(x, y)` with the coordinates of the pixel. `(0, 0)` is in the top left
//! corner.
//!
//! An image buffer can either be create by wrapping an existing buffer
//!
//! ```
//! # use image_buffer::{GrayImage, ImageBuffer};
//! let mut data = vec![0; 100 * 100];
//! let buffer: GrayImage = ImageBuffer::from_raw(100, 100, data).unwrap();
//! data = buffer.into_raw();
//! ```
//!
//! or by constructing a new one baked by a `std::Vec`
//!
//! ```
//! # use image_buffer::{RgbImage, ImageBuffer};
//! let _: RgbImage = ImageBuffer::new(100, 100);
//! ```
//! .
//!
//! # Color types
//!
//! This crate implements various color types, accessible via the `color` module. They implement
//! `From` wherever it makes sense to be able to convert between them. The `Rgb` to `Gray`
//! conversion assumes `Rgb` to be in the [sRGB](https://en.wikipedia.org/wiki/SRGB) color model.
//!
//! The `ImageBuffer` uses this to provide a convenience method for color conversions
//!
//! ```
//! # use image_buffer::{RgbImage, GrayImage, ImageBuffer};
//! let _: GrayImage = RgbImage::new(100, 100).convert_buffer();
//! ```

extern crate num_traits;

mod buffer;
mod color_model;
mod math;
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
	Color,
	ImageView,
	Primitive
};

pub mod color {
    pub use color_model::*;
}
