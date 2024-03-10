use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nal_stats::Cor2;
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

fn benchmark_cor2chol_u(c: &mut Criterion) {
    c.bench_function("cor2chol_u 100x100", |b| {
        let cor = DMatrix::<f64>::identity(100, 100);
        let std_dev = DVector::<f64>::from_element(100, 1.0);
        b.iter(|| cor.cor2chol_u(black_box(&std_dev)));
    });
}

criterion_group!(benches, cor2cov_benchmark, benchmark_cor2chol_u);
criterion_main!(benches);
