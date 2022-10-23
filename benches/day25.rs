use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day25;

fn criterion_benchmark_day25(c: &mut Criterion) {
    let input = aoc_io::read_input(25).unwrap();
    let mut day25 = Day25::new();
    c.bench_function("day25", |b| b.iter(|| day25.solve(&input)));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark_day25
);
criterion_main!(benches);
