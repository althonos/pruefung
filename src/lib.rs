#![crate_type= "lib"]
#![no_std]

#[cfg(feature = "generic")]
extern crate generic_array;
#[cfg(feature = "generic")]
extern crate digest;

#[macro_use]
mod macros;

pub mod adler32;
pub mod bsd;
pub mod crc32;
pub mod fletcher16;
pub mod unix;
pub mod sysv;
