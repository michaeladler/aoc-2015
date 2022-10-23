use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day12;

fn criterion_benchmark_day12(c: &mut Criterion) {
    let input = aoc_io::read_input(12).unwrap();
    let mut day12 = Day12::new();
    c.bench_function("day12", |b| b.iter(|| day12.solve(&input)));
}

criterion_group!(benches, criterion_benchmark_day12);
criterion_main!(benches);
