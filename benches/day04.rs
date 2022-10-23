use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day04;

fn criterion_benchmark_day04(c: &mut Criterion) {
    let input = aoc_io::read_input(04).unwrap();
    let mut day04 = Day04::new();
    c.bench_function("day04", |b| b.iter(|| day04.solve(&input)));
}

criterion_group!(benches, criterion_benchmark_day04);
criterion_main!(benches);
