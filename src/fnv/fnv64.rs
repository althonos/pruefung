//! [FNV-64][1] checksum implementation.
//!
//! [1]: https://en.wikipedia.org/wiki/Fowler–Noll–Vo_hash_function

#[cfg(feature = "generic")]
extern crate digest;
#[cfg(feature = "generic")]
extern crate generic_array;

mod consts {
    pub const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    pub const FNV_PRIME: u64 = 1099511628211;
}

pub mod fnv64 {

    use core::hash::Hasher;
    #[cfg(feature = "generic")]
    use digest;
    #[cfg(feature = "generic")]
    use generic_array;

    /// The FNV1-64 hasher.
    #[derive(Copy, Clone)]
    pub struct Fnv64 {
        state: u64,
    }

    impl Default for Fnv64 {
        fn default() -> Self {
            Fnv64 {
                state: super::consts::FNV_OFFSET,
            }
        }
    }

    impl Hasher for Fnv64 {
        #[inline]
        fn write(&mut self, input: &[u8]) {
            for &byte in input.iter() {
                self.state = self.state.wrapping_mul(super::consts::FNV_PRIME);
                self.state ^= byte as u64;
            }
        }
        #[inline]
        fn finish(&self) -> u64 {
            self.state
        }
    }

    implement_digest!(Fnv64, U2048, U8);

    #[cfg(test)]
    #[cfg(feature = "generic")]
    mod tests {
        unit_test_no_data!(Fnv64, super::super::consts::FNV_OFFSET);
        unit_test_part_data!(Fnv64);
    }

}

pub mod fnv64a {

    use core::hash::Hasher;
    #[cfg(feature = "generic")]
    use digest;
    #[cfg(feature = "generic")]
    use generic_array;

    /// The FNV1a-64 hasher.
    #[derive(Copy, Clone)]
    pub struct Fnv64a {
        state: u64,
    }

    impl Default for Fnv64a {
        fn default() -> Self {
            Fnv64a {
                state: super::consts::FNV_OFFSET,
            }
        }
    }

    impl Hasher for Fnv64a {
        #[inline]
        fn write(&mut self, input: &[u8]) {
            for &byte in input.iter() {
                self.state ^= byte as u64;
                self.state = self.state.wrapping_mul(super::consts::FNV_PRIME);
            }
        }
        #[inline]
        fn finish(&self) -> u64 {
            self.state
        }
    }

    implement_digest!(Fnv64a, U2048, U8);

    #[cfg(test)]
    #[cfg(feature = "generic")]
    mod tests {
        unit_test_no_data!(Fnv64a, super::super::consts::FNV_OFFSET);
        unit_test_part_data!(Fnv64a);
    }

}

pub mod fnv64z {

    use core::hash::Hasher;
    #[cfg(feature = "generic")]
    use digest;
    #[cfg(feature = "generic")]
    use generic_array;

    /// The FNV0-64 hasher.
    #[derive(Copy, Clone)]
    pub struct Fnv64z {
        state: u64,
    }

    impl Default for Fnv64z {
        fn default() -> Self {
            Fnv64z { state: 0 }
        }
    }

    impl Hasher for Fnv64z {
        #[inline]
        fn write(&mut self, input: &[u8]) {
            for &byte in input.iter() {
                self.state = self.state.wrapping_mul(super::consts::FNV_PRIME);
                self.state ^= byte as u64;
            }
        }
        #[inline]
        fn finish(&self) -> u64 {
            self.state as u64
        }
    }

    implement_digest!(Fnv64z, U2048, U8);

    #[cfg(test)]
    #[cfg(feature = "generic")]
    mod tests {
        unit_test_no_data!(Fnv64z, 0);
        unit_test_part_data!(Fnv64z);
    }

}

pub use self::fnv64::Fnv64;
pub use self::fnv64a::Fnv64a;
pub use self::fnv64z::Fnv64z;
