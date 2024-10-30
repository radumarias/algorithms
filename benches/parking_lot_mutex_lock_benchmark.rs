use std::sync::{Arc};
use criterion::{Criterion, criterion_group, criterion_main};
use parking_lot::Mutex;

fn spinlock_benchmark(c: &mut Criterion) {
    let lock = Arc::new(Mutex::new(0));

    c.bench_function("parking_lot_mutex_lock_benchmark", |b| {
        b.iter(|| {
            let cloned = Arc::clone(&lock);

            let guard = cloned.lock();
            *guard
        });
    });
}

criterion_group!(benches, spinlock_benchmark);
criterion_main!(benches);
