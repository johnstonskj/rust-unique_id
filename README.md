# Crate unique_id

A trait and implementations for unique ID generators.

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.40-green.svg)
[![crates.io](https://img.shields.io/crates/v/unique_id.svg)](https://crates.io/crates/unique_id)
[![docs.rs](https://docs.rs/unique_id/badge.svg)](https://docs.rs/unique_id)
[![travis.ci](https://travis-ci.org/johnstonskj/rust-unique_id.svg?branch=master)](https://travis-ci.org/johnstonskj/rust-unique_id)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-unique_id.svg)](https://github.com/johnstonskj/rust-unique_id/stargazers)

This crate provides four simple traits, starting with `Generator`. This will return successive unique identifiers, the
only requirement of an identifier being that it implements `PartialEq`. 

Each implemented generator is in its own feature, by default all of which are included.

* Generator [`RandomGenerator`](https://docs.rs/unique_id/0.1.2/unique_id/random/struct.RandomGenerator.html), in 
  feature `random`, provides a random number scheme returning `u128` values. Depends on the 
  [uuid](https://crates.io/crates/uuid) crate.
* Generator [`SequenceGenerator`](https://docs.rs/unique_id/0.1.2/unique_id/sequence/struct.SequenceGenerator.html), 
  in feature `sequence`, provides monotonically increasing u64 values in a thread safe manner. Depends on the 
  [atomic_refcell](https://crates.io/crates/atomic_refcell) and [lazy_static](https://crates.io/crates/lazy_static) 
  crates.
* Generator [`StringGenerator`](https://docs.rs/unique_id/0.1.2/unique_id/string/struct.StringGenerator.html), in 
  feature `string`, provides random string values. Depends on the [blob-uuid](https://crates.io/crates/blob-uuid) crate.

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

## Benchmarks

The `cargo bench` command will run a comparison benchmark to show the relative performance of all generators. This
benchmark is in `benches/compare.cs` and uses Criterion for report generation.

```bash
$ cargo bench

    Finished bench [optimized] target(s) in 17.16s
     Running target/release/deps/unique_id-4944964a39587480

running 3 tests
test random::tests::test_something ... ignored
test sequence::tests::test_something ... ignored
test string::tests::test_something ... ignored

test result: ok. 0 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out

     Running target/release/deps/compare-cfeb3571caa9de30
Compare Implementations/string
                        time:   [928.16 ns 1.0963 us 1.2829 us]
                        change: [+108.47% +224.51% +419.70%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
Compare Implementations/integer
                        time:   [21.033 ns 21.434 ns 21.885 ns]
                        change: [-4.1097% +0.4756% +5.4467%] (p = 0.84 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
Compare Implementations/random
                        time:   [36.741 ns 38.285 ns 40.487 ns]
                        change: [-33.414% -24.047% -13.624%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe
```

The output can be found in `target/criterion/report/index.html`.

## Changes

**Version 0.1.2**

* Added new `RandomGenerator` implementation.
* Put each implementation into its own feature.
* Added `#[inline]` to some functions.

**Version 0.1.1**

* Added trait `GeneratorFromSeed` and implementation for `SequenceGenerator`.
* Added benchmark to compare the two current implementations.

**Version 0.1.0**

* Simple traits `Generator`, `GeneratorWithInvalid`, and `GeneratorFromStr`.
* `StringGenerator` using UUIDs
* `SequenceGenerator` using `i64`

## TODO

1. Decide on better `FromStr` support.