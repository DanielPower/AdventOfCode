//! A small bounded worker pool.

use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Apply `f` to every item using at most `jobs` threads; results keep input order.
pub fn parallel_map<T: Sync, R: Send>(
    items: &[T],
    jobs: usize,
    f: impl Fn(&T) -> R + Sync,
) -> Vec<R> {
    let next = AtomicUsize::new(0);
    let slots: Mutex<Vec<Option<R>>> = Mutex::new((0..items.len()).map(|_| None).collect());
    let workers = jobs.max(1).min(items.len());
    std::thread::scope(|s| {
        for _ in 0..workers {
            s.spawn(|| {
                loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    let Some(item) = items.get(i) else { break };
                    let r = f(item);
                    slots.lock().unwrap()[i] = Some(r);
                }
            });
        }
    });
    slots
        .into_inner()
        .unwrap()
        .into_iter()
        .map(|r| r.expect("every slot filled"))
        .collect()
}

pub fn default_jobs() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(2)
}
