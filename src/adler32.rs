extern crate byte_tools;
#[cfg(feature = "generic")]
extern crate generic_array;
#[cfg(feature = "generic")]
extern crate digest;

use core::borrow::BorrowMut;


mod consts {
    pub const BASE: u32 = 65521;
    pub const NMAX: usize = 5552;
}


#[derive(Copy, Clone)]
pub struct Adler32 {
    sum1: u32,
    sum2: u32,
}


impl Default for Adler32 {
    fn default() -> Self {
        Adler32 {
            sum1: 1,
            sum2: 0,
        }
    }
}


impl Adler32 {
    #[inline]
    fn finalize(self) -> [u32; 2]{
        [
            self.sum1 % consts::BASE,
            self.sum2 % consts::BASE,
        ]
    }

    #[inline]
    pub fn hash(self) -> u32 {
        let sums = self.finalize();
        (sums[1] << 16) | sums[0]
    }

    #[inline]
    pub fn consume(&mut self, input: &[u8]) {

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
            if i < consts::NMAX {break;}
        }
    }
}


#[cfg(feature = "generic")]
impl digest::BlockInput for Adler32 {
    type BlockSize = generic_array::typenum::U32768;
}

#[cfg(feature = "generic")]
impl digest::Input for Adler32 {
    #[inline]
    fn process(&mut self, input: &[u8]) {
        self.consume(input);
    }
}

#[cfg(feature = "generic")]
impl digest::FixedOutput for Adler32 {
    type OutputSize = generic_array::typenum::U4;

    #[inline]
    fn fixed_result(self) -> generic_array::GenericArray<u8, Self::OutputSize> {
        let mut out = generic_array::GenericArray::default();
        byte_tools::write_u32_be(&mut out, self.hash());
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
        let adler = super::Adler32::new();
        let output: [u8; 4] = [0, 0, 0, 1];

        assert!(adler.hash() == 1);
        assert!(adler.fixed_result() == GenericArray::clone_from_slice(&output));
    }

    #[test]
    fn multi_part_data() {
        let mut adler1 = super::Adler32::new();
        let mut adler2 = super::Adler32::new();

        let data = b"abcdef";

        adler1.process(&data[..3]);
        adler1.process(&data[3..]);
        adler2.process(&data[..]);

        assert!(adler1.hash() == adler2.hash());
    }
}
