// rgb.rs       RGB pixel format.
//
// Copyright (c) 2018-2019  Douglas P Lau
//
use crate::{Alpha, Channel, Ch8, Ch16, Ch32, Format, Opaque, Translucent};

/// RGB pixel [Format](trait.Format.html), with optional
/// [Alpha](trait.Alpha.html) channel.
///
/// The channels are *red*, *green* and *blue*.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Rgb<C: Channel, A: Alpha> {
    red: C,
    green: C,
    blue: C,
    alpha: A,
}

impl<C: Channel, A: Alpha> Iterator for Rgb<C, A> {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        Some(*self)
    }
}

impl<C, A> From<Rgb<C, A>> for i32
    where C: Channel, Ch8: From<C>, A: Alpha<Chan=C>
{
    /// Get an i32 from an Rgb
    fn from(c: Rgb<C, A>) -> i32 {
        let red: u8 = Ch8::from(c.red()).into();
        let red = i32::from(red);
        let green: u8 = Ch8::from(c.green()).into();
        let green = i32::from(green) << 8;
        let blue: u8 = Ch8::from(c.blue()).into();
        let blue = i32::from(blue) << 16;
        let alpha: u8 = Ch8::from(c.alpha().value()).into();
        let alpha = i32::from(alpha) << 24;
        red | green | blue | alpha
    }
}

/// Convert an Rgb to another Rgb with a different Channel
macro_rules! from_impl_rgb {
    ( $c:tt, $h:tt ) => {
        impl<A, B> From<Rgb<$h, B>> for Rgb<$c, A>
            where A: Alpha, B: Alpha, A: From<B>
        {
            fn from(c: Rgb<$h, B>) -> Self {
                let red = c.red().into();
                let green = c.green().into();
                let blue = c.blue().into();
                let alpha = c.alpha().into();
                Rgb { red, green, blue, alpha }
            }
        }
    };
}

from_impl_rgb!(Ch8, Ch16);
from_impl_rgb!(Ch8, Ch32);
from_impl_rgb!(Ch16, Ch8);
from_impl_rgb!(Ch16, Ch32);
from_impl_rgb!(Ch32, Ch8);
from_impl_rgb!(Ch32, Ch16);

/// Convert an Rgb to another Rgb with a different Alpha
macro_rules! from_impl_alpha {
    ( $c:tt, $s:tt, $d:tt ) => {
        impl From<Rgb<$c, $s<$c>>> for Rgb<$c, $d<$c>> {
            fn from(c: Rgb<$c, $s<$c>>) -> Self {
                let red = c.red().into();
                let green = c.green().into();
                let blue = c.blue().into();
                let alpha = c.alpha().into();
                Rgb { red, green, blue, alpha }
            }
        }
    };
}

from_impl_alpha!(Ch8, Translucent, Opaque);
from_impl_alpha!(Ch16, Translucent, Opaque);
from_impl_alpha!(Ch32, Translucent, Opaque);
from_impl_alpha!(Ch8, Opaque, Translucent);
from_impl_alpha!(Ch16, Opaque, Translucent);
from_impl_alpha!(Ch32, Opaque, Translucent);

impl<C: Channel, A: Alpha> Rgb<C, A> {
    /// Build a color by specifying red, green and blue values.
    pub fn new<H>(red: H, green: H, blue: H) -> Self
        where C: From<H>, A: From<Opaque<C>>
    {
        let red = C::from(red);
        let green = C::from(green);
        let blue = C::from(blue);
        let alpha = A::from(Opaque::default());
        Rgb { red, green, blue, alpha }
    }
    /// Create a color by specifying red, green, blue and alpha values.
    pub fn with_alpha<H, B>(red: H, green: H, blue: H, alpha: B) -> Self
        where C: From<H>, A: From<B>
    {
        let red = C::from(red);
        let green = C::from(green);
        let blue = C::from(blue);
        let alpha = A::from(alpha);
        // FIXME: do alpha properly and premultiply if necessary
        Rgb { red, green, blue, alpha }
    }
    /// Get the red channel.
    pub fn red(self) -> C {
        self.red
    }
    /// Get the green channel.
    pub fn green(self) -> C {
        self.green
    }
    /// Get the blue channel.
    pub fn blue(self) -> C {
        self.blue
    }
    /// Get the alpha channel.
    pub fn alpha(self) -> A {
        self.alpha
    }
}

impl<C: Channel, A: Alpha> Format for Rgb<C, A> {
    type Chan = C;
}

/// [Opaque](struct.Opaque.html) 8-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgb8 = Rgb<Ch8, Opaque<Ch8>>;

/// [Opaque](struct.Opaque.html) 16-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgb16 = Rgb<Ch16, Opaque<Ch16>>;

/// [Opaque](struct.Opaque.html) 32-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgb32 = Rgb<Ch32, Opaque<Ch32>>;

/// [Translucent](struct.Translucent.html) 8-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgba8 = Rgb<Ch8, Translucent<Ch8>>;

/// [Translucent](struct.Translucent.html) 16-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgba16 = Rgb<Ch16, Translucent<Ch16>>;

/// [Translucent](struct.Translucent.html) 32-bit [Rgb](struct.Rgb.html) pixel
/// [Format](trait.Format.html).
pub type Rgba32 = Rgb<Ch32, Translucent<Ch32>>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_sizes() {
        assert_eq!(std::mem::size_of::<Rgb8>(), 3);
        assert_eq!(std::mem::size_of::<Rgb16>(), 6);
        assert_eq!(std::mem::size_of::<Rgb32>(), 12);
        assert_eq!(std::mem::size_of::<Rgba8>(), 4);
        assert_eq!(std::mem::size_of::<Rgba16>(), 8);
        assert_eq!(std::mem::size_of::<Rgba32>(), 16);
    }
}
