#[cfg(feature = "generic")]
extern crate generic_array;
#[cfg(feature = "generic")]
extern crate digest;

use core::hash::Hasher;


mod consts {
    pub const SIZE: usize = 16;
    pub const BASE: u32 = 0xFFFF;
}


#[derive(Copy, Clone)]
pub struct Bsd {
    state: u32,
}


impl Default for Bsd {
    fn default() -> Self {
        Bsd {
            state: 0,
        }
    }
}


impl Hasher for Bsd {
    #[inline]
    fn write(&mut self, input: &[u8]) {
        for &byte in input.iter() {
            // Rotate one bit right, add next byte an prevent overflow with mask
            self.state = (self.state >> 1) + ((self.state & 1) << (consts::SIZE - 1));
            self.state = (self.state + (byte as u32)) & consts::BASE;
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
