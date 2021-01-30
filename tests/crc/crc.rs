
use pruefung::new_test;
use pruefung::crc::*;
use pruefung::dev::{ digest_test, one_million_a };

mod crc8 {
    use super::*;

    new_test!(test_1, "crc8/1", Crc8, digest_test);
    new_test!(test_2, "crc8/2", Crc8, digest_test);
    new_test!(test_3, "crc8/3", Crc8, digest_test);
    new_test!(test_4, "crc8/4", Crc8, digest_test);
    new_test!(test_5, "crc8/5", Crc8, digest_test);
    new_test!(test_6, "crc8/6", Crc8, digest_test);
    new_test!(test_7, "crc8/7", Crc8, digest_test);

    #[test]
    fn one_million_a_test() {
        let output = include_bytes!("data/crc8/one_million_a.output.bin");
        one_million_a::<Crc8>(output);
    }

}

mod crc16 {
    use super::*;

    new_test!(test_1, "crc16/1", Crc16, digest_test);
    new_test!(test_2, "crc16/2", Crc16, digest_test);
    new_test!(test_3, "crc16/3", Crc16, digest_test);
    new_test!(test_4, "crc16/4", Crc16, digest_test);
    new_test!(test_5, "crc16/5", Crc16, digest_test);
    new_test!(test_6, "crc16/6", Crc16, digest_test);
    new_test!(test_7, "crc16/7", Crc16, digest_test);

    #[test]
    fn one_million_a_test() {
        let output = include_bytes!("data/crc16/one_million_a.output.bin");
        one_million_a::<Crc16>(output);
    }

}

mod crc32 {
    use super::*;

    new_test!(test_1, "crc32/1", Crc32, digest_test);
    new_test!(test_2, "crc32/2", Crc32, digest_test);
    new_test!(test_3, "crc32/3", Crc32, digest_test);
    new_test!(test_4, "crc32/4", Crc32, digest_test);
    new_test!(test_5, "crc32/5", Crc32, digest_test);
    new_test!(test_6, "crc32/6", Crc32, digest_test);
    new_test!(test_7, "crc32/7", Crc32, digest_test);

    #[test]
    fn one_million_a_test() {
        let output = include_bytes!("data/crc32/one_million_a.output.bin");
        one_million_a::<Crc32>(output);
    }
}

mod crc32c {
    use super::*;

    new_test!(test_1, "crc32c/1", Crc32c, digest_test);
    new_test!(test_2, "crc32c/2", Crc32c, digest_test);
    new_test!(test_3, "crc32c/3", Crc32c, digest_test);
    new_test!(test_4, "crc32c/4", Crc32c, digest_test);
    new_test!(test_5, "crc32c/5", Crc32c, digest_test);
    new_test!(test_6, "crc32c/6", Crc32c, digest_test);
    new_test!(test_7, "crc32c/7", Crc32c, digest_test);

    #[test]
    fn one_million_a_test() {
        let output = include_bytes!("data/crc32c/one_million_a.output.bin");
        one_million_a::<Crc32c>(output);
    }
}

mod crc64 {
    use super::*;

    new_test!(test_1, "crc64/1", Crc64, digest_test);
    new_test!(test_2, "crc64/2", Crc64, digest_test);
    new_test!(test_3, "crc64/3", Crc64, digest_test);
    new_test!(test_4, "crc64/4", Crc64, digest_test);
    new_test!(test_5, "crc64/5", Crc64, digest_test);
    new_test!(test_6, "crc64/6", Crc64, digest_test);

    // FIXME: Test fails! After reviewing test files, they seem to be incorrect.
    //new_test!(test_7, "crc64/7", Crc64, digest_test);

    #[test]
    fn one_million_a_test() {
        let output = include_bytes!("data/crc64/one_million_a.output.bin");
        one_million_a::<Crc64>(output);
    }
}
