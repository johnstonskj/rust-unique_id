/*!
An implementation that provides monotonically increasing integer values.

While `SequenceGenerator` is thread safe and it's shared implementation assures that IDs are
generated uniquely among threads it is always initialized to the same value when a process starts.

# Example

```rust
use unique_id::Generator;
use unique_id::sequence::SequenceGenerator;

let gen = SequenceGenerator::default();
let id = gen.next_id();
*/

use crate::{Generator, GeneratorFromSeed, GeneratorFromStr, GeneratorWithInvalid};
use atomic_refcell::AtomicRefCell;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Generates monotonically increasing `i64` values.
///
/// Provides implementations of:
///
/// * `Generator` - returns increasing `i64` values.
/// * `GeneratorWithInvalid` - returns an invalid, as an ID, `i64` value.
/// * `GeneratorFromStr` - ensures validity of a string representation as an `i64` ID.
/// * `GeneratorFromSeed` - initializes the generator with a known seed value.
///
#[derive(Debug)]
pub struct SequenceGenerator {
    private: PhantomData<String>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct SequenceInner {
    value: Arc<AtomicRefCell<AtomicI64>>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref IDGENERATOR: SequenceInner = SequenceInner::default();
}

impl Default for SequenceGenerator {
    fn default() -> Self {
        Self {
            private: Default::default(),
        }
    }
}

impl Generator<i64> for SequenceGenerator {
    fn next_id(&self) -> i64 {
        IDGENERATOR
            .value
            .borrow_mut()
            .fetch_add(1, Ordering::SeqCst)
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
        IDGENERATOR
            .value
            .borrow_mut()
            .store(seed, Ordering::Relaxed);
        Self::default()
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for SequenceInner {
    fn default() -> Self {
        Self {
            value: Arc::new(AtomicRefCell::new(AtomicI64::new(1))),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        let gen = SequenceGenerator::default();
        let mut last = gen.next_id();
        for _ in 1..100_000 {
            let next = gen.next_id();
            assert_ne!(last, next);
            last = next;
        }
    }
}
