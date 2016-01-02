use base::{FromAscii};

trait FromStrHelper: {
    fn signed() -> bool;
}

#[derive(Debug, PartialEq)]
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
                if src.is_empty() {
                    return Err(ParseIntError::Empty);
                }

                let (is_positive, digits) = match src[0] {
                    b'+' => (true, &src[1..]),
                    b'-' if <$t>::signed() => (false, &src[1..]),
                    _ => (true, src)
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
mod tests {
    use super::super::base::FromAscii;

}