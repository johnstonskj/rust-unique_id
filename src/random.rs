/*!
An implementation that provides random 128-bit unsigned integer values.

Using randomly generated UUIDs these are converted to `u128` values for equality testing
and ease of storage. This implementation depends upon the [uuid](https://crates.io/crates/uuid)
crate.

# Example

```rust
use unique_id::{Generator, GeneratorWithInvalid};
use unique_id::random::RandomGenerator;

let gen = RandomGenerator::default();
let id = gen.next_id();
assert_ne!(id, RandomGenerator::invalid_id())
```
*/

use crate::{Generator, GeneratorWithInvalid};
use std::marker::PhantomData;
use uuid::Uuid;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Generates random u128 values generated from UUIDs.
///
/// Provides implementations of:
///
/// * `Generator` - returns random `u128` values.
/// * `GeneratorWithInvalid` - returns an invalid, as an ID, `u128` value.
///
#[derive(Clone, Debug)]
pub struct RandomGenerator {
    private: PhantomData<String>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for RandomGenerator {
    fn default() -> Self {
        RandomGenerator {
            private: Default::default(),
        }
    }
}

impl Generator<u128> for RandomGenerator {
    #[inline]
    fn next_id(&self) -> u128 {
        Uuid::new_v4().as_u128()
    }
}

impl GeneratorWithInvalid<u128> for RandomGenerator {
    #[inline]
    fn invalid_id() -> u128
    where
        Self: Sized,
    {
        Uuid::nil().as_u128()
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
        let gen = RandomGenerator::default();
        let mut last = gen.next_id();
        for _ in 1..100_000 {
            let next = gen.next_id();
            assert_ne!(last, next);
            last = next;
        }
    }
}
