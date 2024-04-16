use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nal_stats::Stats;
use nalgebra::{Matrix3, Matrix4};

fn symmetrize_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matrix Symmetrization");

    // 3x3 Matrix symmetrization
    group.bench_function("symmetrize 3x3", |b| {
        let mut matrix = Matrix3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        b.iter(|| {
            black_box(&mut matrix).symmetrize_();
        });
    });

    // 4x4 Matrix symmetrization
    group.bench_function("symmetrize 4x4", |b| {
        let mut matrix = Matrix4::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        b.iter(|| {
            black_box(&mut matrix).symmetrize_();
        });
    });

    group.finish();
}

criterion_group!(benches, symmetrize_benchmark);
criterion_main!(benches);
