use std::collections::HashSet;
use std::hash::Hash;
use std::time::Instant;
use unique_id::random::RandomGenerator;
use unique_id::sequence::SequenceGenerator;
use unique_id::string::StringGenerator;
use unique_id::{Generator, GeneratorWithInvalid};

#[test]
fn test_random_uniqueness() {
    let generator = RandomGenerator::default();
    test_generator(generator, Some(RandomGenerator::invalid_id()), "random");
}

#[test]
fn test_sequence_uniqueness() {
    let generator = SequenceGenerator::default();
    test_generator(generator, Some(SequenceGenerator::invalid_id()), "sequence");
}

#[test]
fn test_string_uniqueness() {
    let generator = StringGenerator::default();
    test_generator(generator, Some(StringGenerator::invalid_id()), "string");
}

const ITERATIONS: usize = 1_000_000;

fn test_generator<T: PartialEq + Eq + Hash>(
    generator: impl Generator<T>,
    illegal: Option<T>,
    label: &str,
) {
    let mut generated = HashSet::new();
    let now = Instant::now();
    for _ in 0..ITERATIONS {
        let value = generator.next_id();
        if let Some(illegal) = &illegal {
            assert!(&value != illegal);
        }
        generated.insert(value);
    }
    // Anything other than equality here implies that non-unique
    // values were inserted into the set and therefore de-duped.
    assert_eq!(generated.len(), ITERATIONS);
    println!(
        "Generated {} {} values in {}ms",
        ITERATIONS,
        label,
        now.elapsed().as_millis()
    );
}
