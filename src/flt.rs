pub use std::num::ParseFloatError;
use std::str::{from_utf8_unchecked};

use base::{FromAscii};

impl FromAscii for f32 {
    type Err = ParseFloatError;

    #[inline]
    fn from_ascii(src: &[u8]) -> Result<Self, Self::Err> {
        unsafe {
            from_utf8_unchecked(src).parse()
        }
    }
}

impl FromAscii for f64 {
    type Err = ParseFloatError;

    #[inline]
    fn from_ascii(src: &[u8]) -> Result<Self, Self::Err> {
        unsafe {
            from_utf8_unchecked(src).parse()
        }
    }
}
