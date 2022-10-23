use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day07;

fn criterion_benchmark_day07(c: &mut Criterion) {
    let input = aoc_io::read_input(07).unwrap();
    let mut day07 = Day07::new();
    c.bench_function("day07", |b| b.iter(|| day07.solve(&input)));
}

criterion_group!(benches, criterion_benchmark_day07);
criterion_main!(benches);
