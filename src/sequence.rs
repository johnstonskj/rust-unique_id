/*!
An implementation that provides monotonically increasing integer values.

While `SequenceGenerator` is thread safe, _and_ it's shared implementation assures that IDs are
generated uniquely among threads, it is always initialized to the same value when a process starts.
It provides an implementation of the trait `GeneratorFromSeed` that allows a process to initialize
the counter with a specific value.

# Example

```rust
use unique_id::Generator;
use unique_id::sequence::SequenceGenerator;

let gen = SequenceGenerator::default();
let id = gen.next_id();
*/

use crate::{Generator, GeneratorFromSeed, GeneratorFromStr, GeneratorWithInvalid};
use std::sync::{
    atomic::{AtomicI64, Ordering},
    LazyLock,
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Generates monotonically increasing `i64` values. Note that only values `>= 0` will be returned
/// such that any negative value can be assumed to be invalid.
///
/// Provides implementations of:
///
/// * `Generator` - returns increasing `i64` values.
/// * `GeneratorWithInvalid` - returns an invalid, as an ID, `i64` value.
/// * `GeneratorFromStr` - ensures validity of a string representation as an `i64` ID.
/// * `GeneratorFromSeed` - initializes the generator with a known seed value.
///
#[derive(Clone, Debug, Default)]
pub struct SequenceGenerator;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct SequenceInner {
    value: AtomicI64,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

static GENERATOR: LazyLock<SequenceInner> = LazyLock::new(|| SequenceInner::default());

impl Generator<i64> for SequenceGenerator {
    fn next_id(&self) -> i64 {
        GENERATOR.value.fetch_add(1, Ordering::SeqCst)
    }
}

impl GeneratorWithInvalid<i64> for SequenceGenerator {
    fn invalid_id() -> i64
    where
        Self: Sized,
    {
        -1
    }
}

impl GeneratorFromStr<i64> for SequenceGenerator {
    fn is_valid_value(s: &str) -> bool {
        s.chars().all(|c| c.is_ascii_digit())
    }
}

impl GeneratorFromSeed<i64> for SequenceGenerator {
    fn new(seed: i64) -> Self {
        assert!(seed >= 0);
        GENERATOR.value.store(seed, Ordering::Relaxed);
        Self::default()
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for SequenceInner {
    fn default() -> Self {
        Self {
            value: AtomicI64::new(1),
        }
    }
}
