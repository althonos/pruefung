//! [FNV-32][1] checksum implementation.
//!
//! [1]: https://en.wikipedia.org/wiki/Fowler–Noll–Vo_hash_function

#[cfg(feature = "generic")]
extern crate digest;
#[cfg(feature = "generic")]
extern crate generic_array;

mod consts {
    pub const FNV_OFFSET: u32 = 0x811c9dc5;
    pub const FNV_PRIME: u32 = 16777619;
}

pub mod fnv32 {

    use core::hash::Hasher;
    #[cfg(feature = "generic")]
    use digest;
    #[cfg(feature = "generic")]
    use generic_array;

    /// The FNV1-32 hasher.
    #[derive(Copy, Clone, Debug)]
    pub struct Fnv32 {
        state: u32,
    }

    impl Default for Fnv32 {
        fn default() -> Self {
            Fnv32 {
                state: super::consts::FNV_OFFSET,
            }
        }
    }

    impl Hasher for Fnv32 {
        #[inline]
        fn write(&mut self, input: &[u8]) {
            for &byte in input.iter() {
                self.state = self.state.wrapping_mul(super::consts::FNV_PRIME);
                self.state ^= byte as u32;
            }
        }
        #[inline]
        fn finish(&self) -> u64 {
            self.state as u64
        }
    }

    implement_digest!(Fnv32, U2048, U4);

    #[cfg(test)]
    #[cfg(feature = "generic")]
    mod tests {
        unit_test_no_data!(Fnv32, super::super::consts::FNV_OFFSET);
        unit_test_part_data!(Fnv32);
    }

}

pub mod fnv32a {

    use core::hash::Hasher;
    #[cfg(feature = "generic")]
    use digest;
    #[cfg(feature = "generic")]
    use generic_array;

    /// The FNV1a-32 hasher.
    #[derive(Copy, Clone, Debug)]
    pub struct Fnv32a {
        state: u32,
    }

    impl Default for Fnv32a {
        fn default() -> Self {
            Fnv32a {
                state: super::consts::FNV_OFFSET,
            }
        }
    }

    impl Hasher for Fnv32a {
        #[inline]
        fn write(&mut self, input: &[u8]) {
            for &byte in input.iter() {
                self.state ^= byte as u32;
                self.state = self.state.wrapping_mul(super::consts::FNV_PRIME);
            }
        }
        #[inline]
        fn finish(&self) -> u64 {
            self.state as u64
        }
    }

    implement_digest!(Fnv32a, U2048, U4);

    #[cfg(test)]
    #[cfg(feature = "generic")]
    mod tests {
        unit_test_no_data!(Fnv32a, super::super::consts::FNV_OFFSET);
        unit_test_part_data!(Fnv32a);
    }

}

pub mod fnv32z {

    use core::hash::Hasher;
    #[cfg(feature = "generic")]
    use digest;
    #[cfg(feature = "generic")]
    use generic_array;

    /// The FNV0-32 hasher.
    #[derive(Copy, Clone, Debug)]
    pub struct Fnv32z {
        state: u32,
    }

    impl Default for Fnv32z {
        fn default() -> Self {
            Fnv32z { state: 0 }
        }
    }

    impl Hasher for Fnv32z {
        #[inline]
        fn write(&mut self, input: &[u8]) {
            for &byte in input.iter() {
                self.state = self.state.wrapping_mul(super::consts::FNV_PRIME);
                self.state ^= byte as u32;
            }
        }
        #[inline]
        fn finish(&self) -> u64 {
            self.state as u64
        }
    }

    implement_digest!(Fnv32z, U2048, U4);

    #[cfg(test)]
    #[cfg(feature = "generic")]
    mod tests {
        unit_test_no_data!(Fnv32z, 0);
        unit_test_part_data!(Fnv32z);
    }

}

pub use self::fnv32::Fnv32;
pub use self::fnv32a::Fnv32a;
pub use self::fnv32z::Fnv32z;
