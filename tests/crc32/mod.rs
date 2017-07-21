extern crate pruefung;

use crypto_tests::hash::{Test, main_test, one_million_a};

#[test]
fn crc32_main() {
    let tests = new_tests!(
        "crc32/1",
        "crc32/2",
        "crc32/3",
        "crc32/4",
        "crc32/5",
        "crc32/6"
    );
    main_test::<pruefung::crc32::Crc32>(&tests);
}

#[test]
fn crc32_1million_a() {
    let output = include_bytes!("data/crc32/one_million_a.output.bin");
    one_million_a::<pruefung::crc32::Crc32>(output);
}

#[test]
fn crc32c_main() {
    let tests = new_tests!(
        "crc32c/1",
        "crc32c/2",
        "crc32c/3",
        "crc32c/4",
        "crc32c/5",
        "crc32c/6"
    );
    main_test::<pruefung::crc32::Crc32c>(&tests);
}

#[test]
fn crc32c_1million_a() {
    let output = include_bytes!("data/crc32c/one_million_a.output.bin");
    one_million_a::<pruefung::crc32::Crc32c>(output);
}
