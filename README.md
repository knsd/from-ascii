from-ascii [![Build Status](https://travis-ci.org/knsd/from-ascii.svg?branch=master)](https://travis-ci.org/knsd/from-ascii)
==========

This crate provides two traits: `FromAscii` which creates instances from ascii string (`&[u8]`) and `FromAsciiRadix` wich creates only integral values with given radix.

Usage example:
```

extern crate from_ascii;

use from_ascii::{FromAscii, FromAsciiRadix};

fn main() {
    println!("{}", f64::from_ascii(b"123.456"));
    println!("{}", i16::from_ascii_radix(b"FF", 16));
}

```
