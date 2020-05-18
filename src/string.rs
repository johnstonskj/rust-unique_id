/*!
An implementation that provides unique string values.

These string values are a fixed length and are generated as a representation of random UUID 128-bit
values. This implementation depends upon the [blob_uuid](https://crates.io/crates/blob-uuid) crate.

# Example

```rust
use unique_id::{Generator, GeneratorWithInvalid};
use unique_id::string::StringGenerator;

let gen = StringGenerator::default();
let id = gen.next_id();
assert_ne!(id, StringGenerator::invalid_id())
```
*/

use crate::{Generator, GeneratorFromStr, GeneratorWithInvalid};
use std::marker::PhantomData;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Generates random, unique string values from UUIDs. This implementation does provide an invalid
/// value, the empty string.
///
/// Provides implementations of:
///
/// * `Generator` - returns random `String` values.
/// * `GeneratorWithInvalid` - returns an invalid, as an ID, `String` value.
/// * `GeneratorFromStr` - ensures validity of a string representation as an `String` ID.
///
#[derive(Clone, Debug)]
pub struct StringGenerator {
    private: PhantomData<String>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for StringGenerator {
    fn default() -> Self {
        StringGenerator {
            private: Default::default(),
        }
    }
}

impl Generator<String> for StringGenerator {
    #[inline]
    fn next_id(&self) -> String {
        blob_uuid::random_blob()
    }
}

impl GeneratorWithInvalid<String> for StringGenerator {
    #[inline]
    fn invalid_id() -> String
    where
        Self: Sized,
    {
        String::new()
    }
}

impl GeneratorFromStr<String> for StringGenerator {
    fn is_valid_value(s: &str) -> bool {
        !s.is_empty()
            && s.chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    }
}
