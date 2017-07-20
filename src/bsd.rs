extern crate byte_tools;
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


#[cfg(feature = "generic")]
impl digest::BlockInput for Bsd {
    type BlockSize = generic_array::typenum::U1024;
}

#[cfg(feature = "generic")]
impl digest::Input for Bsd {
    #[inline]
    fn process(&mut self, input: &[u8]) {
        self.write(input);
    }
}

#[cfg(feature = "generic")]
impl digest::FixedOutput for Bsd {
    type OutputSize = generic_array::typenum::U2;

    #[inline]
    fn fixed_result(self) -> generic_array::GenericArray<u8, Self::OutputSize> {
        let mut out = generic_array::GenericArray::default();
        out[0] = (self.state >> 8) as u8;
        out[1] = (self.state & 0xFF) as u8;
        out
    }
}


#[cfg(test)]
#[cfg(feature = "generic")]
mod tests {
    unit_test_no_data!(Bsd, 0);
    unit_test_part_data!(Bsd);
    unit_test_single_byte!(Bsd, b"a", b'a');
}
