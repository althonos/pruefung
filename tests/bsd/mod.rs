#[macro_use]
extern crate digest;
extern crate pruefung;

use digest::dev::Test;

#[test]
fn main() {
    let tests = new_tests!("1", "2", "3", "4", "5", "6");
    digest::dev::main_test::<pruefung::bsd::Bsd>(&tests);
}

#[test]
fn one_million_a() {
    let output = include_bytes!("data/one_million_a.output.bin");
    digest::dev::one_million_a::<pruefung::bsd::Bsd>(output);
}
