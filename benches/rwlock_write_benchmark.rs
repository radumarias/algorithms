use std::sync::{Arc, RwLock};
use criterion::{Criterion, criterion_group, criterion_main};

fn spinlock_benchmark(c: &mut Criterion) {
    let lock = Arc::new(RwLock::new(0));

    c.bench_function("rwlock_write_benchmark", |b| {
        b.iter(|| {
            let cloned = Arc::clone(&lock);

            let guard = cloned.write().unwrap();
            *guard
        });
    });
}

criterion_group!(benches, spinlock_benchmark);
criterion_main!(benches);
