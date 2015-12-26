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

#[inline]
pub fn dec_to_digit(c: u8) -> Option<u8> {
    let val = match c {
        b'0' ... b'9' => c - b'0',
        _ => return None,
    };
    Some(val)
}

#[inline]
pub fn from_bytes(src: &[u8]) -> Option<i64> {
    if src.is_empty() {
        return None;
    }

    let (is_positive, digits) = match src[0] {
        b'+' => (true, &src[1..]),
        b'-' => (false, &src[1..]),
        _ => (true, src)
    };

    if digits.is_empty() {
        return None;
    }

    let mut result = 0;

    if is_positive {
        for &c in digits {
            result = result * 10 + match dec_to_digit(c) {
                Some(x) => x as i64,
                None => return None,
            };
        }
    } else {
        for &c in digits {
            result = result * 10 - match dec_to_digit(c) {
                Some(x) => x as i64,
                None => return None,
            };
        }
    }
    Some(result)
}

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
