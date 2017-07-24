//! [Fowler-Noll-Vo][1] checksum implementations.
//!
//! # Nomenclature
//!
//! The hashers implemented in this module follow this naming convention:
//!
//!   * `FnvXX` is the [FNV-1][2] algorithm with a XXXX bits output
//!   * `FnvXXa` is the [FNV-1a][3] algorithm with a XXXX bits output
//!   * `FnvXXz` is the [FNV-0][4] algorithm with a XXXX bits output.
//!
//! where `XXXX` can be 32 or 64 bits.
//!
//! This convention tries to have a FNV API that follows the same logic as
//! the CRC API, as well as avoiding ugly snake case in struct names (Rust and I
//! both prefer `Fnv32z` over `Fnv0_32`)
//!
//! # Reference
//! 1. Fowler, Glenn, Kiem-Phong Vo, Landon Curt Noll, Donald Eastlake, and Tony Hansen.
//!    *Fowler-Noll-Vo Hash*. IETF Network Working Group, 7 June 2017.
//!    [Internet Draft](https://tools.ietf.org/html/draft-eastlake-fnv-03).
//! 2. Fowler-Noll-Vo online resources: http://www.isthe.com/chongo/tech/comp/fnv/
//!
//! [1]: https://en.wikipedia.org/wiki/Fowler–Noll–Vo_hash_function
//! [2]: https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function#FNV-1_hash
//! [3]: https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function#FNV-1a_hash
//! [4]: https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function#FNV-0_hash_.28deprecated.29

pub mod fnv32;
pub mod fnv64;

pub use self::fnv32::Fnv32;
pub use self::fnv32::Fnv32a;
pub use self::fnv32::Fnv32z;

pub use self::fnv64::Fnv64;
pub use self::fnv64::Fnv64a;
pub use self::fnv64::Fnv64z;
