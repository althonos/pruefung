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
pub struct SysV {
    state: u32,
}


impl Default for SysV {
    fn default() -> Self {
        SysV {
            state: 0,
        }
    }
}

impl Hasher for SysV {
    #[inline]
    fn write(&mut self, input: &[u8]) {
        for &byte in input.iter() {
            // Add the byte to the checksum modulo 0xFFFF
            self.state += byte as u32;
            self.state = (self.state & consts::BASE) + (self.state >> consts::SIZE);
        }
    }
    #[inline]
    fn finish(&self) -> u64 {
        self.state as u64
    }
}


#[cfg(feature = "generic")]
impl digest::BlockInput for SysV {
    type BlockSize = generic_array::typenum::U512;
}

#[cfg(feature = "generic")]
impl digest::Input for SysV {
    #[inline]
    fn process(&mut self, input: &[u8]) {
        self.write(input);
    }
}

#[cfg(feature = "generic")]
impl digest::FixedOutput for SysV {
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
    unit_test_no_data!(SysV, 0);
    unit_test_part_data!(SysV);
    unit_test_single_byte!(SysV, b"a", b'a');
}
