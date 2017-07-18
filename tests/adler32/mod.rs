extern crate pruefung;

use crypto_tests::hash::{Test, main_test, one_million_a};

#[test]
fn adler32_main() {
    let tests = new_tests!("1", "2", "3", "4", "5", "6");
    main_test::<pruefung::adler32::Adler32>(&tests);
}

#[test]
fn adler32_1million_a() {
    let output = include_bytes!("data/one_million_a.output.bin");
    one_million_a::<pruefung::adler32::Adler32>(output);
}
