use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nal_stats::pola2::Df2Nal; // Adjust the import path based on your project structure
use polars::prelude::*;

fn dataframe_conversion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("DataFrame_to_DMatrix");

    group.bench_function("large_dataframe", |b| {
        let df = df![
            "a" => (0..10000).map(|v| v as f64).collect::<Vec<_>>(),
            "b" => (10000..20000).map(|v| v as f64).collect::<Vec<_>>()
        ]
        .unwrap();

        b.iter(|| {
            let _matrix = black_box(&df).to_nal_mat::<f64>().unwrap();
        });
    });

    group.finish();
}

criterion_group!(benches, dataframe_conversion_benchmark);
criterion_main!(benches);
