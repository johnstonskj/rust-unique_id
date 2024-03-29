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
use uuid::Uuid;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Generates random u128 values generated from UUIDs. This implementation does provide an
/// invalid value, `0 as u128`.
///
/// Provides implementations of:
///
/// * `Generator` - returns random `u128` values.
/// * `GeneratorWithInvalid` - returns an invalid, as an ID, `u128` value.
///
#[derive(Clone, Debug, Default)]
pub struct RandomGenerator;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

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
