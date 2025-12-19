use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

fn bench(c: &mut Criterion) {
    c.bench_function("dissimilar::diff", |b| {
        let document1 = fs::read_to_string("benches/document1.txt").unwrap();
        let document2 = fs::read_to_string("benches/document2.txt").unwrap();
        b.iter(|| dissimilar::diff(&document1, &document2));
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
