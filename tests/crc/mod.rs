mod crc32 {

    mod crc32 {

        extern crate pruefung;

        use crypto_tests;
        use crypto_tests::hash::Test;

        #[test]
        fn main() {
            let tests = new_tests!(
                "crc32/1",
                "crc32/2",
                "crc32/3",
                "crc32/4",
                "crc32/5",
                "crc32/6"
            );
            crypto_tests::hash::main_test::<pruefung::crc::crc32::Crc32>(&tests);
        }

        #[test]
        fn one_million_a() {
            let output = include_bytes!("data/crc32/one_million_a.output.bin");
            crypto_tests::hash::one_million_a::<pruefung::crc::crc32::Crc32>(output);
        }

    }

    mod crc32c {

        extern crate pruefung;

        use crypto_tests;
        use crypto_tests::hash::Test;

        #[test]
        fn main() {
            let tests = new_tests!(
                "crc32c/1",
                "crc32c/2",
                "crc32c/3",
                "crc32c/4",
                "crc32c/5",
                "crc32c/6"
            );
            crypto_tests::hash::main_test::<pruefung::crc::crc32::Crc32c>(&tests);
        }

        #[test]
        fn one_million_a() {
            let output = include_bytes!("data/crc32c/one_million_a.output.bin");
            crypto_tests::hash::one_million_a::<pruefung::crc::crc32::Crc32c>(output);
        }

    }

}
