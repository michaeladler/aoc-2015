use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day02;

fn criterion_benchmark_day02(c: &mut Criterion) {
    let input = aoc_io::read_input(2).unwrap();
    let mut day02 = Day02::new();
    c.bench_function("day02", |b| b.iter(|| day02.solve(&input)));
}

criterion_group!(benches, criterion_benchmark_day02);
criterion_main!(benches);
