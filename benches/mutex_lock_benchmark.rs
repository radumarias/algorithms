use std::sync::{Arc, Mutex};
use criterion::{Criterion, criterion_group, criterion_main};

fn spinlock_benchmark(c: &mut Criterion) {
    let lock = Arc::new(Mutex::new(0));

    c.bench_function("mutex_lock_benchmark", |b| {
        b.iter(|| {
            let cloned = Arc::clone(&lock);

            let guard = cloned.lock().unwrap();
            *guard
        });
    });
}

criterion_group!(benches, spinlock_benchmark);
criterion_main!(benches);
