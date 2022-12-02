use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use day01::Day01;

pub fn day01(c: &mut Criterion) {
    let mut group = c.benchmark_group("day01");

    let input = include_str!("../input.txt");

    group.bench_function("parse1", |b| b.iter(|| Day01::part_one(black_box(input))));

    group.bench_function("parse2", |b| b.iter(|| Day01::part_two(black_box(input))));

    group.finish();
}

criterion_group!(benches, day01);
criterion_main!(benches);
