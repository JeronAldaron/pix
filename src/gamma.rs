// gamma.rs     Gamma encoding/decoding for sRGB
//
// Copyright (c) 2019  Douglas P Lau
//
use std::fmt::Debug;
use crate::{Ch16, Ch32, Ch8, Channel};

/// Trait to encode/decode gamma
pub trait Gamma {
    /// Encode an sRGB gamma value from linear intensity
    fn encode_srgb(self) -> Self;
    /// Decode an sRGB gamma value into linear intensity
    fn decode_srgb(self) -> Self;
}

impl Gamma for u8 {
    /// Encode an sRGB gamma value from linear intensity
    fn encode_srgb(self) -> Self {
        ENCODE_SRGB_U8[usize::from(self)]
    }
    /// Decode an sRGB gamma value into linear intensity
    fn decode_srgb(self) -> Self {
        DECODE_SRGB_U8[usize::from(self)]
    }
}

/// Look-up table to encode 8-bit linear to sRGB gamma
const ENCODE_SRGB_U8: &[u8] = &[
    0x00, 0x0D, 0x16, 0x1C, 0x22, 0x26, 0x2A, 0x2E, 0x32, 0x35, 0x38, 0x3B,
    0x3D, 0x40, 0x42, 0x45, 0x47, 0x49, 0x4B, 0x4D, 0x4F, 0x51, 0x53, 0x55,
    0x56, 0x58, 0x5A, 0x5C, 0x5D, 0x5F, 0x60, 0x62, // 32
    0x63, 0x65, 0x66, 0x68, 0x69, 0x6A, 0x6C, 0x6D, 0x6E, 0x70, 0x71, 0x72,
    0x73, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7A, 0x7C, 0x7D, 0x7E, 0x7F, 0x80,
    0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, // 64
    0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E, 0x8F, 0x90, 0x91, 0x92, 0x93, 0x94,
    0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9A, 0x9B, 0x9B, 0x9C, 0x9D, 0x9E,
    0x9F, 0x9F, 0xA0, 0xA1, 0xA2, 0xA3, 0xA3, 0xA4, // 96
    0xA5, 0xA6, 0xA7, 0xA7, 0xA8, 0xA9, 0xAA, 0xAA, 0xAB, 0xAC, 0xAD, 0xAD,
    0xAE, 0xAF, 0xAF, 0xB0, 0xB1, 0xB2, 0xB2, 0xB3, 0xB4, 0xB4, 0xB5, 0xB6,
    0xB6, 0xB7, 0xB8, 0xB9, 0xB9, 0xBA, 0xBB, 0xBB, // 128
    0xBC, 0xBD, 0xBD, 0xBE, 0xBE, 0xBF, 0xC0, 0xC0, 0xC1, 0xC2, 0xC2, 0xC3,
    0xC4, 0xC4, 0xC5, 0xC5, 0xC6, 0xC7, 0xC7, 0xC8, 0xC8, 0xC9, 0xCA, 0xCA,
    0xCB, 0xCB, 0xCC, 0xCD, 0xCD, 0xCE, 0xCE, 0xCF, // 160
    0xD0, 0xD0, 0xD1, 0xD1, 0xD2, 0xD2, 0xD3, 0xD4, 0xD4, 0xD5, 0xD5, 0xD6,
    0xD6, 0xD7, 0xD7, 0xD8, 0xD8, 0xD9, 0xDA, 0xDA, 0xDB, 0xDB, 0xDC, 0xDC,
    0xDD, 0xDD, 0xDE, 0xDE, 0xDF, 0xDF, 0xE0, 0xE0, // 192
    0xE1, 0xE2, 0xE2, 0xE3, 0xE3, 0xE4, 0xE4, 0xE5, 0xE5, 0xE6, 0xE6, 0xE7,
    0xE7, 0xE8, 0xE8, 0xE9, 0xE9, 0xEA, 0xEA, 0xEB, 0xEB, 0xEC, 0xEC, 0xED,
    0xED, 0xEE, 0xEE, 0xEE, 0xEF, 0xEF, 0xF0, 0xF0, // 224
    0xF1, 0xF1, 0xF2, 0xF2, 0xF3, 0xF3, 0xF4, 0xF4, 0xF5, 0xF5, 0xF6, 0xF6,
    0xF6, 0xF7, 0xF7, 0xF8, 0xF8, 0xF9, 0xF9, 0xFA, 0xFA, 0xFB, 0xFB, 0xFB,
    0xFC, 0xFC, 0xFD, 0xFD, 0xFE, 0xFE, 0xFF, 0xFF, // 256
];

/// Look-up table to decode 8-bit sRGB gamma to linear
const DECODE_SRGB_U8: &[u8] = &[
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
    0x02, 0x02, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, // 32
    0x04, 0x04, 0x04, 0x04, 0x04, 0x05, 0x05, 0x05, 0x05, 0x06, 0x06, 0x06,
    0x06, 0x07, 0x07, 0x07, 0x08, 0x08, 0x08, 0x08, 0x09, 0x09, 0x09, 0x0A,
    0x0A, 0x0A, 0x0B, 0x0B, 0x0C, 0x0C, 0x0C, 0x0D, // 64
    0x0D, 0x0D, 0x0E, 0x0E, 0x0F, 0x0F, 0x10, 0x10, 0x11, 0x11, 0x11, 0x12,
    0x12, 0x13, 0x13, 0x14, 0x14, 0x15, 0x16, 0x16, 0x17, 0x17, 0x18, 0x18,
    0x19, 0x19, 0x1A, 0x1B, 0x1B, 0x1C, 0x1D, 0x1D, // 96
    0x1E, 0x1E, 0x1F, 0x20, 0x20, 0x21, 0x22, 0x23, 0x23, 0x24, 0x25, 0x25,
    0x26, 0x27, 0x28, 0x29, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2D, 0x2E, 0x2F,
    0x30, 0x31, 0x32, 0x33, 0x33, 0x34, 0x35, 0x36, // 128
    0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F, 0x40, 0x41, 0x42,
    0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4C, 0x4D, 0x4E, 0x4F,
    0x50, 0x51, 0x52, 0x54, 0x55, 0x56, 0x57, 0x58, // 160
    0x5A, 0x5B, 0x5C, 0x5D, 0x5F, 0x60, 0x61, 0x63, 0x64, 0x65, 0x67, 0x68,
    0x69, 0x6B, 0x6C, 0x6D, 0x6F, 0x70, 0x72, 0x73, 0x74, 0x76, 0x77, 0x79,
    0x7A, 0x7C, 0x7D, 0x7F, 0x80, 0x82, 0x83, 0x85, // 192
    0x86, 0x88, 0x8A, 0x8B, 0x8D, 0x8E, 0x90, 0x92, 0x93, 0x95, 0x97, 0x98,
    0x9A, 0x9C, 0x9D, 0x9F, 0xA1, 0xA3, 0xA4, 0xA6, 0xA8, 0xAA, 0xAB, 0xAD,
    0xAF, 0xB1, 0xB3, 0xB5, 0xB7, 0xB8, 0xBA, 0xBC, // 224
    0xBE, 0xC0, 0xC2, 0xC4, 0xC6, 0xC8, 0xCA, 0xCC, 0xCE, 0xD0, 0xD2, 0xD4,
    0xD6, 0xD8, 0xDA, 0xDC, 0xDE, 0xE0, 0xE2, 0xE5, 0xE7, 0xE9, 0xEB, 0xED,
    0xEF, 0xF2, 0xF4, 0xF6, 0xF8, 0xFA, 0xFD, 0xFF, // 256
];

impl Gamma for Ch8 {
    /// Encode an sRGB gamma value from linear intensity
    fn encode_srgb(self) -> Self {
        Self::new(u8::from(self).encode_srgb())
    }
    /// Decode an sRGB gamma value into linear intensity
    fn decode_srgb(self) -> Self {
        Self::new(u8::from(self).decode_srgb())
    }
}

impl Gamma for u16 {
    /// Encode an sRGB gamma value from linear intensity
    fn encode_srgb(self) -> Self {
        let s = f32::from(self) / 65535.0;
        (s.encode_srgb() * 65535.0).round() as u16
    }
    /// Decode an sRGB gamma value into linear intensity
    fn decode_srgb(self) -> Self {
        let s = f32::from(self) / 65535.0;
        (s.decode_srgb() * 65535.0).round() as u16
    }
}

impl Gamma for Ch16 {
    /// Encode an sRGB gamma value from linear intensity
    fn encode_srgb(self) -> Self {
        Self::new(u16::from(self).encode_srgb())
    }
    /// Decode an sRGB gamma value into linear intensity
    fn decode_srgb(self) -> Self {
        Self::new(u16::from(self).decode_srgb())
    }
}

impl Gamma for f32 {
    /// Encode an sRGB gamma value from linear intensity
    fn encode_srgb(self) -> Self {
        if self <= 0.0 {
            0.0
        } else if self < 0.003_130_8 {
            self * 12.92
        } else if self < 1.0 {
            self.powf(1.0 / 2.4) * 1.055 - 0.055
        } else {
            1.0
        }
    }
    /// Decode an sRGB gamma value into linear intensity
    fn decode_srgb(self) -> Self {
        if self <= 0.0 {
            0.0
        } else if self < 0.04045 {
            self / 12.92
        } else if self < 1.0 {
            ((self + 0.055) / 1.055).powf(2.4)
        } else {
            1.0
        }
    }
}

impl Gamma for Ch32 {
    /// Encode an sRGB gamma value from linear intensity
    fn encode_srgb(self) -> Self {
        Self::new(f32::from(self).encode_srgb())
    }
    /// Decode an sRGB gamma value into linear intensity
    fn decode_srgb(self) -> Self {
        Self::new(f32::from(self).decode_srgb())
    }
}

impl Gamma for f64 {
    /// Encode an sRGB gamma value from linear intensity
    fn encode_srgb(self) -> Self {
        if self <= 0.0 {
            0.0
        } else if self < 0.003_130_8 {
            self * 12.92
        } else if self < 1.0 {
            self.powf(1.0 / 2.4) * 1.055 - 0.055
        } else {
            1.0
        }
    }
    /// Decode an sRGB gamma value into linear intensity
    fn decode_srgb(self) -> Self {
        if self <= 0.0 {
            0.0
        } else if self < 0.04045 {
            self / 12.92
        } else if self < 1.0 {
            ((self + 0.055) / 1.055).powf(2.4)
        } else {
            1.0
        }
    }
}

/// No gamma correction applied
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Linear;

/// Gamma correction using the sRGB formula
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Srgb;

/// Gamma correction with a specified value
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct PowerLaw(f32);

/*/// No gamma correction - unknown.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct UnknownGamma;*/

/*impl<A: GammaMode> GammaConversion for GammaConverter<Linear, A> {
    /// Encode one `Channel` using the gamma mode.
    fn encode<C: Channel>(c: C) -> C {
        c
    }
    /// Decode one `Channel` using the gamma mode.
    fn decode<C: Channel>(c: C) -> C {
        c
    }
}*/

/// Trait for handling associated versus separated alpha
pub trait GammaMode: Copy + Clone + Debug + PartialEq + Default {
    const ID: GammaModeID;

    /// Encode one `Channel` using the gamma mode.
    fn encode<C: Channel, G: GammaMode>(c: C) -> C;
    /// Decode one `Channel` using the gamma mode.
    fn decode<C: Channel, G: GammaMode>(c: C) -> C;
}

impl GammaMode for Linear {
    const ID: GammaModeID = GammaModeID::Linear;

    /// Encode one `Channel` using the gamma mode.
    fn encode<C: Channel, G: GammaMode>(c: C) -> C {
        c
    }
    /// Decode one `Channel` using the gamma mode.
    fn decode<C: Channel, G: GammaMode>(c: C) -> C {
        c
    }
}

impl GammaMode for Srgb {
    const ID: GammaModeID = GammaModeID::Srgb;

    /// Encode one `Channel` using the gamma mode.
    fn encode<C: Channel, G: GammaMode>(c: C) -> C {
        if G::ID != Self::ID && G::ID != GammaModeID::UnknownGamma {
            encode_srgb(c)
        } else {
            c
        }
    }
    /// Decode one `Channel` using the gamma mode.
    fn decode<C: Channel, G: GammaMode>(c: C) -> C {
        if G::ID != Self::ID {
            decode_srgb(c)
        } else {
            c
        }
    }
}

impl PowerLaw {
    /// Encode one `Channel` using the gamma mode.
    pub fn encode<C: Channel>(c: C, g: f32) -> C {
        encode_power_law(c, g)
    }
    /// Decode one `Channel` using the gamma mode.
    pub fn decode<C: Channel>(c: C, g: f32) -> C {
        decode_power_law(c, g)
    }
}

/// Mode for handling gamma encoding / decoding.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GammaModeID {
    /// No gamma correction applied
    Linear,
    /// Gamma correction using the sRGB formula
    Srgb,
    /// Gamma correction with a specified value
    PowerLaw(f32),
    /// Unknown Gamma
    UnknownGamma,
}

fn encode_srgb<C: Channel>(c: C) -> C {
    c.encode_srgb()
}

fn decode_srgb<C: Channel>(c: C) -> C {
    c.decode_srgb()
}

fn encode_power_law<C: Channel>(c: C, g: f32) -> C {
    c.powf(g)
}

fn decode_power_law<C: Channel>(c: C, g: f32) -> C {
    c.powf(1.0 / g)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn lut_decode_u8() {
        for i in 0..256 {
            let s = i as f64 / 255.0;
            let v = (s.decode_srgb() * 255.0).round() as u8;
            assert_eq!(v, DECODE_SRGB_U8[i]);
        }
    }
    #[test]
    fn lut_encode_u8() {
        for i in 0..256 {
            let s = i as f64 / 255.0;
            let v = (s.encode_srgb() * 255.0).round() as u8;
            assert_eq!(v, ENCODE_SRGB_U8[i]);
        }
    }
}
