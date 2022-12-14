use criterion::{criterion_group, criterion_main, Criterion};
use day03::{naive};

pub fn naive_benchmark(c: &mut Criterion) {
    c.bench_function("naive", |b| b.iter(|| naive()));
}

criterion_group!(benches, naive_benchmark);
criterion_main!(benches);
