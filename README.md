# Crate unique_id

A trait and implementations for unique ID generators.

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.40-green.svg)
[![crates.io](https://img.shields.io/crates/v/unique_id.svg)](https://crates.io/crates/unique_id)
[![docs.rs](https://docs.rs/unique_id/badge.svg)](https://docs.rs/unique_id)
[![travis.ci](https://travis-ci.org/johnstonskj/rust-unique_id.svg?branch=master)](https://travis-ci.org/johnstonskj/rust-unique_id)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-unique_id.svg)](https://github.com/johnstonskj/rust-unique_id/stargazers)

This crate provides three simple traits, starting with `Generator`. This will return successive unique identifiers. Two 
implementations of this trait are provided which provide unique string and integer values respectively.

## Example

The following shows an example of the `StringGenerator` implementation.

```rust
use unique_id::Generator;
use unique_id::string::StringGenerator;

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

* Simple traits `Generator`, `GeneratorWithInvalid`, and `GeneratorFromStr`.
* `StringGenerator` using UUIDs
* `SequenceGenerator` using `i64`

## TODO

1. Allow for seeding
1. Decide on FromStr support