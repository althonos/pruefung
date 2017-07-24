//! *Checksums in pure Rust.*
//!
//! [![TravisCI](https://img.shields.io/travis/althonos/pruefung/master.svg?maxAge=600&style=flat-square)](https://travis-ci.org/althonos/pruefung/branches)
//! [![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=86400&style=flat-square)](https://github.com/althonos/pruefung)
//! [![Crate](https://img.shields.io/crates/v/pruefung.svg?maxAge=86400&style=flat-square)](https://crates.io/crates/pruefung)
//! [![Documentation](https://img.shields.io/badge/docs-latest-4d76ae.svg?maxAge=86400&style=flat-square)](https://docs.rs/pruefung)
//! [![CargoMake](https://img.shields.io/badge/built%20with-cargo--make-yellow.svg?maxAge=86400&style=flat-square)](https://sagiegurari.github.io/cargo-make)
//! [![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=86400&style=flat-square)](http://keepachangelog.com/)
//! [![SayThanks](https://img.shields.io/badge/say-thanks!-1EAEDB.svg?maxAge=86400&style=flat-square)](https://saythanks.io/to/althonos)

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
pub mod crc;
pub mod fletcher16;
pub mod unix;
pub mod sysv;
