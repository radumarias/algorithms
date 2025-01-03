use criterion::{criterion_group, criterion_main, Criterion};
use std::sync::Arc;

use add_two_numbers::spinlock::SpinLock;

fn spinlock_benchmark(c: &mut Criterion) {
    let spin_lock = Arc::new(SpinLock::new(0));

    c.bench_function("spinlock_lock_benchmark", |b| {
        b.iter(|| {
            let cloned = Arc::clone(&spin_lock);

            let guard = cloned.lock();
            *guard
        });
    });
}

criterion_group!(benches, spinlock_benchmark);
criterion_main!(benches);
