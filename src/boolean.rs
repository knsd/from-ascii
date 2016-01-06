use std::fmt;
use std::error::{Error};

use base::{FromAscii};

/// An error returned when parsing a `bool` from a ascii string fails.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseBoolError {
    kind: BoolErrorKind
}

#[derive(Debug, Clone, PartialEq)]
enum BoolErrorKind {
    BoolErrorKind
}

impl Error for ParseBoolError {
    fn description(&self) -> &str {
        "failed to parse bool"
    }
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
            _       => Err(ParseBoolError { kind: BoolErrorKind::BoolErrorKind }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::ParseBoolError;
    use super::super::base::FromAscii;

    #[test]
    fn test_parse_bool() {
        assert_eq!(bool::from_ascii(b"true"), Ok(true));
        assert_eq!(bool::from_ascii(b"false"), Ok(false));
        assert_eq!(bool::from_ascii(b""), Err(ParseBoolError::ParseBoolError));
        assert_eq!(bool::from_ascii(b"true "), Err(ParseBoolError::ParseBoolError));
        assert_eq!(bool::from_ascii(b"false "), Err(ParseBoolError::ParseBoolError));
    }
}
