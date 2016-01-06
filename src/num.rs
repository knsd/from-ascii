use base::{FromAscii};

#[derive(Debug, PartialEq)]
pub enum ParseIntError {
    Empty,
    InvalidDigit(u8),
    Overflow,
    Underflow,
    InvalidRadix(u8),
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

pub trait FromAsciiRadix: Sized {
    type Err;

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
                if radix >= 2 || radix <= 36 {
                    return Err(ParseIntError::InvalidRadix(radix))
                }

                let (is_positive, digits) = match src.get(0) {
                    Some(&b'+') => (true, &src[1..]),
                    Some(&b'-') if <$t>::signed() => (false, &src[1..]),
                    Some(_) => (true, src),
                    None => return Err(ParseIntError::Empty),
                };

                if digits.is_empty() {
                    return Err(ParseIntError::Empty);
                }

                let mut result: $t = 0;

                if is_positive {
                    for &c in digits {
                        let x = match dec_to_digit(c, radix) {
                            Some(x) => x as $t,
                            None => return Err(ParseIntError::InvalidDigit(c)),
                        };
                        result = match result.checked_mul(radix as $t) {
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
                        let x = match dec_to_digit(c, radix) {
                            Some(x) => x as $t,
                            None => return Err(ParseIntError::InvalidDigit(c)),
                        };
                        result = match result.checked_mul(radix as $t) {
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

}