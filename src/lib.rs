pub trait FromAscii: Sized {
    type Err;

    fn from_str(s: &[u8]) -> Result<Self, Self::Err>;
}
