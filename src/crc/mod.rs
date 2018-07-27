//! [Cyclic Redundancy Check][1] implementations.
//!
//! [1]: https://en.wikipedia.org/wiki/CRC32

pub mod crc16;
pub mod crc32;
pub mod crc64;
pub mod crc8;

pub use self::crc16::Crc16;
pub use self::crc32::Crc32;
pub use self::crc32::Crc32c;
pub use self::crc64::Crc64;
pub use self::crc8::Crc8;
