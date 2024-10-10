use std::cell::UnsafeCell;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;
use std::time::Duration;

static VAR: SpinLock<u32> = SpinLock::new(0);

fn main() {
    let mut handles = vec![];
    for _ in 1..=5 {
        handles.push(thread::spawn(move || {
            let mut lock = VAR.lock();
            *lock = *lock + 1;
            println!("{:?}", *lock);
            thread::sleep(Duration::from_secs(1));
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[derive(Debug)]
struct SpinLock<T: Debug> {
    inner: UnsafeCell<T>,
    lock: AtomicU32,
}

unsafe impl Send for SpinLock<u32> {}
unsafe impl Sync for SpinLock<u32> {}

impl<T: Debug> SpinLock<T> {
    const fn new(inner: T) -> Self {
        Self {
            inner: UnsafeCell::new(inner),
            lock: AtomicU32::new(0),
        }
    }

    fn lock(&self) -> LockGuard<'_, T> {
        loop {
            if self
                .lock
                .compare_exchange(0, 42, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                break;
            }
            thread::sleep(Duration::from_millis(1));
        }

        LockGuard {
            inner: &self.inner,
            lock: &self.lock,
        }
    }
}

#[derive(Debug)]
struct LockGuard<'a, T> {
    inner: &'a UnsafeCell<T>,
    lock: &'a AtomicU32,
}

impl Deref for LockGuard<'_, u32> {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.inner.get()) }
    }
}

impl DerefMut for LockGuard<'_, u32> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut (*self.inner.get()) }
    }
}

impl<T> Drop for LockGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.store(0, Ordering::SeqCst);
    }
}
