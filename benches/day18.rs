use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day18;

fn criterion_benchmark_day18(c: &mut Criterion) {
    let input = aoc_io::read_input(18).unwrap();
    let mut day18 = Day18::new();
    c.bench_function("day18", |b| b.iter(|| day18.solve(&input)));
}

criterion_group!(benches, criterion_benchmark_day18);
criterion_main!(benches);
