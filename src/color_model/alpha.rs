use std::mem;
use std::ops::{Index, IndexMut};
use num_traits::Zero;

use traits::{Color, ColorMathOps, ChannelMax};

macro_rules! implement_alpha {
    {$(
        $ident: ident,
        $CHANNELS: expr;
    )*} => {
$( // START Structure definitions

/// Color with an associated alpha value.
#[derive(Copy, Clone)]
pub struct $ident<C: Color>([C::Subpixel; $CHANNELS]);

impl<C: Color> $ident<C> {
    pub fn new(array: [C::Subpixel; $CHANNELS]) -> Self {
        $ident(array)
    }
}

impl<C: Color> AsRef<<$ident<C> as Color>::Storage> for $ident<C> {
    fn as_ref(&self) -> &<Self as Color>::Storage {
        &self.0
    }
}

impl<C: Color> AsMut<<$ident<C> as Color>::Storage> for $ident<C> {
    fn as_mut(&mut self) -> &mut <Self as Color>::Storage {
        &mut self.0
    }
}

impl<C: Color> Color for $ident<C> {
    type Subpixel = C::Subpixel;
    type Storage = [C::Subpixel; $CHANNELS];

    /// Returns the number of channels of this pixel type.
    fn channel_count() -> usize {
        $CHANNELS
    }

    #[inline(always)]
    fn channels(&self) -> &[Self::Subpixel; $CHANNELS] {
        &self.0
    }

    #[inline(always)]
    fn channels_mut(&mut self) -> &mut [Self::Subpixel; $CHANNELS] {
        &mut self.0
    }

    fn from_channels(other: [Self::Subpixel; $CHANNELS]) -> Self {
        *<Self as Color>::from_slice(&other[..$CHANNELS])
    }

    fn from_slice<'a>(slice: &'a [Self::Subpixel]) -> &'a Self {
        unsafe {
            assert_eq!(slice.len(), $CHANNELS);
            mem::transmute(slice.as_ptr())
        }
    }

    fn from_slice_mut<'a>(slice: &'a mut [Self::Subpixel]) -> &'a mut Self {
        unsafe {
            assert_eq!(slice.len(), $CHANNELS);
            mem::transmute(slice.as_ptr())
        }
    }

    fn apply_with_alpha<F, G>(&mut self, f: F, g: G)
        where F: Fn(Self::Subpixel) -> Self::Subpixel,
              G: Fn(Self::Subpixel) -> Self::Subpixel
    {
        for v in self.0[..$CHANNELS as usize - 1 as usize].iter_mut() {
            *v = f(*v)
        }
        if $CHANNELS as usize != 0 {
            let v = &mut self.0[$CHANNELS - 1];
            *v = g(*v)
        }
    }

    fn color_model() -> &'static str {
        C::color_model()
    }
}



impl<C: Color> Index<usize> for $ident<C> {
    type Output = C::Subpixel;
    #[inline(always)]
    fn index<'a>(&'a self, _index: usize) -> &'a C::Subpixel {
        &self.0[_index]
    }
}

impl<C: Color> IndexMut<usize> for $ident<C> {
    #[inline(always)]
    fn index_mut<'a>(&'a mut self, _index: usize) -> &'a mut C::Subpixel {
        &mut self.0[_index]
    }
}

impl<C: Color> ColorMathOps<$ident<C>> for $ident<C> {
    #[inline(always)]
    fn add(mut self, rhs: Self) -> Self {
        for i in 0..$CHANNELS {
            self.0[i] = self.0[i] + rhs.0[i]
        }
        self
    }
    #[inline(always)]
    fn sub(mut self, rhs: Self) -> Self {
        for i in 0..$CHANNELS {
            self.0[i] = self.0[i] - rhs.0[i]
        }
        self
    }
    #[inline(always)]
    fn div(mut self, rhs: Self) -> Self {
        for i in 0..$CHANNELS {
            self.0[i] = self.0[i] / rhs.0[i]
        }
        self
    }
    #[inline(always)]
    fn mul(mut self, rhs: Self) -> Self {
        for i in 0..$CHANNELS {
            self.0[i] = self.0[i] * rhs.0[i]
        }
        self
    }
}

impl<C: Color, T: ColorMathOps<$ident<C>>> ::std::ops::Add<T> for $ident<C> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        rhs.add(self)
    }
}

impl<C: Color> ::std::ops::AddAssign for $ident<C> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<C: Color> ::std::ops::Sub for $ident<C> {
    type Output = Self;
    #[inline]
    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..$CHANNELS {
            self.0[i] = self.0[i] - rhs.0[i]
        }
        self
    }
}

impl<C: Color> ::std::ops::SubAssign for $ident<C> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<C: Color> ::std::ops::Div for $ident<C> {
    type Output = Self;
    #[inline]
    fn div(mut self, rhs: Self) -> Self::Output {
        for i in 0..$CHANNELS {
            self.0[i] = self.0[i] / rhs.0[i]
        }
        self
    }
}

impl<C: Color> ::std::ops::DivAssign for $ident<C> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<C: Color> ::std::ops::Mul for $ident<C> {
    type Output = Self;
    #[inline]
    fn mul(mut self, rhs: Self) -> Self::Output {
        for i in 0..$CHANNELS {
            self.0[i] = self.0[i] * rhs.0[i]
        }
        self
    }
}

impl<C: Color> ::std::ops::MulAssign for $ident<C> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<C: Color> From<C> for $ident<C>
    where C::Subpixel: ChannelMax
{
    fn from(other: C) -> Self {
        let mut storage = [Zero::zero(); $CHANNELS];
        storage.as_mut()[..C::channel_count()].copy_from_slice(other.as_ref().as_ref());
        storage[$CHANNELS - 1] = ChannelMax::channel_max();
        $ident(storage)
    }
}

)* // END Structure definitions

    }
}

implement_alpha!(
    Alpha2, 2;
    Alpha3, 3;
    Alpha4, 4;
);
