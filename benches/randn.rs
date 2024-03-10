use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nal_stats::Randn;
use nalgebra::{DMatrix, DVector};

fn bench_randn_matrix(c: &mut Criterion) {
    c.bench_function("randn_vector 50000", |b| {
        b.iter(|| DVector::<f64>::randn(black_box(42), black_box(50000)))
    });
    c.bench_function("randn_matrix 100x500", |b| {
        b.iter(|| DMatrix::<f64>::randn(black_box(42), black_box((100, 500))))
    });
    c.bench_function("randn_3d 10x5x1000", |b| {
        b.iter(|| Vec::<DMatrix<f64>>::randn(black_box(42), black_box((10, 5, 1000))))
    });
}

criterion_group!(benches, bench_randn_matrix);
criterion_main!(benches);
