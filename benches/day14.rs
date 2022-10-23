use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day14;

fn criterion_benchmark_day14(c: &mut Criterion) {
    let input = aoc_io::read_input(14).unwrap();
    let mut day14 = Day14::new();
    c.bench_function("day14", |b| b.iter(|| day14.solve(&input)));
}

criterion_group!(benches, criterion_benchmark_day14);
criterion_main!(benches);
