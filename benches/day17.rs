use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day17;

fn criterion_benchmark_day17(c: &mut Criterion) {
    let input = aoc_io::read_input(17).unwrap();
    let mut day17 = Day17::new();
    c.bench_function("day17", |b| b.iter(|| day17.solve(&input)));
}

criterion_group!(benches, criterion_benchmark_day17);
criterion_main!(benches);
