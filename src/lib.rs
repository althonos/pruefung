#![crate_type= "lib"]
#![no_std]

extern crate byte_tools;
#[cfg(feature = "generic")]
extern crate generic_array;
#[cfg(feature = "generic")]
extern crate digest;


pub mod crc32;
pub mod crc32c;
pub mod fletcher16;
pub mod adler32;
