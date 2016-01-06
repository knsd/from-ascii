/// A trait to abstract the idea of creating a new instance of a type from a
/// ascii string. It's a near clone of standard `FromStr` trait.
pub trait FromAscii: Sized {
    type Err;

    fn from_ascii(s: &[u8]) -> Result<Self, Self::Err>;
}
