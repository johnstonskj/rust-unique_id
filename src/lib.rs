/*!
One-line description.

More detailed description, with

# Example

*/

#[macro_use]
extern crate lazy_static;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Generator<T>: Default
where
    T: Sized + PartialEq,
{
    fn invalid_id() -> T
    where
        Self: Sized;

    fn next_id(&self) -> T;

    fn is_valid_value(&self, s: &str) -> bool;
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod sequence;

pub mod string;
