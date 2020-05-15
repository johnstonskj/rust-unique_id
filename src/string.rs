/*!
One-line description.

More detailed description, with

# Example

*/

use crate::Generator;
use std::marker::PhantomData;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct StringGenerator {
    private: PhantomData<String>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const INVALID_VALUE: &str = "<invalid-state-tag>";

impl Default for StringGenerator {
    fn default() -> Self {
        StringGenerator {
            private: Default::default(),
        }
    }
}

impl Generator<String> for StringGenerator {
    fn invalid_id() -> String
    where
        Self: Sized,
    {
        INVALID_VALUE.into()
    }

    fn next_id(&self) -> String {
        blob_uuid::random_blob()
    }

    fn is_valid_value(&self, s: &str) -> bool {
        !s.is_empty()
            && s.chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
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
        let gen = StringGenerator::default();
        let mut last = gen.next_id();
        for _ in 1..100_000 {
            let next = gen.next_id();
            assert_ne!(last, next);
            last = next;
        }
    }
}
