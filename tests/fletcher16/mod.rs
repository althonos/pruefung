
extern crate pruefung;

use pruefung::new_test;
use pruefung::fletcher16::Fletcher16;
use pruefung::dev::{ digest_test, one_million_a };


new_test!(test_1, "1", Fletcher16, digest_test);
new_test!(test_2, "2", Fletcher16, digest_test);
new_test!(test_3, "3", Fletcher16, digest_test);
new_test!(test_4, "4", Fletcher16, digest_test);

// FIXME: Test data files exist, but were not included in original tests.
//new_test!(test_5, "5", Fletcher16, digest_test);
//new_test!(test_6, "6", Fletcher16, digest_test);

#[test]
fn one_million_a_test() {
    let output = include_bytes!("data/one_million_a.output.bin");
    one_million_a::<Fletcher16>(output);
}
