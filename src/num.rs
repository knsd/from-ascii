// Copyright (c) 2016 Fedor Gogolev <knsd@knsd.net>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error::{Error};
use std::fmt;

use base::{FromAscii};

/// An error returned when parsing a numeric value from a ascii string fails.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseIntError {
    kind: IntErrorKind
}

#[derive(Debug, Clone, PartialEq)]
enum IntErrorKind {
    Empty,
    InvalidDigit(u8),
    Overflow,
    Underflow,
    InvalidRadix(u8),
}

impl IntErrorKind {
    fn description(&self) -> &str {
        match self {
            &IntErrorKind::Empty => "cannot parse integer from empty string",
            &IntErrorKind::InvalidDigit(_) => "invalid digit found in string",
            &IntErrorKind::Overflow => "number too large to fit in target type",
            &IntErrorKind::Underflow => "number too small to fit in target type",
            &IntErrorKind::InvalidRadix(_) => "radix should be in 2..36 range",
        }
    }
}

impl fmt::Display for ParseIntError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.kind.description().fmt(f)
    }
}

impl Error for ParseIntError {
    fn description(&self) -> &str {
        self.kind.description()
    }
}

#[inline]
pub fn dec_to_digit(c: u8, radix: u8) -> Option<u8> {
    let val = match c {
        b'0' ... b'9' => c - b'0',
        b'a' ... b'z' => c - b'a' + 10,
        b'A' ... b'Z' => c - b'A' + 10,
        _ => return None,
    };
    if val < radix {
        Some(val)
    } else {
        None
    }
}

trait FromAsciiHelper: {
    fn signed() -> bool;
}

/// A trait to abstract the idea of creating a new instance of a numeric type from a
/// ascii string with given radix.
///
/// It's a near clone of standard `FromStr` trait.
pub trait FromAsciiRadix: Sized {
    /// The associated error which can be returned from parsing.
    type Err;

    /// Parses a ascii string `s` to return a value of this type.
    ///
    /// If parsing succeeds, return the value inside `Ok`, otherwise
    /// when the string is ill-formatted return an error specific to the
    /// inside `Err`. The error type is specific to implementation of the trait.
    ///
    /// # Examples
    ///
    /// Basic usage with `i32`, a type that implements `FromAsciiRadix`:
    ///
    /// ```
    /// use from_ascii::FromAsciiRadix;
    ///
    /// let s = b"FA";
    /// let x = i32::from_ascii_radix(s, 16).unwrap();
    ///
    /// assert_eq!(250, x);
    /// ```
    fn from_ascii_radix(s: &[u8], radix: u8) -> Result<Self, Self::Err>;
}

macro_rules! implement {
    ($t:ty, $signed: expr) => {
        impl FromAsciiHelper for $t {
            #[inline]
            fn signed() -> bool { $signed }
        }

        impl FromAsciiRadix for $t {
            type Err = ParseIntError;

            #[inline]
            fn from_ascii_radix(src: &[u8], radix: u8) -> Result<Self, Self::Err> {
                if radix < 2 || radix > 36 {
                    return Err(ParseIntError { kind: IntErrorKind::InvalidRadix(radix) })
                }

                let (is_positive, digits) = match src.get(0) {
                    Some(&b'+') => (true, &src[1..]),
                    Some(&b'-') if <$t>::signed() => (false, &src[1..]),
                    Some(_) => (true, src),
                    None => return Err(ParseIntError { kind: IntErrorKind::Empty }),
                };

                if digits.is_empty() {
                    return Err(ParseIntError { kind: IntErrorKind::Empty });
                }

                let mut result: $t = 0;

                if is_positive {
                    for &c in digits {
                        let x = match dec_to_digit(c, radix) {
                            Some(x) => x as $t,
                            None => return Err(ParseIntError { kind: IntErrorKind::InvalidDigit(c) } ),
                        };
                        result = match result.checked_mul(radix as $t) {
                            Some(result) => result,
                            None => return Err(ParseIntError { kind: IntErrorKind::Overflow }),
                        };
                        result = match result.checked_add(x) {
                            Some(result) => result,
                            None => return Err(ParseIntError { kind: IntErrorKind::Overflow }),
                        };
                    }
                } else {
                    for &c in digits {
                        let x = match dec_to_digit(c, radix) {
                            Some(x) => x as $t,
                            None => return Err(ParseIntError { kind: IntErrorKind::InvalidDigit(c) }),
                        };
                        result = match result.checked_mul(radix as $t) {
                            Some(result) => result,
                            None => return Err(ParseIntError { kind: IntErrorKind::Underflow }),
                        };
                        result = match result.checked_sub(x) {
                            Some(result) => result,
                            None => return Err(ParseIntError { kind: IntErrorKind::Underflow }),
                        };
                    }
                }
                Ok(result)
            }
        }

        impl FromAscii for $t {
            type Err = ParseIntError;

            #[inline]
            fn from_ascii(src: &[u8]) -> Result<Self, Self::Err> {
                <$t>::from_ascii_radix(src, 10)
            }
        }
    }
}

implement!(i8, true);
implement!(i16, true);
implement!(i32, true);
implement!(i64, true);
implement!(isize, true);
implement!(u8, false);
implement!(u16, false);
implement!(u32, false);
implement!(u64, false);
implement!(usize, false);

#[cfg(test)]
mod tests {
    use super::super::base::FromAscii;
    use super::FromAsciiRadix;

    #[test]
    fn test_from_ascii() {
        assert_eq!(i8::from_ascii(b"10"), Ok(10));
        assert_eq!(u8::from_ascii(b"10"), Ok(10));
        assert_eq!(i64::from_ascii(b"10"), Ok(10));
        assert_eq!(u64::from_ascii(b"10"), Ok(10));

        assert_eq!(i8::from_ascii(b"-10"), Ok(-10));
        assert_eq!(u8::from_ascii(b"-10").is_err(), true);
        assert_eq!(i64::from_ascii(b"-10"), Ok(-10));
        assert_eq!(u64::from_ascii(b"-10").is_err(), true);

        assert_eq!(i8::from_ascii(b"1000").is_err(), true);
        assert_eq!(u8::from_ascii(b"1000").is_err(), true);
        assert_eq!(i64::from_ascii(b"1000"), Ok(1000));
        assert_eq!(u64::from_ascii(b"1000"), Ok(1000));

        assert_eq!(i8::from_ascii(b"-1000").is_err(), true);
        assert_eq!(u8::from_ascii(b"-1000").is_err(), true);
        assert_eq!(i64::from_ascii(b"-1000"), Ok(-1000));
        assert_eq!(u64::from_ascii(b"-1000").is_err(), true);
    }

    #[test]
    fn test_from_ascii_radix() {
        assert_eq!(i8::from_ascii_radix(b"ff", 16).is_err(), true);
        assert_eq!(u8::from_ascii_radix(b"ff", 16), Ok(255));
        assert_eq!(i8::from_ascii_radix(b"FF", 16).is_err(), true);
        assert_eq!(u8::from_ascii_radix(b"FF", 16), Ok(255));
    }
}
