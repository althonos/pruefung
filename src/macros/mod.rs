#[macro_use]
mod test;


#[macro_export]
macro_rules! implement_digest {


    ($Hasher:ident, $BlockSize:ident, $OutputSize:ident) => (


        #[cfg(feature = "generic")]
        impl digest::BlockInput for $Hasher {
            type BlockSize = generic_array::typenum::$BlockSize;
        }

        #[cfg(feature = "generic")]
        impl digest::Input for $Hasher {
            #[inline]
            fn process(&mut self, input: &[u8]) {
                self.write(input)
            }
        }

        #[cfg(feature = "generic")]
        impl digest::FixedOutput for $Hasher {

            type OutputSize = generic_array::typenum::$OutputSize;

            #[inline]
            fn fixed_result(self) -> generic_array::GenericArray<u8, Self::OutputSize> {

                use generic_array::typenum::Unsigned;

                let mut array = generic_array::GenericArray::default();
                let mut out = self.finish();
                let size = Self::OutputSize::to_usize();

                for i in 0..size {
                    array[size-i-1] = (out & u8::max_value() as u64) as u8;
                    out >>= 8;
                }

                array
            }
        }





    )


}
