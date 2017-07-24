extern crate pruefung;

use crypto_tests;
use crypto_tests::hash::Test;

#[test]
fn main() {
    let tests = new_tests!("1", "2", "3", "4", "5", "6");
    crypto_tests::hash::main_test::<pruefung::bsd::Bsd>(&tests);
}

#[test]
fn one_million_a() {
    let output = include_bytes!("data/one_million_a.output.bin");
    crypto_tests::hash::one_million_a::<pruefung::bsd::Bsd>(output);
}
