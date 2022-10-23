use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day20;

fn criterion_benchmark_day20(c: &mut Criterion) {
    let input = aoc_io::read_input(20).unwrap();
    let mut day20 = Day20::new();
    c.bench_function("day20", |b| b.iter(|| day20.solve(&input)));
}

criterion_group!(benches, criterion_benchmark_day20);
criterion_main!(benches);
