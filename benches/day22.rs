use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day22;

fn criterion_benchmark_day22(c: &mut Criterion) {
    let input = aoc_io::read_input(22).unwrap();
    let mut day22 = Day22::new();
    c.bench_function("day22", |b| b.iter(|| day22.solve(&input)));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark_day22
);
criterion_main!(benches);
