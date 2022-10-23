use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day10;

fn criterion_benchmark_day10(c: &mut Criterion) {
    let input = aoc_io::read_input(10).unwrap();
    let mut day10 = Day10::new();
    c.bench_function("day10", |b| b.iter(|| day10.solve(&input)));
}

criterion_group!(benches, criterion_benchmark_day10);
criterion_main!(benches);
