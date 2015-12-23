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
