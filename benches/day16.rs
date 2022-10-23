use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day16;

fn criterion_benchmark_day16(c: &mut Criterion) {
    let input = aoc_io::read_input(16).unwrap();
    let mut day16 = Day16::new();
    c.bench_function("day16", |b| b.iter(|| day16.solve(&input)));
}

criterion_group!(benches, criterion_benchmark_day16);
criterion_main!(benches);
