use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day06;

fn criterion_benchmark_day06(c: &mut Criterion) {
    let input = aoc_io::read_input(06).unwrap();
    let mut day06 = Day06::new();
    c.bench_function("day06", |b| b.iter(|| day06.solve(&input)));
}

criterion_group!(benches, criterion_benchmark_day06);
criterion_main!(benches);
