use criterion::{criterion_group, criterion_main, Criterion};
use day03::{part_one_naive, part_two_naive};

pub fn part_one_naive_benchmark(c: &mut Criterion) {
    c.bench_function("part_one_naive", |b| b.iter(|| part_one_naive()));
}

pub fn part_two_naive_benchmark(c: &mut Criterion) {
    c.bench_function("part_two_naive", |b| b.iter(|| part_two_naive()));
}

criterion_group!(benches, part_one_naive_benchmark, part_two_naive_benchmark);
criterion_main!(benches);
