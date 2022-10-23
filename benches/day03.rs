use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day03;

fn criterion_benchmark_day03(c: &mut Criterion) {
    let input = aoc_io::read_input(03).unwrap();
    let mut day03 = Day03::new();
    c.bench_function("day03", |b| b.iter(|| day03.solve(&input)));
}

criterion_group!(benches, criterion_benchmark_day03);
criterion_main!(benches);
