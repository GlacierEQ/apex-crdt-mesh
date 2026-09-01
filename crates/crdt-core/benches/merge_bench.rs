use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crdt_core::GCounter;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("gcounter_merge_1m", |b| {
        let mut c1 = GCounter::new("A");
        let mut c2 = GCounter::new("B");
        
        for i in 0..100 {
            c1.increment(1);
            c2.increment(2);
            let mut other = GCounter::new(&format!("Node{}", i));
            other.increment(i as u64);
            c2.merge(&other);
        }
        
        b.iter(|| {
            let mut c1_clone = black_box(c1.clone());
            c1_clone.merge(black_box(&c2));
            c1_clone
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
