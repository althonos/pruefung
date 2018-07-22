//! [BSD][1] checksum implementation.
//!
//! Known in UNIX as the `sum` command.
//!
//! [1]: https://en.wikipedia.org/wiki/BSD_checksum

#[cfg(feature = "generic")]
extern crate digest;
#[cfg(feature = "generic")]
extern crate generic_array;

use core::hash::Hasher;

/// The BSD hasher.
#[derive(Copy, Clone)]
pub struct Bsd {
    state: u16,
}

impl Default for Bsd {
    fn default() -> Self {
        Bsd { state: 0 }
    }
}

impl Hasher for Bsd {
    #[inline]
    fn write(&mut self, input: &[u8]) {
        for &byte in input.iter() {
            // Rotate one bit right, add next byte and prevent
            self.state = self.state.rotate_right(1).wrapping_add(byte as u16);
        }
    }
    #[inline]
    fn finish(&self) -> u64 {
        self.state as u64
    }
}

implement_digest!(Bsd, U1024, U2);

#[cfg(test)]
#[cfg(feature = "generic")]
mod tests {
    unit_test_no_data!(Bsd, 0);
    unit_test_part_data!(Bsd);
    unit_test_single_byte!(Bsd, b"a", b'a');
}
