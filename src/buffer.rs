use std::slice::{Chunks, ChunksMut};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::marker::PhantomData;
use num_traits::Zero;

use color_model::{Rgb, Rgba, Gray, GrayA};
use traits::{Color as Pixel, ImageView};

/// Iterator over references to pixels.
pub struct Pixels<'a, P: Pixel + 'a>
    where P::Subpixel: 'a
{
    chunks: Chunks<'a, P::Subpixel>,
}

impl<'a, P: Pixel + 'a> Iterator for Pixels<'a, P>
    where P::Subpixel: 'a
{
    type Item = &'a P;

    #[inline(always)]
    fn next(&mut self) -> Option<&'a P> {
        self.chunks.next().map(|v| <P as Pixel>::from_slice(v))
    }
}

impl<'a, P: Pixel + 'a> DoubleEndedIterator for Pixels<'a, P>
    where P::Subpixel: 'a
{
    #[inline(always)]
    fn next_back(&mut self) -> Option<&'a P> {
        self.chunks.next_back().map(|v| <P as Pixel>::from_slice(v))
    }
}

/// Iterator over mutable references to pixels.
pub struct PixelsMut<'a, P: Pixel + 'a>
    where P::Subpixel: 'a
{
    chunks: ChunksMut<'a, P::Subpixel>,
}

impl<'a, P: Pixel + 'a> Iterator for PixelsMut<'a, P>
    where P::Subpixel: 'a
{
    type Item = &'a mut P;

    #[inline(always)]
    fn next(&mut self) -> Option<&'a mut P> {
        self.chunks.next().map(|v| <P as Pixel>::from_slice_mut(v))
    }
}

impl<'a, P: Pixel + 'a> DoubleEndedIterator for PixelsMut<'a, P>
    where P::Subpixel: 'a
{
    #[inline(always)]
    fn next_back(&mut self) -> Option<&'a mut P> {
        self.chunks.next_back().map(|v| <P as Pixel>::from_slice_mut(v))
    }
}

/// Enumerate the pixels of an image.
pub struct EnumeratePixels<'a, P: Pixel + 'a>
    where <P as Pixel>::Subpixel: 'a
{
    pixels: Pixels<'a, P>,
    x: u32,
    y: u32,
    width: u32,
}

impl<'a, P: Pixel + 'a> Iterator for EnumeratePixels<'a, P>
    where P::Subpixel: 'a
{
    type Item = (u32, u32, &'a P);

    #[inline(always)]
    fn next(&mut self) -> Option<(u32, u32, &'a P)> {
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }
        let (x, y) = (self.x, self.y);
        self.x += 1;
        match self.pixels.next() {
            None => None,
            Some(p) => Some((x, y, p)),
        }
    }
}

/// Enumerate the mutable pixels of an image.
pub struct EnumeratePixelsMut<'a, P: Pixel + 'a>
    where <P as Pixel>::Subpixel: 'a
{
    pixels: PixelsMut<'a, P>,
    x: u32,
    y: u32,
    width: u32,
}

impl<'a, P: Pixel + 'a> Iterator for EnumeratePixelsMut<'a, P>
    where P::Subpixel: 'a
{
    type Item = (u32, u32, &'a mut P);

    #[inline(always)]
    fn next(&mut self) -> Option<(u32, u32, &'a mut P)> {
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }
        let (x, y) = (self.x, self.y);
        self.x += 1;
        match self.pixels.next() {
            None => None,
            Some(p) => Some((x, y, p)),
        }
    }
}

/// Generic image buffer.
pub struct ImageBuffer<P: Pixel, Container: Deref<Target = [P::Subpixel]>> {
    width: u32,
    height: u32,
    data: Container,
    _pixel_type: PhantomData<P>,
}

// generic implementation, shared along all image buffers
impl<P, Container> ImageBuffer<P, Container>
    where P: Pixel,
          Container: Deref<Target = [P::Subpixel]>
{
    /// Contructs a buffer from a generic container
    /// (for example a `Vec` or a slice)
    ///
    /// Returns None if the container is not big enough
    pub fn from_raw(width: u32, height: u32, buf: Container) -> Option<ImageBuffer<P, Container>> {
        if width as usize * height as usize * <P as Pixel>::channel_count() as usize <= buf.len() {
            Some(ImageBuffer {
                data: buf,
                width: width,
                height: height,
                _pixel_type: PhantomData,
            })
        } else {
            None
        }
    }

    /// Returns the underlying raw buffer
    pub fn into_raw(self) -> Container {
        self.data
    }

    /// The width and height of this image.
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// The width of this image.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of this image.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns an iterator over the pixels of this image.
    ///
    /// # Examples
    ///
    /// ```
    /// # use image_buffer::{GrayImage, color};
    /// let buffer = GrayImage::new(100, 100);
    /// let mut sum = 0;
    /// for pixel in buffer.pixels() {
    ///     sum += pixel[0]
    /// }
    /// ```
    pub fn pixels<'a>(&'a self) -> Pixels<'a, P> {
        Pixels { chunks: self.data.chunks(<P as Pixel>::channel_count() as usize) }
    }

    /// Enumerates over the pixels of the image.
    ///
    /// The iterator yields the coordinates of each pixel
    /// along with a reference to them.
    ///
    /// # Examples
    ///
    /// ```
    /// # use image_buffer::{GrayImage, color};
    /// let buffer = GrayImage::new(100, 100);
    /// let mut column_sum = vec![0; 100];
    /// for (_, y, pixel) in buffer.enumerate_pixels() {
    ///     column_sum[y as usize] += pixel[0]
    /// }
    /// ```
    pub fn enumerate_pixels<'a>(&'a self) -> EnumeratePixels<'a, P> {
        EnumeratePixels {
            pixels: self.pixels(),
            x: 0,
            y: 0,
            width: self.width,
        }
    }

    /// Gets a reference to the pixel at location `(x, y)`
    ///
    /// # Panics
    ///
    /// Panics if `(x, y)` is out of the bounds `(width, height)`.
    fn get_pixel(&self, x: u32, y: u32) -> &P {
        let no_channels = <P as Pixel>::channel_count() as usize;
        let index = no_channels * (y * self.width + x) as usize;
        <P as Pixel>::from_slice(&self.data[index..index + no_channels])
    }
}

impl<P, Container> ImageBuffer<P, Container>
    where P: Pixel,
          Container: Deref<Target = [P::Subpixel]> + DerefMut
{
    /// Returns an iterator over the mutable pixels of this image.
    /// The iterator yields the coordinates of each pixel
    /// along with a mutable reference to them.
    ///
    /// # Examples
    ///
    /// ```
    /// # use image_buffer::{ImageBuffer, color};
    /// let mut buffer = ImageBuffer::new(100, 100);
    /// for (i, pixel) in buffer.pixels_mut().enumerate() {
    ///     *pixel = color::Gray::new([i as u16]);
    /// }
    /// ```
    pub fn pixels_mut(&mut self) -> PixelsMut<P> {
        PixelsMut { chunks: self.data.chunks_mut(<P as Pixel>::channel_count() as usize) }
    }

    /// Enumerates over the mutable pixels of the image.
    ///
    /// # Examples
    ///
    /// ```
    /// # use image_buffer::{ImageBuffer, color};
    /// let mut buffer = ImageBuffer::new(100, 100);
    /// for (x, y, pixel) in buffer.enumerate_pixels_mut() {
    ///     *pixel = color::Gray::new([x * y]);
    /// }
    /// ```
    pub fn enumerate_pixels_mut<'a>(&'a mut self) -> EnumeratePixelsMut<'a, P> {
        let width = self.width;
        EnumeratePixelsMut {
            pixels: self.pixels_mut(),
            x: 0,
            y: 0,
            width: width,
        }
    }

    /// Gets a reference to the mutable pixel at location `(x, y)`
    ///
    /// # Panics
    ///
    /// Panics if `(x, y)` is out of the bounds `(width, height)`.
    fn get_pixel_mut(&mut self, x: u32, y: u32) -> &mut P {
        let no_channels = <P as Pixel>::channel_count() as usize;
        let index = no_channels * (y * self.width + x) as usize;
        <P as Pixel>::from_slice_mut(&mut self.data[index..index + no_channels])
    }
}

impl<P, Container> Deref for ImageBuffer<P, Container>
    where P: Pixel,
          Container: Deref<Target = [P::Subpixel]>
{
    type Target = [P::Subpixel];

    fn deref<'a>(&'a self) -> &'a <Self as Deref>::Target {
        &*self.data
    }
}

impl<P, Container> DerefMut for ImageBuffer<P, Container>
    where P: Pixel,
          Container: Deref<Target = [P::Subpixel]> + DerefMut
{
    fn deref_mut<'a>(&'a mut self) -> &'a mut <Self as Deref>::Target {
        &mut *self.data
    }
}

impl<P, Container> Index<(u32, u32)> for ImageBuffer<P, Container>
    where P: Pixel,
          Container: Deref<Target = [P::Subpixel]>
{
    type Output = P;

    fn index(&self, (x, y): (u32, u32)) -> &P {
        self.get_pixel(x, y)
    }
}

impl<P, Container> IndexMut<(u32, u32)> for ImageBuffer<P, Container>
    where P: Pixel,
          Container: Deref<Target = [P::Subpixel]> + DerefMut
{
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut P {
        self.get_pixel_mut(x, y)
    }
}

impl<P, Container> ImageView<P> for ImageBuffer<P, Container>
    where P: Pixel,
          Container: Deref<Target = [P::Subpixel]> + DerefMut
{
}

impl<P, Container> Clone for ImageBuffer<P, Container>
    where P: Pixel,
          Container: Deref<Target = [P::Subpixel]> + Clone
{
    fn clone(&self) -> ImageBuffer<P, Container> {
        ImageBuffer {
            data: self.data.clone(),
            width: self.width,
            height: self.height,
            _pixel_type: PhantomData,
        }
    }
}

/// Specialized implementation for `Vec`-backed buffers.
impl<P: Pixel> ImageBuffer<P, Vec<P::Subpixel>> {
    /// Creates a new image buffer based on a `Vec<P::Subpixel>`.
    pub fn new(width: u32, height: u32) -> ImageBuffer<P, Vec<P::Subpixel>> {
        ImageBuffer {
            data: vec![Zero::zero();
                      (width as u64
                      * height as u64
                      * (<P as Pixel>::channel_count() as u64)
                      ) as usize],
            width: width,
            height: height,
            _pixel_type: PhantomData,
        }
    }

    /// Constructs a new ImageBuffer by copying a pixel
    pub fn from_pixel(width: u32, height: u32, pixel: P) -> ImageBuffer<P, Vec<P::Subpixel>> {
        let mut buf = ImageBuffer::new(width, height);
        for p in buf.pixels_mut() {
            *p = pixel
        }
        buf
    }

    /// Constructs a new ImageBuffer by repeated application of the supplied function.
    /// The arguments to the function are the pixel's x and y coordinates.
    pub fn from_fn<F>(width: u32, height: u32, f: F) -> ImageBuffer<P, Vec<P::Subpixel>>
        where F: Fn(u32, u32) -> P
    {
        let mut buf = ImageBuffer::new(width, height);
        for (x, y, p) in buf.enumerate_pixels_mut() {
            *p = f(x, y)
        }
        buf
    }
}

impl<'a, 'b, Container, FromColor: Pixel> ImageBuffer<FromColor, Container>
    where Container: Deref<Target = [FromColor::Subpixel]>
{
    /// Performs a color conversion of the image buffer.
    ///
    /// Converts the color `FromColor` to the color `ToColor`. Allocates a new image buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use image_buffer::{RgbImage, GrayImage, color};
    /// let rgb = RgbImage::new(100, 100);
    /// let grayscale = rgb.convert_buffer::<color::Gray<u8>>();
    /// ```

    pub fn convert_buffer<ToColor>(&self) -> ImageBuffer<ToColor, Vec<ToColor::Subpixel>>
        where ToColor: Pixel + From<FromColor>
    {
        let mut buffer = ImageBuffer::new(self.width, self.height);
        for (mut to, from) in buffer.pixels_mut().zip(self.pixels()) {
            *to = From::from(*from)
        }
        buffer
    }
}

/// Sendable Rgb image buffer
pub type RgbImage = ImageBuffer<Rgb<u8>, Vec<u8>>;
/// Sendable Rgb + alpha channel image buffer
pub type RgbaImage = ImageBuffer<Rgba<u8>, Vec<u8>>;
/// Sendable grayscale image buffer
pub type GrayImage = ImageBuffer<Gray<u8>, Vec<u8>>;
/// Sendable grayscale + alpha channel image buffer
pub type GrayAlphaImage = ImageBuffer<GrayA<u8>, Vec<u8>>;

#[cfg(test)]
mod test {

    use super::{ImageBuffer, RgbImage, GrayImage};
    use Color;
    use color_model;

    #[test]
    /// Tests if image buffers from slices work
    fn slice_buffer() {
        let data = [0; 9];
        let buf: ImageBuffer<color_model::Gray<u8>, _> = ImageBuffer::from_raw(3, 3, &data[..])
            .unwrap();
        assert_eq!(&*buf, &data[..])
    }

    #[test]
    fn test_get_pixel() {
        let mut a: RgbImage = ImageBuffer::new(10, 10);
        {
            let b = a.get_mut(3 * 10).unwrap();
            *b = 255;
        }
        assert_eq!(a.get_pixel(0, 1)[0], 255)

    }

    #[test]
    fn test_mut_iter() {
        let mut a: RgbImage = ImageBuffer::new(10, 10);
        {
            let val = a.pixels_mut().next().unwrap();
            *val = color_model::Rgb::new([42, 0, 0]);
        }
        assert_eq!(a.data[0], 42)
    }

    #[test]
    fn test_conversion() {
        let mut a: RgbImage = ImageBuffer::new(100, 100);
        for mut p in a.pixels_mut() {
            let rgb = p.channels_mut();
            rgb[0] = 255;
            rgb[1] = 23;
            rgb[2] = 42;
        }
        let b: GrayImage = a.convert_buffer();
        assert_eq!(b.data[0], 129)
    }
}
