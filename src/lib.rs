//!
//! This crate provides two traits: `FromAscii` which creates instances from ascii string (`&[u8]`) and `FromAsciiRadix` wich creates only integral values with given radix.
//!
//! Usage example:
//!
//! ```rust
//! extern crate from_ascii;
//!
//! use from_ascii::{FromAscii, FromAsciiRadix};
//!
//!
//! fn main() {
//!     println!("{:?}", f64::from_ascii(b"123.456"));
//!     println!("{:?}", i16::from_ascii_radix(b"FF", 16));
//! }
//! ```

mod base;
mod boolean;
mod num;
mod flt;

pub use base::{FromAscii};
pub use boolean::{ParseBoolError};
pub use num::{ParseIntError, FromAsciiRadix};
pub use flt::{ParseFloatError};
