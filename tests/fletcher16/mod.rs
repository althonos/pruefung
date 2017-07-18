extern crate pruefung;

use crypto_tests::hash::{Test, main_test, one_million_a};

#[test]
fn fletcher16_main() {
    let tests = new_tests!("1", "2", "3", "4");
    main_test::<pruefung::fletcher16::Fletcher16>(&tests);
}

#[test]
fn fletcher16_1million_a() {
    let output = include_bytes!("data/one_million_a.output.bin");
    one_million_a::<pruefung::fletcher16::Fletcher16>(output);
}
