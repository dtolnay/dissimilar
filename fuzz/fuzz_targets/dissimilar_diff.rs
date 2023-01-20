#![no_main]

use libfuzzer_sys::fuzz_target;
use std::str;

fuzz_target!(|input: (&str, &str)| {
    let (text1, text2) = input;
    if text1.len() < 100 && text2.len() < 100 {
        let _chunks = dissimilar::diff(text1, text2);
        // TODO: verify that concatenating the Equal+Delete chunks equals text1,
        // concatenating Equal+Insert chunks equals text2.
    }
});
