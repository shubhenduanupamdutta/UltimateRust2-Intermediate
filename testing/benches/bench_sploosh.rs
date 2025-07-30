// Challenge: Create a benchmark that measures the speed of sploosh(8, 9, 10)
// - Speed up the implementation of sploosh(8, 9, 10) without breaking the other tests.
// - Hint: See Cargo.toml to get you started

use criterion::{criterion_group, criterion_main, Criterion};
use testing::sploosh;

fn bench_sploosh(c: &mut Criterion) {
    c.bench_function("sploosh(8, 9, 10)", |b| b.iter(|| sploosh(8, 9, 10)));
}
criterion_group!(benches, bench_sploosh);
criterion_main!(benches);