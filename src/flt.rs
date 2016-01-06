use std::num;
use std::fmt;
use std::error::{Error};
use std::str::{from_utf8_unchecked};

use base::{FromAscii};

/// An error returned when parsing a floating value from a ascii string fails.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseFloatError {
    kind: FloatErrorKind
}

#[derive(Debug, Clone, PartialEq)]
enum FloatErrorKind {
    FloatErrorKind
}

impl Error for ParseFloatError {
    fn description(&self) -> &str {
        "failed to parse float"
    }
}

impl From<num::ParseFloatError> for ParseFloatError {
    fn from(t: num::ParseFloatError) -> ParseFloatError {
        ParseFloatError { kind: FloatErrorKind::FloatErrorKind }
    }
}

impl fmt::Display for ParseFloatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl FromAscii for f32 {
    type Err = ParseFloatError;

    #[inline]
    fn from_ascii(src: &[u8]) -> Result<Self, Self::Err> {
        unsafe {
            from_utf8_unchecked(src).parse().map_err(From::from)
        }
    }
}

impl FromAscii for f64 {
    type Err = ParseFloatError;

    #[inline]
    fn from_ascii(src: &[u8]) -> Result<Self, Self::Err> {
        unsafe {
            from_utf8_unchecked(src).parse().map_err(From::from)
        }
    }
}
