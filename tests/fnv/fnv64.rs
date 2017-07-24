mod fnv64 {

    extern crate pruefung;

    use crypto_tests;
    use crypto_tests::hash::Test;

    #[test]
    fn main() {
        let tests = new_tests!(
            "fnv64/1",
            "fnv64/2",
            "fnv64/3",
            "fnv64/4",
            "fnv64/5",
            "fnv64/6"
        );
        crypto_tests::hash::main_test::<pruefung::fnv::fnv64::Fnv64>(&tests);
    }

    #[test]
    fn one_million_a() {
        let output = include_bytes!("data/fnv64/one_million_a.output.bin");
        crypto_tests::hash::one_million_a::<pruefung::fnv::fnv64::Fnv64>(output);
    }

}

mod fnv64a {

    extern crate pruefung;

    use crypto_tests;
    use crypto_tests::hash::Test;

    #[test]
    fn main() {
        let tests = new_tests!(
            "fnv64a/1",
            "fnv64a/2",
            "fnv64a/3",
            "fnv64a/4",
            "fnv64a/5",
            "fnv64a/6"
        );
        crypto_tests::hash::main_test::<pruefung::fnv::fnv64::Fnv64a>(&tests);
    }

    #[test]
    fn one_million_a() {
        let output = include_bytes!("data/fnv64a/one_million_a.output.bin");
        crypto_tests::hash::one_million_a::<pruefung::fnv::fnv64::Fnv64a>(output);
    }

}

mod fnv64z {

    extern crate pruefung;

    use crypto_tests;
    use crypto_tests::hash::Test;

    #[test]
    fn main() {
        let tests = new_tests!(
            "fnv64z/1",
            "fnv64z/2",
            "fnv64z/3",
            "fnv64z/4",
            "fnv64z/5",
            "fnv64z/6"
        );
        crypto_tests::hash::main_test::<pruefung::fnv::Fnv64z>(&tests);
    }

    #[test]
    fn one_million_a() {
        let output = include_bytes!("data/fnv64z/one_million_a.output.bin");
        crypto_tests::hash::one_million_a::<pruefung::fnv::Fnv64z>(output);
    }

}
