use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use quancoms_core::qubit::QuantumRegister;

#[allow(non_snake_case)]
fn bench_X_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("X_Optimization");

    for n in 4..=10 {
        group.bench_with_input(BenchmarkId::new("New_Bit_Mask", n), &n, |b, &n| {
            let mut q = QuantumRegister::new(n).unwrap();
            b.iter(|| {
                // 这里调用你那个没优化的老算法
                black_box(q.X_test(n / 2)).unwrap();
            });
        });

        group.bench_with_input(BenchmarkId::new("Bit_Insertion_2n-1", n), &n, |b, &n| {
            let mut q = QuantumRegister::new(n).unwrap();
            b.iter(|| {
                // 这里调用你优化的新算法
                black_box(q.X(n / 2).unwrap());
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_X_comparison);
criterion_main!(benches);
