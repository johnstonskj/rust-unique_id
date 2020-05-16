#[macro_use]
extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion};
use unique_id::random::RandomGenerator;
use unique_id::sequence::SequenceGenerator;
use unique_id::string::StringGenerator;
use unique_id::Generator;

fn generator_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Compare Implementations");
    let str_gen = StringGenerator::default();
    let int_gen = SequenceGenerator::default();
    let rnd_gen = RandomGenerator::default();

    group.bench_function("string", |b| b.iter(|| str_gen.next_id()));
    group.bench_function("integer", |b| b.iter(|| int_gen.next_id()));
    group.bench_function("random", |b| b.iter(|| rnd_gen.next_id()));

    group.finish();
}

criterion_group!(benches, generator_benchmark);
criterion_main!(benches);
