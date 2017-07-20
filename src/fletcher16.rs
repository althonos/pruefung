#[cfg(feature = "generic")]
extern crate generic_array;
#[cfg(feature = "generic")]
extern crate digest;

use core::hash::Hasher;
use core::borrow::BorrowMut;


mod consts {
    pub const BASE: u16 = 0xFF;
    pub const NMAX: usize = 20;
}


#[derive(Copy, Clone)]
pub struct Fletcher16 {
    sum1: u16,
    sum2: u16,
}


impl Default for Fletcher16 {
    fn default() -> Self {
        Fletcher16 { sum1: 0, sum2: 0 }
    }
}


impl Fletcher16 {
    #[inline]
    fn finalize(&self) -> [u16; 2] {
        [
            (self.sum1 & consts::BASE) + (self.sum1 >> 8),
            (self.sum2 & consts::BASE) + (self.sum2 >> 8),
        ]
    }
}


impl Hasher for Fletcher16 {
    #[inline]
    fn write(&mut self, input: &[u8]) {
        let mut byte_it = input.iter();
        let mut i: usize;

        loop {

            i = 0;
            // Read bytes by block of 20 (max value before u8 overflow)
            for &byte in byte_it.borrow_mut().take(consts::NMAX) {
                self.sum1 += byte as u16;
                self.sum2 += self.sum1;
                i += 1;
            }

            // Reduce sums to u8
            self.sum1 = (self.sum1 & consts::BASE) + (self.sum1 >> 8);
            self.sum2 = (self.sum2 & consts::BASE) + (self.sum2 >> 8);

            // If the last block was read, stop
            if i < consts::NMAX {
                break;
            }
        }
    }

    #[inline]
    fn finish(&self) -> u64 {
        let sums = self.finalize();
        ((sums[1] << 8) | sums[0]) as u64
    }
}

implement_digest!(Fletcher16, U1024, U2);

#[cfg(test)]
#[cfg(feature = "generic")]
mod tests {
    unit_test_no_data!(Fletcher16, 0);
    unit_test_part_data!(Fletcher16);
}
