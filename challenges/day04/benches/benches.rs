use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use day04::Day04;

pub fn day04(c: &mut Criterion) {
    let mut group = c.benchmark_group("day04");

    let input = include_str!("../input.txt");

    group.bench_function("parse1", |b| {
        b.iter(|| Day04::part_one(black_box(input)))
    });

    group.bench_function("parse2", |b| b.iter(|| Day04::part_two(black_box(input))));

    group.finish();
}

criterion_group!(benches, day04);
criterion_main!(benches);
