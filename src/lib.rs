#![crate_type= "lib"]
#![no_std]

extern crate byte_tools;
#[cfg(feature = "generic")]
extern crate generic_array;
#[cfg(feature = "generic")]
extern crate digest;


pub mod crc32;
