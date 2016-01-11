from-ascii [![Build Status](https://travis-ci.org/knsd/from-ascii.svg?branch=master)](https://travis-ci.org/knsd/from-ascii)
==========

This crate provides two traits: `FromAscii` which creates instances from ascii string (`&[u8]`) and `FromAsciiRadix` wich creates only integral values with given radix.

The documentation is located at http://knsd.github.io/from-ascii/.

Usage example:

```rust

extern crate from_ascii;

use from_ascii::{FromAscii, FromAsciiRadix};

fn main() {
    println!("{:?}", f64::from_ascii(b"123.456"));
    println!("{:?}", i16::from_ascii_radix(b"FF", 16));
}

```

### License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
