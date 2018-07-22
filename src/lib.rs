//! *Checksums in pure Rust.*
//!
//! [![TravisCI](https://img.shields.io/travis/althonos/pruefung/master.svg?maxAge=600&style=flat-square)](https://travis-ci.org/althonos/pruefung/branches)
//! [![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=86400&style=flat-square)](https://github.com/althonos/pruefung)
//! [![Crate](https://img.shields.io/crates/v/pruefung.svg?maxAge=86400&style=flat-square)](https://crates.io/crates/pruefung)
//! [![Documentation](https://img.shields.io/badge/docs-latest-4d76ae.svg?maxAge=86400&style=flat-square)](https://docs.rs/pruefung)
//! [![CargoMake](https://img.shields.io/badge/built%20with-cargo--make-yellow.svg?maxAge=86400&style=flat-square)](https://sagiegurari.github.io/cargo-make)
//! [![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=86400&style=flat-square)](http://keepachangelog.com/)
//! [![SayThanks](https://img.shields.io/badge/say-thanks!-1EAEDB.svg?maxAge=86400&style=flat-square)](https://saythanks.io/to/althonos)

#![crate_type = "lib"]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "generic")]
extern crate digest;
#[cfg(feature = "generic")]
extern crate generic_array;
#[cfg(feature = "generic")]
pub use digest::Digest;

#[cfg(feature = "std")]
use std as core;

pub use core::hash::Hasher;

#[macro_use]
mod macros;

#[cfg(feature = "adler32")]
pub mod adler32;
#[cfg(feature = "bsd")]
pub mod bsd;
#[cfg(feature = "crc")]
pub mod crc;
#[cfg(feature = "fletcher16")]
pub mod fletcher16;
#[cfg(feature = "fnv")]
pub mod fnv;
#[cfg(feature = "unix")]
pub mod unix;
#[cfg(feature = "sysv")]
pub mod sysv;
