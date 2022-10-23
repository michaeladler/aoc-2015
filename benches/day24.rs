use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day24;

fn criterion_benchmark_day24(c: &mut Criterion) {
    let input = aoc_io::read_input(24).unwrap();
    let mut day24 = Day24::new();
    c.bench_function("day24", |b| b.iter(|| day24.solve(&input)));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark_day24
);
criterion_main!(benches);
