mod crc8 {

    use digest;
    use digest::dev::Test;
    use pruefung;

    #[test]
    fn main() {
        let tests =
            new_tests!("crc8/1", "crc8/2", "crc8/3", "crc8/4", "crc8/5", "crc8/6", "crc8/7");
        digest::dev::main_test::<pruefung::crc::crc8::Crc8>(&tests);
    }

    #[test]
    fn one_million_a() {
        let output = include_bytes!("data/crc8/one_million_a.output.bin");
        digest::dev::one_million_a::<pruefung::crc::crc8::Crc8>(output);
    }

}

mod crc16 {

    use digest;
    use digest::dev::Test;
    use pruefung;

    #[test]
    fn main() {
        let tests =
            new_tests!("crc16/1", "crc16/2", "crc16/3", "crc16/4", "crc16/5", "crc16/6", "crc16/7");
        digest::dev::main_test::<pruefung::crc::crc16::Crc16>(&tests);
    }

    #[test]
    fn one_million_a() {
        let output = include_bytes!("data/crc16/one_million_a.output.bin");
        digest::dev::one_million_a::<pruefung::crc::crc16::Crc16>(output);
    }

}

mod crc32 {

    use digest;
    use digest::dev::Test;
    use pruefung;

    #[test]
    fn main() {
        let tests =
            new_tests!("crc32/1", "crc32/2", "crc32/3", "crc32/4", "crc32/5", "crc32/6", "crc32/7");
        digest::dev::main_test::<pruefung::crc::crc32::Crc32>(&tests);
    }

    #[test]
    fn one_million_a() {
        let output = include_bytes!("data/crc32/one_million_a.output.bin");
        digest::dev::one_million_a::<pruefung::crc::crc32::Crc32>(output);
    }

}

mod crc32c {

    use digest;
    use digest::dev::Test;
    use pruefung;

    #[test]
    fn main() {
        let tests = new_tests!(
            "crc32c/1", "crc32c/2", "crc32c/3", "crc32c/4", "crc32c/5", "crc32c/6", "crc32c/7"
        );
        digest::dev::main_test::<pruefung::crc::crc32::Crc32c>(&tests);
    }

    #[test]
    fn one_million_a() {
        let output = include_bytes!("data/crc32c/one_million_a.output.bin");
        digest::dev::one_million_a::<pruefung::crc::crc32::Crc32c>(output);
    }

}

mod crc64 {

    use digest;
    use digest::dev::Test;
    use pruefung;

    #[test]
    fn main() {
        let tests = new_tests!(
            "crc64/1", "crc64/2", /*, "crc64/3", "crc64/4", "crc64/5", "crc64/6", "crc64/7"*/
        );
        digest::dev::main_test::<pruefung::crc::crc64::Crc64>(&tests);
    }

    #[test]
    fn one_million_a() {
        let output = include_bytes!("data/crc64/one_million_a.output.bin");
        digest::dev::one_million_a::<pruefung::crc::crc64::Crc64>(output);
    }

}
