pub trait FromAscii: Sized {
    type Err;

    fn from_ascii(s: &[u8]) -> Result<Self, Self::Err>;
}