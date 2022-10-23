use criterion::{criterion_group, criterion_main, Criterion};

use aoc2015::aoc_io::{self, Solver};
use aoc2015::Day23;

fn criterion_benchmark_day23(c: &mut Criterion) {
    let input = aoc_io::read_input(23).unwrap();
    let mut day23 = Day23::new();
    c.bench_function("day23", |b| b.iter(|| day23.solve(&input)));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark_day23
);
criterion_main!(benches);
