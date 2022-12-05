use criterion::{criterion_group, criterion_main, Criterion};
use day01::naive;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("naive", |b| b.iter(|| naive()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
