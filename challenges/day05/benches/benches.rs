use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use day05::Day05;

pub fn day05(c: &mut Criterion) {
    let mut group = c.benchmark_group("day05");

    let input = include_str!("../input.txt");

    group.bench_function("parse1", |b| {
        b.iter(|| Day05::part_one(black_box(input)))
    });

    group.bench_function("parse2", |b| b.iter(|| Day05::part_two(black_box(input))));

    group.finish();
}

criterion_group!(benches, day05);
criterion_main!(benches);
