mod cie;
mod gray;
mod rgb;
mod alpha;

use std::ops::{Index, IndexMut};
use std::mem;

use traits::Color;
use traits::{Primitive, ColorMathOps};

pub use self::alpha::Alpha;

macro_rules! define_color_model {
    {$(
        $ident: ident,
        $channels: expr,
        $alphas: expr,
        $interpretation: expr,
        #[$doc:meta];
    )*} => {
$( // START Structure definitions

#[$doc]
#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct $ident<T: Primitive>([T; $channels]);

impl<T: Primitive> $ident<T> {
    pub fn new(array: [T; $channels]) -> Self {
        $ident(array)
    }
}

impl<T: Primitive> AsRef<[T; $channels]> for $ident<T> {
    fn as_ref(&self) -> &<Self as Color>::Storage {
        &self.0
    }
}

impl<T: Primitive> AsMut<[T; $channels]> for $ident<T> {
    fn as_mut(&mut self) -> &mut <Self as Color>::Storage {
        &mut self.0
    }
}

impl<T: Primitive> Color for $ident<T> {

    type Subpixel = T;
    type Storage = [T; $channels];

    /// Returns the number of channels of this pixel type.
    fn channel_count() -> usize {
        $channels
    }

    #[inline(always)]
    fn channels(&self) -> &[T; $channels] {
        &self.0
    }

    #[inline(always)]
    fn channels_mut(&mut self) -> &mut [T; $channels] {
        &mut self.0
    }

    fn from_channels(other: [T; $channels]) -> $ident<T> {
        *<$ident<T> as Color>::from_slice(&other[..$channels])
    }

    fn from_slice<'a>(slice: &'a [T]) -> &'a $ident<T> {
        unsafe {
            assert_eq!(slice.len(), $channels);
            mem::transmute(slice.as_ptr())
        }
    }

    fn from_slice_mut<'a>(slice: &'a mut [T]) -> &'a mut $ident<T> {
        unsafe {
            assert_eq!(slice.len(), $channels);
            mem::transmute(slice.as_ptr())
        }
    }

    fn apply_with_alpha<F, G>(&mut self, f: F, g: G) where F: Fn(T) -> T, G: Fn(T) -> T {
        for v in self.0[..$channels as usize-$alphas as usize].iter_mut() {
            *v = f(*v)
        }
        if $alphas as usize != 0 {
            let v = &mut self.0[$channels as usize-$alphas as usize-1];
            *v = g(*v)
        }
    }

    fn color_model() -> &'static str {
        $interpretation
    }
}

impl<T: Primitive> Index<usize> for $ident<T> {
    type Output = T;
    #[inline(always)]
    fn index<'a>(&'a self, _index: usize) -> &'a T {
        &self.0[_index]
    }
}

impl<T: Primitive> IndexMut<usize> for $ident<T> {
    #[inline(always)]
    fn index_mut<'a>(&'a mut self, _index: usize) -> &'a mut T {
        &mut self.0[_index]
    }
}

impl<T: Primitive> ColorMathOps<$ident<T>> for $ident<T> {
    #[inline(always)]
    fn add(mut self, rhs: Self) -> Self {
        for i in 0..$channels {
            self.0[i] = self.0[i] + rhs.0[i]
        }
        self
    }

    #[inline(always)]
    fn sub(mut self, rhs: Self) -> Self {
        for i in 0..$channels {
            self.0[i] = self.0[i] - rhs.0[i]
        }
        self
    }

    #[inline(always)]
    fn div(mut self, rhs: Self) -> Self {
        for i in 0..$channels {
            self.0[i] = self.0[i] / rhs.0[i]
        }
        self
    }

    #[inline(always)]
    fn mul(mut self, rhs: Self) -> Self {
        for i in 0..$channels {
            self.0[i] = self.0[i] * rhs.0[i]
        }
        self
    }
}

impl<T: Primitive, V: ColorMathOps<$ident<T>>> ::std::ops::Add<V> for $ident<T> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: V) -> Self::Output {
        rhs.add(self)
    }
}

impl<T: Primitive> ::std::ops::AddAssign for $ident<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T: Primitive, V: ColorMathOps<$ident<T>>> ::std::ops::Sub<V> for $ident<T> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: V) -> Self::Output {
        rhs.sub(self)
    }
}

impl<T: Primitive> ::std::ops::SubAssign for $ident<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<T: Primitive, V: ColorMathOps<$ident<T>>> ::std::ops::Div<V> for $ident<T> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: V) -> Self::Output {
        rhs.div(self)
    }
}

impl<T: Primitive> ::std::ops::DivAssign for $ident<T> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<T: Primitive, V: ColorMathOps<$ident<T>>> ::std::ops::Mul<V> for $ident<T> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: V) -> Self::Output {
        rhs.mul(self)
    }
}

impl<T: Primitive> ::std::ops::MulAssign for $ident<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T: Primitive> From<Alpha<$ident<T>>> for $ident<T> {
    fn from(other: Alpha<$ident<T>>) -> Self {
        *Color::from_slice(&other.as_ref().as_ref()[..$channels])
    }
}

)* // END Structure definitions

/// An enumeration over supported color types and their bit depths.
#[derive(Copy, PartialEq, Eq, Debug, Clone)]
pub enum ColorType {
    $(#[$doc]$ident(u8),)*
}

impl ColorType {
    /// Returns the number of bits contained in a pixel of ColorType `self`.
    pub fn bits_per_pixel(self) -> usize {
        match self {
            $(ColorType::$ident(n) => $channels * n as usize,)*
        }
    }

    /// Returns the number of color channels that are in a pixel of ColorType `self`.
    pub fn num_components(self) -> usize {
        match self {
            $(ColorType::$ident(_) => $channels,)*
        }
    }
}

    }
}

define_color_model! {
    Rgb, 3, 0, "RGB", #[doc = "sRGB."];
    Xyz, 3, 0, "XYZ", #[doc = "CIE XYZ."];
    Lab, 3, 0, "CIE Lab", #[doc = "CIE L*a*b*."];
    Gray, 1, 0, "Y", #[doc = "Grayscale"];
    Indexed, 1, 0, "Idx", #[doc = "Indexed colors.\n\nNo specific color moddel is assumed."];
}

pub type Rgba<T> = Alpha<Rgb<T>>;
pub type Xyza<T> = Alpha<Xyz<T>>;
pub type LabA<T> = Alpha<Lab<T>>;
pub type GrayA<T> = Alpha<Gray<T>>;

#[test]
fn test_add() {
    let a: Alpha<Rgb<u8>> = Alpha::new([0, 0, 0, 0]);
    let b = a + 1;
    assert_eq!(&[1, 1, 1, 1], b.as_ref());
    assert_eq!(&[2, 2, 2, 2], (b + b).as_ref());
}
