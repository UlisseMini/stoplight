#![deny(missing_docs)]

//! Stoplight is a small library for stoppable threads/tasks.
//!```
//! use stoplight::Thread;
//! use std::sync::atomic::{AtomicBool, Ordering};
//!
//! // spawn our task, this creates a new OS thread.
//! let th = Thread::spawn(|stop| {
//!     while !stop.load(Ordering::Relaxed) {}
//!     42
//! });
//!
//! // stop() signals the thread to stop, and then join returns its return value.
//! th.stop();
//! assert_eq!(th.join().unwrap(), 42);
//!```

use std::any::Any;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

/// Handle to a stoppable thread.
pub struct Thread<T> {
    jh: JoinHandle<T>,
    stop: Arc<AtomicBool>,
}

impl<T> Thread<T>
where
    T: Send + 'static,
{
    /// Spawn a new job with cancelation.
    pub fn spawn<F>(f: F) -> Thread<T>
    where
        F: FnOnce(Arc<AtomicBool>) -> T + Send + 'static,
    {
        let stop = Arc::new(AtomicBool::new(false));

        Thread {
            stop: stop.clone(),
            jh: thread::spawn(move || f(stop)),
        }
    }

    /// Join waits for the thread to exit then returns the return value.
    pub fn join(self) -> Result<T, Box<(dyn Any + Send + 'static)>> {
        self.jh.join()
    }

    /// Signal the Thread to stop, NOTE: This does not ensure the thread has stopped
    /// but only that the signal has been sent.
    // TODO: Clean up type signature of Result<T, E> (copied off compile errors)
    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_busy_loop() {
        let th = Thread::spawn(|stop| {
            thread::sleep(Duration::from_millis(300));
            while !stop.load(Ordering::Relaxed) {}
            42
        });

        th.stop();
        assert_eq!(th.join().unwrap(), 42);
    }
}
