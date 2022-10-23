use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day01;

fn criterion_benchmark_day01(c: &mut Criterion) {
    let input = aoc_io::read_input(1).unwrap();
    let mut day01 = Day01::new();
    c.bench_function("day01", |b| b.iter(|| day01.solve(&input)));
}

criterion_group!(benches, criterion_benchmark_day01,);
criterion_main!(benches);
