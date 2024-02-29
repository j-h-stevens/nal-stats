use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nal_stats::round::Round;
use nalgebra::{DMatrix, DVector};

fn bench_round_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("RoundOperations");

    let mut matrix = DMatrix::from_element(100, 100, 123.456789);
    let mut vector = DVector::from_element(1000, 123.456789);

    group.bench_function("DMatrix round_", |b| b.iter(|| matrix.round_(black_box(2))));
    group.bench_function("DMatrix round", |b| b.iter(|| matrix.round(black_box(2))));

    group.bench_function("DVector round_", |b| b.iter(|| vector.round_(black_box(3))));
    group.bench_function("DVector round", |b| b.iter(|| vector.round(black_box(3))));

    group.finish();
}

criterion_group!(benches, bench_round_operations);
criterion_main!(benches);
