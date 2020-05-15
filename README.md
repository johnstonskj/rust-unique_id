# Crate unique_id

A trait and implementations for unique ID generators.

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.40-green.svg)
[![crates.io](https://img.shields.io/crates/v/upnp-rs.svg)](https://crates.io/crates/unique_id)
[![docs.rs](https://docs.rs/xml_dom/badge.svg)](https://docs.rs/unique_id)
[![travis.ci](https://travis-ci.org/johnstonskj/rust-unique_id.svg?branch=master)](https://travis-ci.org/johnstonskj/rust-unique_id)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-unique_id.svg)](https://github.com/johnstonskj/rust-unique_id/stargazers)

Add a longer description here.

## Example

TBD

```rust
let gen = StringGenerator::default();
let mut last = gen.next_id();
for _ in 1..100_000 {
    let next = gen.next_id();
    assert_ne!(last, next);
    last = next;
}
```

## Changes

**Version 0.1.0**

* Simple trait
* `StringGenerator` using UUIDs
* `SequenceGenerator` using `i64`

## TODO

1. Allow for seeding
1. Decide on FromStr support