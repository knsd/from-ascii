use std::fmt;

pub trait FromAscii: Sized {
    type Err;

    fn from_ascii(s: &[u8]) -> Result<Self, Self::Err>;
}

/// An error returned when parsing a `bool` from a ascii string fails.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseBoolError {
    ParseBoolError
}

impl fmt::Display for ParseBoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "provided string was not `true` or `false`".fmt(f)
    }
}

impl FromAscii for bool {
    type Err = ParseBoolError;

    #[inline]
    fn from_ascii(s: &[u8]) -> Result<bool, ParseBoolError> {
        match s {
            b"true"  => Ok(true),
            b"false" => Ok(false),
            _       => Err(ParseBoolError::ParseBoolError),
        }
    }
}

trait FromStrHelper: {
    fn signed() -> bool;
}

pub enum ParseIntError {
    Empty,
    InvalidDigit,
    Overflow,
    Underflow,
}

#[inline]
pub fn dec_to_digit(c: u8) -> Option<u8> {
    let val = match c {
        b'0' ... b'9' => c - b'0',
        _ => return None,
    };
    Some(val)
}

macro_rules! from_bytes {
    ($src: expr, $t: ty) => ({
        if $src.is_empty() {
            return Err(ParseIntError::Empty);
        }

        let (is_positive, digits) = match $src[0] {
            b'+' => (true, &$src[1..]),
            b'-' if <$t>::signed() => (false, &$src[1..]),
            _ => (true, $src)
        };

        if digits.is_empty() {
            return Err(ParseIntError::Empty);
        }

        let mut result: $t = 0;

        if is_positive {
            for &c in digits {
                let x = match dec_to_digit(c) {
                    Some(x) => x as $t,
                    None => return Err(ParseIntError::InvalidDigit),
                };
                result = match result.checked_mul(10) {
                    Some(result) => result,
                    None => return Err(ParseIntError::Overflow),
                };
                result = match result.checked_add(x) {
                    Some(result) => result,
                    None => return Err(ParseIntError::Overflow),
                };
            }
        } else {
            for &c in digits {
                let x = match dec_to_digit(c) {
                    Some(x) => x as $t,
                    None => return Err(ParseIntError::InvalidDigit),
                };
                result = match result.checked_mul(10) {
                    Some(result) => result,
                    None => return Err(ParseIntError::Underflow),
                };
                result = match result.checked_sub(x) {
                    Some(result) => result,
                    None => return Err(ParseIntError::Underflow),
                };
            }
        }
        Ok(result)
    })
}


macro_rules! impl_helpers {
    ($t:ty, $signed: expr) => {
        impl FromStrHelper for $t {
            #[inline]
            fn signed() -> bool { $signed }
        }

        impl FromAscii for $t {
            type Err = ParseIntError;

            #[inline]
            fn from_ascii(src: &[u8]) -> Result<Self, Self::Err> {
                from_bytes!(src, $t)
            }
        }
    }
}

impl_helpers!(i8, true);
impl_helpers!(i16, true);
impl_helpers!(i32, true);
impl_helpers!(i64, true);
impl_helpers!(isize, true);
impl_helpers!(u8, false);
impl_helpers!(u16, false);
impl_helpers!(u32, false);
impl_helpers!(u64, false);
impl_helpers!(usize, false);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_bool() {
        assert_eq!(bool::from_ascii(b"true"), Ok(true));
        assert_eq!(bool::from_ascii(b"false"), Ok(false));
        assert_eq!(bool::from_ascii(b""), Err(ParseBoolError::ParseBoolError));
        assert_eq!(bool::from_ascii(b"true "), Err(ParseBoolError::ParseBoolError));
        assert_eq!(bool::from_ascii(b"false "), Err(ParseBoolError::ParseBoolError));
    }
}
