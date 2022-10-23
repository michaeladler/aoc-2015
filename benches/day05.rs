use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day05;

fn criterion_benchmark_day05(c: &mut Criterion) {
    let input = aoc_io::read_input(05).unwrap();
    let mut day05 = Day05::new();
    c.bench_function("day05", |b| b.iter(|| day05.solve(&input)));
}

criterion_group!(benches, criterion_benchmark_day05);
criterion_main!(benches);
