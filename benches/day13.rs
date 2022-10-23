use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day13;

fn criterion_benchmark_day13(c: &mut Criterion) {
    let input = aoc_io::read_input(13).unwrap();
    let mut day13 = Day13::new();
    c.bench_function("day13", |b| b.iter(|| day13.solve(&input)));
}

criterion_group!(benches, criterion_benchmark_day13);
criterion_main!(benches);
