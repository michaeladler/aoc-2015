use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day15;

fn criterion_benchmark_day15(c: &mut Criterion) {
    let input = aoc_io::read_input(15).unwrap();
    let mut day15 = Day15::new();
    c.bench_function("day15", |b| b.iter(|| day15.solve(&input)));
}

criterion_group!(benches, criterion_benchmark_day15);
criterion_main!(benches);
