## Stoplight
Is a small library for stoppable threads/tasks.

```rust
use stoplight::Thread;
use std::sync::atomic::{AtomicBool, Ordering};

// spawn our task, this creates a new OS thread.
let th = Thread::spawn(|stop| {
    while !stop.load(Ordering::Relaxed) {}
    42
});

// join() signals the thread to stop, and then returns its return value.
assert_eq!(th.join().unwrap(), 42);
```
