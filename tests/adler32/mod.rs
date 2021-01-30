
extern crate pruefung;

use pruefung::new_test;
use pruefung::adler32::Adler32;
use pruefung::dev::{ digest_test, one_million_a };

new_test!(test_1, "1", Adler32, digest_test);
new_test!(test_2, "2", Adler32, digest_test);
new_test!(test_3, "3", Adler32, digest_test);
new_test!(test_4, "4", Adler32, digest_test);
new_test!(test_5, "5", Adler32, digest_test);
new_test!(test_6, "6", Adler32, digest_test);

#[test]
fn one_million_a_test() {
    let output = include_bytes!("data/one_million_a.output.bin");
    one_million_a::<Adler32>(output);
}
