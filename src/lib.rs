/*!
A trait and implementations for unique ID generators.

This crate provides a very simple trait, `Generator`, that will return successive unique identifiers. Two
implementations of this trait are provided which provide unique string and integer values respectively.

# Example

The following shows an example of the `StringGenerator` implementation.

```rust,ignore
use unique_id::Generator;
use unique_id::string::StringGenerator;

let gen = StringGenerator::default();
let mut last = gen.next_id();
for _ in 1..100_000 {
    let next = gen.next_id();
    assert_ne!(last, next);
    last = next;
}

*/

#![warn(
// ---------- Stylistic
future_incompatible,
nonstandard_style,
rust_2018_idioms,
trivial_casts,
trivial_numeric_casts,
// ---------- Public
missing_debug_implementations,
missing_docs,
unreachable_pub,
// ---------- Unsafe
unsafe_code,
// ---------- Unused
unused_extern_crates,
unused_import_braces,
unused_qualifications,
unused_results,
)]

#[cfg(feature = "sequence")]
#[macro_use]
extern crate lazy_static;

use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The primary ID generator trait, it provides for generating a new ID with `next_id()`. There
/// is no implication that this returns any overall sequence of values, only that it returns a
/// unique value for each call.
///
pub trait Generator<T>: Default
where
    T: PartialEq,
{
    ///
    /// Generate a new ID, this may be an entirely random value, or as the name implies the
    /// next in some logical sequence.
    ///
    fn next_id(&self) -> T;
}

///
/// For generators that are able to reserve a unique value that is **not valid** as an ID.
///
pub trait GeneratorWithInvalid<T>: Generator<T>
where
    T: PartialEq,
{
    ///
    /// Return a unique value that is **not valid** as an ID.
    ///
    fn invalid_id() -> T
    where
        Self: Sized;
}

///
/// If the type `T` implements `FromStr` then the associated function `is_valid_value` determines
/// whether the value `s` is valid as an ID value.
///
pub trait GeneratorFromStr<T>: Generator<T>
where
    T: PartialEq + FromStr,
{
    ///
    /// Is the value `s` valid as an ID?
    ///
    fn is_valid_value(s: &str) -> bool;
}

///
/// While it is required for a generator to support the `Default` trait, in some cases it is useful
/// to create a new generator some some known initial value, the seed.
///
pub trait GeneratorFromSeed<T>: Generator<T>
where
    T: PartialEq,
{
    ///
    /// Create a new generator with the provided value as a seed. If the value for `seed` is invalid as
    /// an ID this function will panic.
    ///
    fn new(seed: T) -> Self;
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "random")]
pub mod random;

#[cfg(feature = "sequence")]
pub mod sequence;

#[cfg(feature = "string")]
pub mod string;
