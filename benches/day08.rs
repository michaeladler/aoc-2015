use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day08;

fn criterion_benchmark_day08(c: &mut Criterion) {
    let input = aoc_io::read_input(08).unwrap();
    let mut day08 = Day08::new();
    c.bench_function("day08", |b| b.iter(|| day08.solve(&input)));
}

criterion_group!(benches, criterion_benchmark_day08);
criterion_main!(benches);
