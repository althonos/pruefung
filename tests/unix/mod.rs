
extern crate pruefung;

use pruefung::new_test;
use pruefung::unix::Unix;
use pruefung::dev::{ digest_test, one_million_a };

new_test!(test_1, "1", Unix, digest_test);
new_test!(test_2, "2", Unix, digest_test);
new_test!(test_3, "3", Unix, digest_test);
new_test!(test_4, "4", Unix, digest_test);
new_test!(test_5, "5", Unix, digest_test);
new_test!(test_6, "6", Unix, digest_test);

#[test]
fn one_million_a_test() {
    let output = include_bytes!("data/one_million_a.output.bin");
    one_million_a::<Unix>(output);
}
