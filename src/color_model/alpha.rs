use std::mem;
use std::ops::{Index, IndexMut};
use num_traits::Zero;

use traits::{Color, ColorMathOps, ChannelMax};

const ALPHA_CHANNELS: usize = 4;

/// Color with an associated alpha value.
#[derive(Copy, Clone)]
pub struct Alpha<C: Color>([C::Subpixel; ALPHA_CHANNELS]);

impl<C: Color> Alpha<C> {
    pub fn new(array: [C::Subpixel; ALPHA_CHANNELS]) -> Self {
        Alpha(array)
    }
}

impl<C: Color> AsRef<<Alpha<C> as Color>::Storage> for Alpha<C> {
    fn as_ref(&self) -> &<Self as Color>::Storage {
        &self.0
    }
}

impl<C: Color> AsMut<<Alpha<C> as Color>::Storage> for Alpha<C> {
    fn as_mut(&mut self) -> &mut <Self as Color>::Storage {
        &mut self.0
    }
}

impl<C: Color> Color for Alpha<C> {
    type Subpixel = C::Subpixel;
    type Storage = [C::Subpixel; ALPHA_CHANNELS];

    /// Returns the number of channels of this pixel type.
    fn channel_count() -> usize {
        ALPHA_CHANNELS
    }

    #[inline(always)]
    fn channels(&self) -> &[Self::Subpixel; ALPHA_CHANNELS] {
        &self.0
    }

    #[inline(always)]
    fn channels_mut(&mut self) -> &mut [Self::Subpixel; ALPHA_CHANNELS] {
        &mut self.0
    }

    fn from_channels(other: [Self::Subpixel; ALPHA_CHANNELS]) -> Self {
        *<Self as Color>::from_slice(&other[..ALPHA_CHANNELS])
    }

    fn from_slice<'a>(slice: &'a [Self::Subpixel]) -> &'a Self {
        unsafe {
            assert_eq!(slice.len(), ALPHA_CHANNELS);
            mem::transmute(slice.as_ptr())
        }
    }

    fn from_slice_mut<'a>(slice: &'a mut [Self::Subpixel]) -> &'a mut Self {
        unsafe {
            assert_eq!(slice.len(), ALPHA_CHANNELS);
            mem::transmute(slice.as_ptr())
        }
    }

    fn apply_with_alpha<F, G>(&mut self, f: F, g: G)
        where F: Fn(Self::Subpixel) -> Self::Subpixel,
              G: Fn(Self::Subpixel) -> Self::Subpixel
    {
        for v in self.0[..ALPHA_CHANNELS as usize - 1 as usize].iter_mut() {
            *v = f(*v)
        }
        if ALPHA_CHANNELS as usize != 0 {
            let v = &mut self.0[ALPHA_CHANNELS - 1];
            *v = g(*v)
        }
    }

    fn color_model() -> &'static str {
        C::color_model()
    }
}



impl<C: Color> Index<usize> for Alpha<C> {
    type Output = C::Subpixel;
    #[inline(always)]
    fn index<'a>(&'a self, _index: usize) -> &'a C::Subpixel {
        &self.0[_index]
    }
}

impl<C: Color> IndexMut<usize> for Alpha<C> {
    #[inline(always)]
    fn index_mut<'a>(&'a mut self, _index: usize) -> &'a mut C::Subpixel {
        &mut self.0[_index]
    }
}

impl<C: Color> ColorMathOps<Alpha<C>> for Alpha<C> {
    #[inline(always)]
    fn add(mut self, rhs: Self) -> Self {
        for i in 0..ALPHA_CHANNELS {
            self.0[i] = self.0[i] + rhs.0[i]
        }
        self
    }
    #[inline(always)]
    fn sub(mut self, rhs: Self) -> Self {
        for i in 0..ALPHA_CHANNELS {
            self.0[i] = self.0[i] - rhs.0[i]
        }
        self
    }
    #[inline(always)]
    fn div(mut self, rhs: Self) -> Self {
        for i in 0..ALPHA_CHANNELS {
            self.0[i] = self.0[i] / rhs.0[i]
        }
        self
    }
    #[inline(always)]
    fn mul(mut self, rhs: Self) -> Self {
        for i in 0..ALPHA_CHANNELS {
            self.0[i] = self.0[i] * rhs.0[i]
        }
        self
    }
}

impl<C: Color, T: ColorMathOps<Alpha<C>>> ::std::ops::Add<T> for Alpha<C> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        rhs.add(self)
    }
}

impl<C: Color> ::std::ops::AddAssign for Alpha<C> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<C: Color> ::std::ops::Sub for Alpha<C> {
    type Output = Self;
    #[inline]
    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..ALPHA_CHANNELS {
            self.0[i] = self.0[i] - rhs.0[i]
        }
        self
    }
}

impl<C: Color> ::std::ops::SubAssign for Alpha<C> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<C: Color> ::std::ops::Div for Alpha<C> {
    type Output = Self;
    #[inline]
    fn div(mut self, rhs: Self) -> Self::Output {
        for i in 0..ALPHA_CHANNELS {
            self.0[i] = self.0[i] / rhs.0[i]
        }
        self
    }
}

impl<C: Color> ::std::ops::DivAssign for Alpha<C> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<C: Color> ::std::ops::Mul for Alpha<C> {
    type Output = Self;
    #[inline]
    fn mul(mut self, rhs: Self) -> Self::Output {
        for i in 0..ALPHA_CHANNELS {
            self.0[i] = self.0[i] * rhs.0[i]
        }
        self
    }
}

impl<C: Color> ::std::ops::MulAssign for Alpha<C> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<C: Color> From<C> for Alpha<C>
    where C::Subpixel: ChannelMax
{
    fn from(other: C) -> Self {
        let mut storage = [Zero::zero(); ALPHA_CHANNELS];
        storage.as_mut()[..C::channel_count()].copy_from_slice(other.as_ref().as_ref());
        storage[ALPHA_CHANNELS - 1] = ChannelMax::channel_max();
        Alpha(storage)
    }
}
