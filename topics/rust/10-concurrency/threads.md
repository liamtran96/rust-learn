---
title: 10.1 Threads
tags: [rust, threads]
---

# 10.1 Threads

## Spawning

```rust
use std::thread;

let handle = thread::spawn(|| {
    for i in 1..5 { println!("thread: {i}"); }
});

for i in 1..3 { println!("main: {i}"); }

handle.join().unwrap();     // wait for thread to finish; propagate panics
```

- `spawn` returns a `JoinHandle<T>`. `T` is whatever the closure returned.
- If the main thread exits, all other threads are killed — always `join()` what you care about.

## `move` — required for most real uses

```rust
let v = vec![1, 2, 3];
thread::spawn(move || {     // ← move transfers ownership of v into the thread
    println!("{v:?}");
});
```

Without `move`, the closure would *borrow* from the outer scope, but the thread may outlive that scope — compiler refuses.

## Scoped threads (since 1.63)

Threads that are guaranteed to finish before the function returns can borrow non-`'static` data:

```rust
let v = vec![1, 2, 3];
thread::scope(|s| {
    s.spawn(|| println!("{v:?}"));        // borrow — no move needed
    s.spawn(|| println!("{v:?}"));
});                                       // joins all spawned threads here
```

Use whenever the threads' lifetime is bounded by the scope — much more ergonomic than juggling `Arc`s.

## When **not** to use threads

- Spawning per-request for a server → use async. Threads are relatively heavy (~MB of stack each).
- Embarrassingly parallel CPU work → use [`rayon`](https://docs.rs/rayon): `data.par_iter().map(...).sum()`.

## Thread-local data

```rust
thread_local! {
    static COUNTER: std::cell::Cell<u32> = std::cell::Cell::new(0);
}

COUNTER.with(|c| c.set(c.get() + 1));
```

## Related
- [[channels|Channels]]
- [[shared-state|Shared state]]
- [[../11-async/index|Async — for high-concurrency IO]]
