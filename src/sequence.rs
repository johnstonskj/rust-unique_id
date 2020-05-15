/*!
One-line description.

More detailed description, with

# Example

*/

use crate::Generator;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct SequenceGenerator {
    private: PhantomData<String>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct SequenceInner {
    value: Arc<RefCell<AtomicI64>>,
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
    fn invalid_id() -> i64
    where
        Self: Sized,
    {
        -1
    }

    fn next_id(&self) -> i64 {
        IDGENERATOR
            .value
            .borrow_mut()
            .fetch_add(1, Ordering::SeqCst)
    }

    fn is_valid_value(&self, s: &str) -> bool {
        s.chars().all(|c| c.is_ascii_digit())
    }
}

impl Default for SequenceInner {
    fn default() -> Self {
        Self {
            value: Arc::new(RefCell::new(AtomicI64::new(1))),
        }
    }
}

#[allow(unsafe_code)]
unsafe impl Sync for SequenceInner {}

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
