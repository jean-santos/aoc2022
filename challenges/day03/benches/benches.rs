use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use day03::Day03;

pub fn day03(c: &mut Criterion) {
    let mut group = c.benchmark_group("day03");

    let input = include_str!("../input.txt");

    group.bench_function("parse1", |b| {
        b.iter(|| Day03::sum_priorities(black_box(input)))
    });

    group.bench_function("parse2", |b| b.iter(|| Day03::sum_badge(black_box(input))));

    group.finish();
}

criterion_group!(benches, day03);
criterion_main!(benches);
