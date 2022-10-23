use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day11;

fn criterion_benchmark_day11(c: &mut Criterion) {
    let input = aoc_io::read_input(11).unwrap();
    let mut day11 = Day11::new();
    c.bench_function("day11", |b| b.iter(|| day11.solve(&input)));
}

criterion_group!(benches, criterion_benchmark_day11);
criterion_main!(benches);
