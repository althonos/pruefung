extern crate pruefung;

use crypto_tests::hash::{Test, main_test, one_million_a};

#[test]
fn crc32c_main() {
    let tests = new_tests!("1", "2", "3", "4");
    main_test::<pruefung::crc32c::CRC32C>(&tests);
}

// #[test]
// fn crc32c_1million_a() {
//     let output = include_bytes!("data/one_million_a.output.bin");
//     one_million_a::<pruefung::crc32c::CRC32C>(output);
// }
