//! [Adler32][1] checksum implementation.
//!
//! # Reference
//! 1. Deutsch, Peter, and Jean-Loup Gailly. *ZLIB Compressed Data Format Specification Version
//!    3.3*, 1996. [rfc:1950](https://tools.ietf.org/html/rfc1950).
//!
//!
//! [1]: https://en.wikipedia.org/wiki/Adler-32

#[cfg(feature = "generic")]
extern crate digest;
#[cfg(feature = "generic")]
extern crate generic_array;

use core::borrow::BorrowMut;

mod consts {
    pub const BASE: u32 = 65521;
    pub const NMAX: usize = 5552;
}

/// The Adler32 hasher.
#[derive(Copy, Clone)]
pub struct Adler32 {
    sum1: u32,
    sum2: u32,
}

impl Default for Adler32 {
    fn default() -> Self {
        Adler32 { sum1: 1, sum2: 0 }
    }
}

impl Hasher for Adler32 {
    #[inline]
    fn write(&mut self, input: &[u8]) {
        let mut byte_it = input.iter();
        let mut i: usize;
        loop {
            i = 0;
            // Read bytes by block of NMAX (max value before u16 overflow)
            for &byte in byte_it.borrow_mut().take(consts::NMAX) {
                self.sum1 += byte as u32;
                self.sum2 += self.sum1;
                i += 1;
            }
            // Reduce sums to u16
            self.sum1 %= consts::BASE;
            self.sum2 %= consts::BASE;
            // If the last block was read, stop
            if i < consts::NMAX {
                break;
            }
        }
    }

    #[inline]
    fn finish(&self) -> u64 {
        (((self.sum2 % consts::BASE) << 16) | self.sum1) as u64
    }
}

implement_digest!(Adler32, U32768, U4);

#[cfg(test)]
#[cfg(feature = "generic")]
mod tests {
    unit_test_no_data!(Adler32, 1);
    unit_test_part_data!(Adler32);
}
