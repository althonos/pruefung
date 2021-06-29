
extern crate pruefung;

use pruefung::new_test;
use pruefung::fnv::fnv32::*;
use pruefung::dev::{ digest_test, one_million_a };

mod fnv32 {
    use super::*;

    new_test!(test_1, "fnv32/1", Fnv32, digest_test);
    new_test!(test_2, "fnv32/2", Fnv32, digest_test);
    new_test!(test_3, "fnv32/3", Fnv32, digest_test);
    new_test!(test_4, "fnv32/4", Fnv32, digest_test);
    new_test!(test_5, "fnv32/5", Fnv32, digest_test);
    new_test!(test_6, "fnv32/6", Fnv32, digest_test);

    #[test]
    fn one_million_a_test() {
        let output = include_bytes!("data/fnv32/one_million_a.output.bin");
        one_million_a::<Fnv32>(output);
    }
}

mod fnv32a {
    use super::*;

    new_test!(test_1, "fnv32a/1", Fnv32a, digest_test);
    new_test!(test_2, "fnv32a/2", Fnv32a, digest_test);
    new_test!(test_3, "fnv32a/3", Fnv32a, digest_test);
    new_test!(test_4, "fnv32a/4", Fnv32a, digest_test);
    new_test!(test_5, "fnv32a/5", Fnv32a, digest_test);
    new_test!(test_6, "fnv32a/6", Fnv32a, digest_test);

    #[test]
    fn one_million_a_test() {
        let output = include_bytes!("data/fnv32a/one_million_a.output.bin");
        one_million_a::<Fnv32a>(output);
    }
}

mod fnv32z {
    use super::*;

    new_test!(test_1, "fnv32z/1", Fnv32z, digest_test);
    new_test!(test_2, "fnv32z/2", Fnv32z, digest_test);
    new_test!(test_3, "fnv32z/3", Fnv32z, digest_test);
    new_test!(test_4, "fnv32z/4", Fnv32z, digest_test);
    new_test!(test_5, "fnv32z/5", Fnv32z, digest_test);
    new_test!(test_6, "fnv32z/6", Fnv32z, digest_test);

    #[test]
    fn one_million_a_test() {
        let output = include_bytes!("data/fnv32z/one_million_a.output.bin");
        one_million_a::<Fnv32z>(output);
    }
}
