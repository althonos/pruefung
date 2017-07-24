//! [Cyclic Redundancy Check][1] implementations.
//!
//! [1]: https://en.wikipedia.org/wiki/CRC32

pub mod crc32;

pub use self::crc32::Crc32;
pub use self::crc32::Crc32c;
