#![no_main]

use dissimilar::Chunk;
use libfuzzer_sys::fuzz_target;
use std::str;

fuzz_target!(|input: (&str, &str)| {
    let (mut text1, mut text2) = input;
    if text1.len() < 100 && text2.len() < 100 {
        let chunks = dissimilar::diff(text1, text2);
        for chunk in chunks {
            match chunk {
                Chunk::Equal(chunk) => {
                    text1 = text1.strip_prefix(chunk).unwrap();
                    text2 = text2.strip_prefix(chunk).unwrap();
                }
                Chunk::Delete(chunk) => {
                    text1 = text1.strip_prefix(chunk).unwrap();
                }
                Chunk::Insert(chunk) => {
                    text2 = text2.strip_prefix(chunk).unwrap();
                }
            }
        }
        assert!(text1.is_empty());
        assert!(text2.is_empty());
    }
});
