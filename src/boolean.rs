// Copyright (c) 2016 Fedor Gogolev <knsd@knsd.net>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
    use super::super::base::FromAscii;

    #[test]
    fn test_parse_bool() {
        assert_eq!(bool::from_ascii(b"true"), Ok(true));
        assert_eq!(bool::from_ascii(b"false"), Ok(false));
        assert!(bool::from_ascii(b"").is_err());
        assert!(bool::from_ascii(b"true ").is_err());
        assert!(bool::from_ascii(b" false").is_err());
    }
}
