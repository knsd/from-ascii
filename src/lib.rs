// Copyright (c) 2016 Fedor Gogolev <knsd@knsd.net>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
