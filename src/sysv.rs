//! [SysV][1] checksum implementation.
//!
//! Known in UNIX as the `sum -s` command.
//!
//! [1]: https://en.wikipedia.org/wiki/SYSV_checksum

#[cfg(feature = "generic")]
extern crate generic_array;
#[cfg(feature = "generic")]
extern crate digest;

use core::hash::Hasher;
use core::ops::Rem;


/// The SysV hasher.
#[derive(Copy, Clone)]
pub struct SysV {
    state: u32,
}


impl Default for SysV {
    fn default() -> Self {
        SysV { state: 0 }
    }
}

impl Hasher for SysV {
    #[inline]
    fn write(&mut self, input: &[u8]) {
        for &byte in input.iter() {
            // Add the byte to the checksum
            self.state = self.state.wrapping_add(byte as u32);
        }
    }
    #[inline]
    fn finish(&self) -> u64 {
        self.state.rem(u16::max_value() as u32) as u64
    }
}


implement_digest!(SysV, U512, U2);


#[cfg(test)]
#[cfg(feature = "generic")]
mod tests {
    unit_test_no_data!(SysV, 0);
    unit_test_part_data!(SysV);
    unit_test_single_byte!(SysV, b"a", b'a');
}
