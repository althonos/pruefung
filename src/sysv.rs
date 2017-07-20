extern crate byte_tools;
#[cfg(feature = "generic")]
extern crate generic_array;
#[cfg(feature = "generic")]
extern crate digest;


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


impl SysV {

    #[inline]
    pub fn hash(self) -> u16 {
        self.state as u16
    }

    #[inline]
    pub fn consume(&mut self, input: &[u8]) {
        for &byte in input.iter() {
            // Add the byte to the checksum modulo 0xFFFF
            self.state += byte as u32;
            self.state = (self.state & consts::BASE) + (self.state >> consts::SIZE);
        }
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
        self.consume(input);
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

    use digest::Digest;
    use digest::Input;
    use digest::FixedOutput;
    use generic_array::GenericArray;

    #[test]
    fn no_data() {
        let sysv = super::SysV::new();
        let output: [u8; 2] = [0, 0];

        assert!(sysv.hash() == 0);
        assert!(sysv.fixed_result() == GenericArray::clone_from_slice(&output));
    }

    #[test]
    fn single_byte() {
        let mut sysv = super::SysV::new();
        let output: [u8; 2] = [0, 'a' as u8];

        sysv.consume("a".as_bytes());

        assert!(sysv.hash() == 'a' as u16);
        assert!(sysv.fixed_result() ==  GenericArray::clone_from_slice(&output))
    }

    #[test]
    fn multi_part_data() {
        let mut sysv1 = super::SysV::new();
        let mut sysv2 = super::SysV::new();
        let data = b"abcdef";

        sysv1.process(&data[..3]);
        sysv1.process(&data[3..]);
        sysv2.process(&data[..]);

        assert!(sysv1.hash() == sysv2.hash());
    }
}
