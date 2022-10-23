use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day21;

fn criterion_benchmark_day21(c: &mut Criterion) {
    let input = aoc_io::read_input(21).unwrap();
    let mut day21 = Day21::new();
    c.bench_function("day21", |b| b.iter(|| day21.solve(&input)));
}

criterion_group!(benches, criterion_benchmark_day21);
criterion_main!(benches);
