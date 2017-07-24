//! Fowler-Noll-Vo checksum implementations.
//!
//! The hashers implemented in this module follow this naming convention:
//!   * `FnvXXXX` is the FNV-1 algorithm with a XXXX bits output
//!   * `FnvXXXXa` is the FNV-1a algorithm with a XXXX bits output
//!   * `FnvXXXXz` is the FNV-0 algorithm with a XXXX bits output.
//!
//! This convention tries to have a FNV API that follows the same logic as
//! the CRC API, as well as avoiding ugly snake case in struct names (Rust and I
//! both prefer `Fnv32z` over `Fnv0_32`)

pub mod fnv32;

pub use self::fnv32::Fnv32;
pub use self::fnv32::Fnv32a;
pub use self::fnv32::Fnv32z;
