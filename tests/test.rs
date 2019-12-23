// Upstream diff-match-patch's test suite is imported as unit tests in
// src/tests.rs, as they test APIs which are private in the Rust implementation.
//
// This directory is for Rust-specific integration tests and regression tests.

use dissimilar::{diff, Chunk};

#[test]
fn test_unicode() {
    // Unicode snowman and unicode comet have the same first two bytes. A
    // byte-based diff would produce a 2-byte Equal followed by 1-byte Delete
    // and Insert.
    let snowman = "\u{2603}";
    let comet = "\u{2604}";
    assert_eq!(snowman.as_bytes()[..2], comet.as_bytes()[..2]);

    let d = diff(snowman, comet);
    assert_eq!(d, vec![Chunk::Delete(snowman), Chunk::Insert(comet)]);
}
