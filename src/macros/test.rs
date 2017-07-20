
#[macro_export]
macro_rules! output_array {

    ($Hasher:ident, $output:expr, $array_name:ident) => (

        use digest::FixedOutput;
        use generic_array::typenum::Unsigned;

        type OutputSize = <super::$Hasher as FixedOutput>::OutputSize;
        let size = OutputSize::to_usize();

        let mut output = $output;
        let mut $array_name = GenericArray::<u8, OutputSize>::default();
        for i in 0..size {
            $array_name[size-i-1] = ((output as u64) & u8::max_value() as u64) as u8;
            output >>= 8;
        }

    )


}




#[macro_export]
macro_rules! unit_test_no_data {
    ($Hasher:ident, $output:expr) => (

        #[test]
        fn no_data() {

            use digest::Digest;
            use core::hash::Hasher;
            use generic_array::GenericArray;

            // Create a hasher
            let hasher = super::$Hasher::new();

            // Check the hasher has the good empty output
            assert!(hasher.finish() == $output as u64);

            // Create a byte array with given output
            output_array!($Hasher, $output, array);

            // Check the output array is good
            assert!(hasher.fixed_result() == array);
        }

    )
}


#[macro_export]
macro_rules! unit_test_part_data {

    ($Hasher:ident) => (

        #[test]
        fn part_data() {

            use digest::Digest;
            use core::hash::Hasher;

            let mut hasher1 = super::$Hasher::new();
            let mut hasher2 = super::$Hasher::new();

            let data = b"abcdef";

            hasher1.write(&data[..3]);
            hasher1.write(&data[3..]);
            hasher2.write(data);

            assert!(hasher1.finish() == hasher2.finish());

        }



    )
}

#[macro_export]
macro_rules! unit_test_single_byte {

    ($Hasher:ident, $input:expr, $output:expr) => (

        #[test]
        fn single_byte() {

            use digest::Digest;
            use core::hash::Hasher;
            use generic_array::GenericArray;

            let mut hasher = super::$Hasher::new();
            hasher.write($input);

            output_array!($Hasher, $output as u64, array);

            assert!(hasher.finish() == $output as u64);
            assert!(hasher.fixed_result() == array);
        }
    )

}
