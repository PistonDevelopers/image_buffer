# image_buffer [![Build Status](https://travis-ci.org/PistonDevelopers/image_buffer.svg?branch=master)](https://travis-ci.org/PistonDevelopers/image_buffer)

## Image buffer abstraction

Provides an image buffer and various color types ([API Documentation](https://docs.rs/image_buffer)).

### Usage

```rust
//!An example of generating Julia fractals.
extern crate num;
extern crate image_buffer;

use num::complex::Complex;

fn main() {
    let max_iterations = 256u16;

    let imgx = 800;
    let imgy = 800;

    let scalex = 4.0 / imgx as f32;
    let scaley = 4.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image_buffer::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordiantes and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let cy = y as f32 * scaley - 2.0;
        let cx = x as f32 * scalex - 2.0;

        let mut z = Complex::new(cx, cy);
        let c = Complex::new(-0.4, 0.6);

        let mut i = 0;

        for t in (0..max_iterations) {
            if z.norm() > 2.0 {
                break
            }
            z = z * z + c;
            i = t;
        }

        // Create an 8bit pixel of type Luma and value i
        // and assign in to the pixel at position (x, y)
        *pixel = image::Gray([i as u8]);

    }
}
```
