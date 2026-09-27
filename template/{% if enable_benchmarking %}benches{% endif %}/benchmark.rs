use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_addition(c: &mut Criterion) {
    c.bench_function("add one", |b| b.iter(|| black_box(41) + 1));
}

criterion_group!(benches, bench_addition);
criterion_main!(benches);
