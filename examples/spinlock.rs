use std::cell::UnsafeCell;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::sync::Arc;
use std::time::Duration;
use rand::{Rng, thread_rng};

fn main() {
    let spin_lock = Arc::new(SpinLock::new(0));

    let mut handles = vec![];
    for _ in 1..=5 {
        let cloned = spin_lock.clone();
        handles.push(thread::spawn(move || {
            let mut guard = cloned.lock();
            *guard = *guard + 1;
            println!("{:?}", *guard);
            thread::sleep(Duration::from_millis(thread_rng().gen_range(42..1024)));
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[derive(Debug)]
pub struct SpinLock<T: Debug> {
    inner: UnsafeCell<T>,
    lock: AtomicBool,
}

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
            if !self
                .lock
                .swap(true, Ordering::Acquire)
            {
                break;
            }
            std::hint::spin_loop();
        }

        LockGuard {
            inner: &self,
        }
    }
}

#[derive(Debug)]
pub struct LockGuard<'a, T: Debug> {
    inner: &'a SpinLock<T>,
}

impl Deref for LockGuard<'_, u32> {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.inner.inner.get()) }
    }
}

impl DerefMut for LockGuard<'_, u32> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut (*self.inner.inner.get()) }
    }
}

impl<T: Debug> Drop for LockGuard<'_, T> {
    fn drop(&mut self) {
        self.inner.lock.store(false, Ordering::Release);
    }
}