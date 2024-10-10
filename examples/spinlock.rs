use std::cell::UnsafeCell;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use rand::{Rng, thread_rng};

static VAR: SpinLock<u32> = SpinLock::new(0);

fn main() {
    let mut handles = vec![];
    for _ in 1..=5 {
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(thread_rng().gen_range(0..1000)));
            let mut lock = VAR.lock();
            *lock = *lock + 1;
            println!("{:?}", *lock);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[derive(Debug)]
struct SpinLock<T: Debug> {
    inner: UnsafeCell<T>,
    lock: AtomicBool,
}

unsafe impl Send for SpinLock<u32> {}
unsafe impl Sync for SpinLock<u32> {}

impl<T: Debug> SpinLock<T> {
    const fn new(inner: T) -> Self {
        Self {
            inner: UnsafeCell::new(inner),
            lock: AtomicBool::new(false),
        }
    }

    fn lock(&self) -> LockGuard<'_, T> {
        loop {
            if self
                .lock
                .swap(true, Ordering::SeqCst)
            {
                break;
            }
            thread::yield_now();
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
    lock: &'a AtomicBool,
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
        self.lock.store(false, Ordering::SeqCst);
    }
}
