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
        SysV { state: 0 }
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

implement_digest!(SysV, U512, U2);



#[cfg(test)]
#[cfg(feature = "generic")]
mod tests {
    unit_test_no_data!(SysV, 0);
    unit_test_part_data!(SysV);
    unit_test_single_byte!(SysV, b"a", b'a');
}
