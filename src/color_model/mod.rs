mod cie;
mod gray;
mod rgb;

use std::ops::{Index, IndexMut};
use std::mem;

use traits::Pixel;
use traits::Primitive;

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
pub struct $ident<T: Primitive>(pub [T; $channels]);

impl<T: Primitive + 'static> Pixel for $ident<T> {

    type Subpixel = T;
    type Storage = [T; $channels];

    /// Returns the number of channels of this pixel type.
    fn channel_count() -> u8 {
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
        *<$ident<T> as Pixel>::from_slice(&other[..$channels])
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

    fn map<F>(& self, f: F) -> $ident<T> where F: Fn(T) -> T {
        let mut this = (*self).clone();
        this.apply(f);
        this
    }

    fn apply<F>(&mut self, f: F) where F: Fn(T) -> T {
        for v in self.0.iter_mut() {
            *v = f(*v)
        }
    }

    fn map_with_alpha<F, G>(&self, f: F, g: G) -> $ident<T> where F: Fn(T) -> T, G: Fn(T) -> T {
        let mut this = (*self).clone();
        this.apply_with_alpha(f, g);
        this
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

    fn map2<F>(&self, other: &Self, f: F) -> $ident<T> where F: Fn(T, T) -> T {
        let mut this = (*self).clone();
        this.apply2(other, f);
        this
    }

    fn apply2<F>(&mut self, other: &$ident<T>, f: F) where F: Fn(T, T) -> T {
        for (a, &b) in self.0.iter_mut().zip(other.0.iter()) {
            *a = f(*a, b)
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

impl<T: Primitive> ::std::ops::Add for $ident<T> {
    type Output = $ident<T>;
    #[inline]
    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..$channels {
            self.0[i] = self.0[i] + rhs.0[i]
        }
        self
    }
}

impl<T: Primitive> ::std::ops::AddAssign for $ident<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T: Primitive> ::std::ops::Add<T> for $ident<T> {
    type Output = $ident<T>;
    #[inline]
    fn add(mut self, rhs: T) -> Self::Output {
        for i in 0..$channels {
            self.0[i] = self.0[i] + rhs
        }
        self
    }
}

impl<T: Primitive> ::std::ops::AddAssign<T> for $ident<T> {
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl<T: Primitive> ::std::ops::Sub for $ident<T> {
    type Output = $ident<T>;
    #[inline]
    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..$channels {
            self.0[i] = self.0[i] - rhs.0[i]
        }
        self
    }
}

impl<T: Primitive> ::std::ops::SubAssign for $ident<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<T: Primitive> ::std::ops::Sub<T> for $ident<T> {
    type Output = $ident<T>;
    #[inline]
    fn sub(mut self, rhs: T) -> Self::Output {
        for i in 0..$channels {
            self.0[i] = self.0[i] - rhs
        }
        self
    }
}

impl<T: Primitive> ::std::ops::SubAssign<T> for $ident<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

impl<T: Primitive> ::std::ops::Div for $ident<T> {
    type Output = $ident<T>;
    #[inline]
    fn div(mut self, rhs: Self) -> Self::Output {
        for i in 0..$channels {
            self.0[i] = self.0[i] / rhs.0[i]
        }
        self
    }
}

impl<T: Primitive> ::std::ops::DivAssign for $ident<T> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<T: Primitive> ::std::ops::Div<T> for $ident<T> {
    type Output = $ident<T>;
    #[inline]
    fn div(mut self, rhs: T) -> Self::Output {
        for i in 0..$channels {
            self.0[i] = self.0[i] / rhs
        }
        self
    }
}

impl<T: Primitive> ::std::ops::DivAssign<T> for $ident<T> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl<T: Primitive> ::std::ops::Mul for $ident<T> {
    type Output = $ident<T>;
    #[inline]
    fn mul(mut self, rhs: Self) -> Self::Output {
        for i in 0..$channels {
            self.0[i] = self.0[i] * rhs.0[i]
        }
        self
    }
}

impl<T: Primitive> ::std::ops::MulAssign for $ident<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T: Primitive> ::std::ops::Mul<T> for $ident<T> {
    type Output = $ident<T>;
    #[inline]
    fn mul(mut self, rhs: T) -> Self::Output {
        for i in 0..$channels {
            self.0[i] = self.0[i] * rhs
        }
        self
    }
}

impl<T: Primitive> ::std::ops::MulAssign<T> for $ident<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
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
    Rgba, 4, 1, "RGBA", #[doc = "sRGB + alpha channel."];
    Bgra, 4, 1, "BGRA", #[doc = "BGRA.\n\nFor convenience. Only conversion with RGBA is defined."];
    Xyz, 3, 0, "XYZ", #[doc = "CIE XYZ."];
    XyzA, 4, 1, "XYZ", #[doc = "CIE XYZ + alpha channel."];
    Lab, 3, 0, "CIE Lab", #[doc = "CIE L*a*b*."];
    LabA, 4, 1, "CIE Lab alpha", #[doc = "CIE L*a*b* + alpha channel."];
    Gray, 1, 0, "Y", #[doc = "Grayscale"];
    GrayA, 2, 1, "YA", #[doc = "Grayscale + alpha channel."];
    Indexed, 1, 0, "Idx", #[doc = "Indexed colors.\n\nNo specific color moddel is assumed."];
}
