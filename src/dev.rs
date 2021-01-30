
#[cfg(feature = "generic")]
pub use digest::dev::{ digest_test, one_million_a };

#[cfg(feature = "generic")]
#[allow(unused_macros)]
#[macro_export]
macro_rules! new_test {
    ($name:ident, $test_name:expr, $hasher:ty, $test_func:ident) => {
        #[test]
        fn $name() {
            let input = include_bytes!(concat!("data/", $test_name, ".input.bin"));
            let output = include_bytes!(concat!("data/", $test_name, ".output.bin"));

            if let Some(desc) = $test_func::<$hasher>(input, output) {
                    panic!(
                    "\n\
                     Failed test №{}: {}\n\
                     input:\t{:?}\n\
                     output:\t{:?}\n",
                    $test_name, desc, input, output,
                );
            }
        }
    };
}
