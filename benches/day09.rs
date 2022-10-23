use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day09;

fn criterion_benchmark_day09(c: &mut Criterion) {
    let input = aoc_io::read_input(09).unwrap();
    let mut day09 = Day09::new();
    c.bench_function("day09", |b| b.iter(|| day09.solve(&input)));
}

criterion_group!(benches, criterion_benchmark_day09);
criterion_main!(benches);
