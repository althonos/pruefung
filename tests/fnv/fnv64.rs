
extern crate pruefung;

use pruefung::new_test;
use pruefung::fnv::fnv64::*;
use pruefung::dev::{ digest_test, one_million_a };

mod fnv64 {
    use super::*;

    new_test!(test_1, "fnv64/1", Fnv64, digest_test);
    new_test!(test_2, "fnv64/2", Fnv64, digest_test);
    new_test!(test_3, "fnv64/3", Fnv64, digest_test);
    new_test!(test_4, "fnv64/4", Fnv64, digest_test);
    new_test!(test_5, "fnv64/5", Fnv64, digest_test);
    new_test!(test_6, "fnv64/6", Fnv64, digest_test);

    #[test]
    fn one_million_a_test() {
        let output = include_bytes!("data/fnv64/one_million_a.output.bin");
        one_million_a::<Fnv64>(output);
    }
}

mod fnv64a {
    use super::*;

    new_test!(test_1, "fnv64a/1", Fnv64a, digest_test);
    new_test!(test_2, "fnv64a/2", Fnv64a, digest_test);
    new_test!(test_3, "fnv64a/3", Fnv64a, digest_test);
    new_test!(test_4, "fnv64a/4", Fnv64a, digest_test);
    new_test!(test_5, "fnv64a/5", Fnv64a, digest_test);
    new_test!(test_6, "fnv64a/6", Fnv64a, digest_test);

    #[test]
    fn one_million_a_test() {
        let output = include_bytes!("data/fnv64a/one_million_a.output.bin");
        one_million_a::<Fnv64a>(output);
    }
}

mod fnv64z {
    use super::*;

    new_test!(test_1, "fnv64z/1", Fnv64z, digest_test);
    new_test!(test_2, "fnv64z/2", Fnv64z, digest_test);
    new_test!(test_3, "fnv64z/3", Fnv64z, digest_test);
    new_test!(test_4, "fnv64z/4", Fnv64z, digest_test);
    new_test!(test_5, "fnv64z/5", Fnv64z, digest_test);
    new_test!(test_6, "fnv64z/6", Fnv64z, digest_test);

    #[test]
    fn one_million_a_test() {
        let output = include_bytes!("data/fnv64z/one_million_a.output.bin");
        one_million_a::<Fnv64z>(output);
    }
}
