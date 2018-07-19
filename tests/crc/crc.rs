mod crc32 {

    extern crate pruefung;

    use crypto_tests;
    use crypto_tests::hash::Test;

    #[test]
    fn main() {
        let tests = new_tests!("crc32/1", "crc32/2", "crc32/3", "crc32/4", "crc32/5", "crc32/6");
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
        let tests =
            new_tests!("crc32c/1", "crc32c/2", "crc32c/3", "crc32c/4", "crc32c/5", "crc32c/6");
        crypto_tests::hash::main_test::<pruefung::crc::crc32::Crc32c>(&tests);
    }

    #[test]
    fn one_million_a() {
        let output = include_bytes!("data/crc32c/one_million_a.output.bin");
        crypto_tests::hash::one_million_a::<pruefung::crc::crc32::Crc32c>(output);
    }

}

mod crc16 {

    extern crate pruefung;

    use crypto_tests;
    use crypto_tests::hash::Test;

    #[test]
    fn main() {
        let tests = new_tests!("crc16/1", "crc16/2", "crc16/3", "crc16/4", "crc16/5", "crc16/6");
        crypto_tests::hash::main_test::<pruefung::crc::crc16::Crc16>(&tests);
    }

    #[test]
    fn one_million_a() {
        let output = include_bytes!("data/crc16/one_million_a.output.bin");
        crypto_tests::hash::one_million_a::<pruefung::crc::crc16::Crc16>(output);
    }

}

mod crc8 {

    extern crate pruefung;

    use crypto_tests;
    use crypto_tests::hash::Test;

    #[test]
    fn main() {
        let tests = new_tests!("crc8/1", "crc8/2", "crc8/3", "crc8/4", "crc8/5", "crc8/6");
        crypto_tests::hash::main_test::<pruefung::crc::crc8::Crc8>(&tests);
    }

    #[test]
    fn one_million_a() {
        let output = include_bytes!("data/crc8/one_million_a.output.bin");
        crypto_tests::hash::one_million_a::<pruefung::crc::crc8::Crc8>(output);
    }
}
