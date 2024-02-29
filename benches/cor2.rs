use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nal_stats::Cor2cov;
use nalgebra::{DMatrix, DVector}; // Replace `your_crate_name` with the name of your crate

fn cor2cov_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Cor2cov");
    let matrix = DMatrix::from_diagonal(&DVector::from_vec(vec![1.0; 100]));
    let scale = DVector::from_vec(vec![1.0; 100]);

    group.bench_function("cor2cov_ in-place", |b| {
        b.iter(|| {
            let mut matrix_clone = matrix.clone();
            matrix_clone.cor2cov_(black_box(&scale));
        })
    });
    group.bench_function("cor2cov out-of-place", |b| {
        b.iter(|| {
            matrix.cor2cov(black_box(&scale));
        })
    });
    group.finish();
}

criterion_group!(benches, cor2cov_benchmark);
criterion_main!(benches);
