use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use day06::Day06;

pub fn day06(c: &mut Criterion) {
    let mut group = c.benchmark_group("day06");

    let input = include_str!("../input.txt");

    group.bench_function("parse1", |b| {
        b.iter(|| Day06::part_one(black_box(input)))
    });

    group.bench_function("parse2", |b| b.iter(|| Day06::part_two(black_box(input))));

    group.finish();
}

criterion_group!(benches, day06);
criterion_main!(benches);
