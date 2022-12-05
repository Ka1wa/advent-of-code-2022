use criterion::{criterion_group, criterion_main, Criterion};
use day01::{naive, itertools};

pub fn naive_benchmark(c: &mut Criterion) {
    c.bench_function("naive", |b| b.iter(|| naive()));
}

pub fn itertools_benchmark(c: &mut Criterion) {
    c.bench_function("itertools", |b| b.iter(|| itertools()));
}

criterion_group!(benches, naive_benchmark, itertools_benchmark);
criterion_main!(benches);
