extern crate pruefung;

use crypto_tests;
use crypto_tests::hash::Test;

#[test]
fn main() {
    let tests = new_tests!("1", "2", "3", "4");
    crypto_tests::hash::main_test::<pruefung::fletcher16::Fletcher16>(&tests);
}

#[test]
fn one_million_a() {
    let output = include_bytes!("data/one_million_a.output.bin");
    crypto_tests::hash::one_million_a::<pruefung::fletcher16::Fletcher16>(output);
}
