/// A trait to abstract the idea of creating a new instance of a type from a
/// ascii string.
///
/// It's a near clone of standard `FromStr` trait.
pub trait FromAscii: Sized {
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
    /// Basic usage with `i32`, a type that implements `FromAscii`:
    ///
    /// ```
    /// use from_ascii::FromAscii;
    ///
    /// let s = b"5";
    /// let x = i32::from_ascii(s).unwrap();
    ///
    /// assert_eq!(5, x);
    /// ```
    fn from_ascii(s: &[u8]) -> Result<Self, Self::Err>;
}
