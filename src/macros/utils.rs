//! Utility macros.

/// Implement [`digest::Digest`][1] for a struct implementing [`hash::Hasher`][2].
///
/// [1]: https://docs.rs/digest/trait.Digest.html
/// [2]: https://doc.rust-lang.org/core/hash/trait.Hasher.html
#[allow(unused)]
macro_rules! implement_digest {
    ($Hasher:ident, $BlockSize:ident, $OutputSize:ident) => {
        #[cfg(feature = "generic")]
        impl digest::BlockInput for $Hasher {
            type BlockSize = digest::generic_array::typenum::$BlockSize;
        }

        #[cfg(feature = "generic")]
        impl digest::Update for $Hasher {
            #[inline]
            fn update(&mut self, data: impl AsRef<[u8]>) {
                self.write(data.as_ref());
            }
        }

        #[cfg(feature = "generic")]
        impl digest::FixedOutput for $Hasher {
            type OutputSize = digest::generic_array::typenum::$OutputSize;

            #[inline]
            fn finalize_into(self, out: &mut generic_array::GenericArray<u8, Self::OutputSize>) {
                use generic_array::typenum::Unsigned;
                let mut data = self.finish();
                let size = Self::OutputSize::to_usize();
                for i in 0..size {
                    out[size - i - 1] = (data & u8::max_value() as u64) as u8;
                    data >>= 8;
                }
            }

            #[inline]
            fn finalize_into_reset(&mut self, out: &mut generic_array::GenericArray<u8, Self::OutputSize>) {
                use generic_array::typenum::Unsigned;
                let mut data = self.finish();
                let size = Self::OutputSize::to_usize();
                for i in 0..size {
                    out[size - i - 1] = (data & u8::max_value() as u64) as u8;
                    data >>= 8;
                }
                *self = Self::default();
            }
        }

        #[cfg(feature = "generic")]
        impl digest::Reset for $Hasher {
            #[inline]
            fn reset(&mut self) {
                *self = Self::default();
            }
        }
    };
}
