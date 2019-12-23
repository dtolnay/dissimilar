Dissimilar: diff library with semantic cleanup
==============================================

[![Build Status](https://api.travis-ci.com/dtolnay/dissimilar.svg?branch=master)](https://travis-ci.com/dtolnay/dissimilar)
[![Latest Version](https://img.shields.io/crates/v/dissimilar.svg)](https://crates.io/crates/dissimilar)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/dissimilar)

This library is a port of the Diff component of [Diff Match Patch] to Rust. The
diff implementation is based on [Myers' diff algorithm] but includes some
[semantic cleanups] to increase human readability by factoring out commonalities
which are likely to be coincidental.

Diff Match Patch was originally built in 2006 to power Google Docs.

[Diff Match Patch]: https://github.com/google/diff-match-patch
[Myers' diff algorithm]: https://neil.fraser.name/writing/diff/myers.pdf
[semantic cleanups]: https://neil.fraser.name/writing/diff/

```toml
[dependencies]
dissimilar = "0.0"
```

*Compiler support: requires rustc 1.31+*

## Interface

Here is the entire API of the Rust implementation. It operates on borrowed
strings and the return value of the diff algorithm is a vector of chunks
pointing into slices of those input strings.

```rust
pub enum Chunk<'a> {
    Equal(&'a str),
    Delete(&'a str),
    Insert(&'a str),
}

pub fn diff(text1: &str, text2: &str) -> Vec<Chunk>;
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
