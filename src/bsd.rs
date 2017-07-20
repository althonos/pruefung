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
pub struct BSD {
    state: u32,
}


impl Default for BSD {
    fn default() -> Self {
        BSD {
            state: 0,
        }
    }
}


impl Hasher for BSD {
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
impl digest::BlockInput for BSD {
    type BlockSize = generic_array::typenum::U1024;
}

#[cfg(feature = "generic")]
impl digest::Input for BSD {
    #[inline]
    fn process(&mut self, input: &[u8]) {
        self.write(input);
    }
}

#[cfg(feature = "generic")]
impl digest::FixedOutput for BSD {
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

    use core::hash::Hasher;
    use digest::Digest;
    use digest::Input;
    use digest::FixedOutput;
    use generic_array::GenericArray;

    #[test]
    fn no_data() {
        let bsd = super::BSD::new();
        let output: [u8; 2] = [0, 0];

        assert!(bsd.finish() == 0);
        assert!(bsd.fixed_result() == GenericArray::clone_from_slice(&output));
    }

    #[test]
    fn single_byte() {
        let mut bsd = super::BSD::new();
        let output: [u8; 2] = [0, 'a' as u8];

        bsd.write("a".as_bytes());

        assert!(bsd.finish() == 'a' as u64);
        assert!(bsd.fixed_result() ==  GenericArray::clone_from_slice(&output))
    }

    #[test]
    fn multi_part_data() {
        let mut bsd1 = super::BSD::new();
        let mut bsd2 = super::BSD::new();
        let data = b"abcdef";

        bsd1.process(&data[..3]);
        bsd1.process(&data[3..]);
        bsd2.process(&data[..]);

        assert!(bsd1.finish() == bsd2.finish());
    }
}
