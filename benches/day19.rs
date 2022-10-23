use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day19;

fn criterion_benchmark_day19(c: &mut Criterion) {
    let input = aoc_io::read_input(19).unwrap();
    let mut day19 = Day19::new();
    c.bench_function("day19", |b| b.iter(|| day19.solve(&input)));
}

criterion_group!(benches, criterion_benchmark_day19);
criterion_main!(benches);
