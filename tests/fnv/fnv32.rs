mod fnv32 {

    extern crate pruefung;

    use crypto_tests;
    use crypto_tests::hash::Test;

    #[test]
    fn main() {
        let tests = new_tests!(
            "fnv32/1",
            "fnv32/2",
            "fnv32/3",
            "fnv32/4",
            "fnv32/5",
            "fnv32/6"
        );
        crypto_tests::hash::main_test::<pruefung::fnv::Fnv32>(&tests);
    }

    #[test]
    fn one_million_a() {
        let output = include_bytes!("data/fnv32/one_million_a.output.bin");
        crypto_tests::hash::one_million_a::<pruefung::fnv::Fnv32>(output);
    }

}

mod fnv32a {

    extern crate pruefung;

    use crypto_tests;
    use crypto_tests::hash::Test;

    #[test]
    fn main() {
        let tests = new_tests!(
            "fnv32a/1",
            "fnv32a/2",
            "fnv32a/3",
            "fnv32a/4",
            "fnv32a/5",
            "fnv32a/6"
        );
        crypto_tests::hash::main_test::<pruefung::fnv::Fnv32a>(&tests);
    }

    #[test]
    fn one_million_a() {
        let output = include_bytes!("data/fnv32a/one_million_a.output.bin");
        crypto_tests::hash::one_million_a::<pruefung::fnv::Fnv32a>(output);
    }

}

mod fnv32z {

    extern crate pruefung;

    use crypto_tests;
    use crypto_tests::hash::Test;

    #[test]
    fn main() {
        let tests = new_tests!(
            "fnv32z/1",
            "fnv32z/2",
            "fnv32z/3",
            "fnv32z/4",
            "fnv32z/5",
            "fnv32z/6"
        );
        crypto_tests::hash::main_test::<pruefung::fnv::Fnv32z>(&tests);
    }

    #[test]
    fn one_million_a() {
        let output = include_bytes!("data/fnv32z/one_million_a.output.bin");
        crypto_tests::hash::one_million_a::<pruefung::fnv::Fnv32z>(output);
    }

}
