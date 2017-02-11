use std::ops::{Index, IndexMut};
use num_traits::{Bounded, Num, NumCast};

/// A generalized pixel.
///
/// A pixel object is usually not used standalone but as a view into an image buffer.
pub trait Color
    : Copy + Clone + AsRef<<Self as Color>::Storage> + AsMut<<Self as Color>::Storage> + 'static
    {
    /// The underlying subpixel type.

    type Subpixel: Primitive;
    // TODO: Workaround until associated consts work.
    type Storage: AsRef<[Self::Subpixel]> + AsMut<[Self::Subpixel]> + 'static;
    // TODO: The preferred solution would be:
    // type Subpixel: Primitive;
    // const NUM_CHANNELS: usize;

    /// Returns the number of channels of this pixel type.
    // TODO: Remove is NUM_CHANNELS is available
    fn channel_count() -> usize;

    /// Returns the components as a slice.
    fn channels(&self) -> &Self::Storage;
    // TODO: The preferred solution would be:
    // fn channels(&self) -> &[Self::Subpixel; Self::NUM_CHANNELS];

    /// Returns the components as a mutable slice
    fn channels_mut(&mut self) -> &mut Self::Storage;
    // TODO: The preferred solution would be:
    // fn channels_mut(&mut self) -> &mut [Self::Subpixel; Self::NUM_CHANNELS];

    /// Construct a pixel from the 4 channels a, b, c and d.
    /// If the pixel does not contain 4 channels the extra are ignored.
    fn from_channels(Self::Storage) -> Self;
    // TODO: The preferred solution would be:
    // fn from_channels([Self::Subpixel; Self::NUM_CHANNELS]) -> Self;

    /// Returns a string that can help to interprete the meaning each channel
    /// See [gimp babl](http://gegl.org/babl/).
    fn color_model() -> &'static str;

    /// Returns a view into a slice.
    ///
    /// # Panics
    ///
    /// If the slice it not long enough this method will panic.
    fn from_slice<'a>(slice: &'a [Self::Subpixel]) -> &'a Self;

    /// Returns mutable view into a mutable slice.
    ///
    /// # Panics
    ///
    /// If the slice it not long enough this method will panic.
    fn from_slice_mut<'a>(slice: &'a mut [Self::Subpixel]) -> &'a mut Self;

    /// Apply the function ```f``` to each channel of this pixel.
    fn map<F>(&self, f: F) -> Self
        where F: Fn(Self::Subpixel) -> Self::Subpixel
    {
        let mut this = (*self).clone();
        this.apply(f);
        this
    }

    /// Apply the function ```f``` to each channel of this pixel.
    fn apply<F>(&mut self, f: F)
        where F: Fn(Self::Subpixel) -> Self::Subpixel
    {
        for v in self.as_mut().as_mut().iter_mut() {
            *v = f(*v)
        }
    }

    /// Apply the function ```f``` to each channel except the alpha channel.
    /// Apply the function ```g``` to the alpha channel.
    fn map_with_alpha<F, G>(&self, f: F, g: G) -> Self
        where F: Fn(Self::Subpixel) -> Self::Subpixel,
              G: Fn(Self::Subpixel) -> Self::Subpixel
    {
        let mut this = (*self).clone();
        this.apply_with_alpha(f, g);
        this
    }

    /// Apply the function ```f``` to each channel except the alpha channel.
    /// Apply the function ```g``` to the alpha channel. Works in-place.
    fn apply_with_alpha<F, G>(&mut self, f: F, g: G)
        where F: Fn(Self::Subpixel) -> Self::Subpixel,
              G: Fn(Self::Subpixel) -> Self::Subpixel;

    /// Apply the function ```f``` to each channel of this pixel and
    /// ```other``` pairwise.
    fn map2<F>(&self, other: &Self, f: F) -> Self
        where F: Fn(Self::Subpixel, Self::Subpixel) -> Self::Subpixel
    {
        let mut this = (*self).clone();
        this.apply2(other, f);
        this
    }
    /// Apply the function ```f``` to each channel of this pixel and
    /// ```other``` pairwise. Works in-place.
    fn apply2<F>(&mut self, other: &Self, f: F)
        where F: Fn(Self::Subpixel, Self::Subpixel) -> Self::Subpixel
    {
        for (a, &b) in self.as_mut().as_mut().iter_mut().zip(other.as_ref().as_ref().iter()) {
            *a = f(*a, b)
        }

    }
}

/// Color math operations.
///
/// Math operations on a color. Uses double dispatch to avoid type problems due to conflicting
/// implementations of `Add` and friends.
pub trait ColorMathOps<C: Color>: Sized {
    #[inline(always)]
    fn add(self, rhs: C) -> C;
    #[inline(always)]
    fn sub(self, rhs: C) -> C;
    #[inline(always)]
    fn div(self, rhs: C) -> C;
    #[inline(always)]
    fn mul(self, rhs: C) -> C;
}

/// A view into an image
pub trait ImageView<P: Color>
    : Index<(u32, u32), Output = P> + IndexMut<(u32, u32)> {
}

/// Returns value which is used to scale a value of a channel.
///
/// Returns `T::max_value()` for unsigned integers and `1.0` for floats.
pub trait ChannelMax {
    fn channel_max() -> Self;
}

impl ChannelMax for usize {
    fn channel_max() -> Self {
        usize::max_value()
    }
}
impl ChannelMax for u8 {
    fn channel_max() -> Self {
        u8::max_value()
    }
}
impl ChannelMax for u16 {
    fn channel_max() -> Self {
        u16::max_value()
    }
}
impl ChannelMax for u32 {
    fn channel_max() -> Self {
        u32::max_value()
    }
}
impl ChannelMax for u64 {
    fn channel_max() -> Self {
        u64::max_value()
    }
}
impl ChannelMax for f32 {
    fn channel_max() -> Self {
        1.0
    }
}
impl ChannelMax for f64 {
    fn channel_max() -> Self {
        1.0
    }
}

/// `Primitive` trait from old stdlib.
pub trait Primitive
    : Copy + Clone + NumCast + Num + PartialOrd<Self> + Bounded + 'static {
}

macro_rules! primitive_impls {
    {$(
        $ident: ident,
    )*} => {
$( // START Implementations

impl Primitive for $ident {}

impl<C: Color<Subpixel=$ident>> ColorMathOps<C> for $ident
    where C::Storage: AsRef<[$ident]> + AsMut<[$ident]>
{
    #[inline(always)]
    fn add(self, mut rhs: C) -> C {
        for val in rhs.as_mut().as_mut() {
            *val = *val + self
        }
        rhs
    }
    #[inline(always)]
    fn sub(self, mut rhs: C) -> C {
        for val in rhs.as_mut().as_mut() {
            *val = *val - self
        }
        rhs
    }
    #[inline(always)]
    fn div(self, mut rhs: C) -> C {
        for val in rhs.as_mut().as_mut() {
            *val = *val / self
        }
        rhs
    }
    #[inline(always)]
    fn mul(self, mut rhs: C) -> C {
        for val in rhs.as_mut().as_mut() {
            *val = *val * self
        }
        rhs
    }
}

)* // END Implementations

    }
}

primitive_impls!(
    usize,
    u8,
    u16,
    u32,
    u64,
    isize,
    i8,
    i16,
    i32,
    i64,
    f32,
    f64,
);
