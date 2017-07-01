// Copyright 2015-2017 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Efficient large, fixed-size big integers and hashes.

#![cfg_attr(asm_available, feature(asm))]

extern crate rand;
extern crate rustc_serialize;
extern crate libc;
extern crate serde;
extern crate byteorder;
#[macro_use]
extern crate heapsize;
#[cfg(test)]
extern crate serde_json;

pub mod hash;
pub mod uint;

pub mod prelude {
    pub use uint::*;
    pub use hash::*;
}