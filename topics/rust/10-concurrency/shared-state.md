---
title: 10.3 Shared State — Mutex & RwLock
tags: [rust, mutex, rwlock, concurrency]
---

# 10.3 Shared State — `Mutex<T>` & `RwLock<T>`

## `Arc<Mutex<T>>` — the workhorse

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let data = Arc::new(Mutex::new(0u64));

let handles: Vec<_> = (0..8).map(|_| {
    let d = Arc::clone(&data);
    thread::spawn(move || {
        let mut guard = d.lock().unwrap();
        *guard += 1;
    })
}).collect();

for h in handles { h.join().unwrap(); }
println!("{}", *data.lock().unwrap());     // 8
```

- `lock()` returns a `LockResult<MutexGuard<T>>`. The guard implements `Deref`/`DerefMut` for `T`.
- The mutex **unlocks when the guard drops** (RAII).
- `unwrap()` handles *poisoning*: if a thread panics holding the lock, the mutex becomes "poisoned". `unwrap` crashes, or you can recover with `err.into_inner()`.

## Lock scope matters

Hold the lock as briefly as possible:

```rust
// ❌ lock held across the slow call
let mut g = state.lock().unwrap();
g.push(slow_io_call());

// ✅ compute outside, then lock briefly
let value = slow_io_call();
state.lock().unwrap().push(value);
```

## `RwLock<T>` — multiple readers OR one writer

```rust
use std::sync::RwLock;

let lock = RwLock::new(5);
{
    let r1 = lock.read().unwrap();
    let r2 = lock.read().unwrap();     // ✅ many readers OK
    assert_eq!(*r1 + *r2, 10);
}
*lock.write().unwrap() += 1;           // exclusive
```

Good for read-heavy workloads. For balanced / write-heavy, `Mutex` is often faster due to simpler internals.

## Picking a primitive

| Need | Use |
|---|---|
| Mutual exclusion | `Mutex<T>` |
| Many readers, few writers | `RwLock<T>` |
| Counters, flags | `AtomicUsize`, `AtomicBool`, … |
| Channel-style hand-off | `mpsc::channel` (see [[channels]]) |
| One-time init | `OnceLock<T>` |

## Common patterns

```rust
let cfg = Arc::new(Config::load());              // cheap to clone
let cfg_for_thread = Arc::clone(&cfg);

// Writer-exclusive guard pattern
let guard = state.write().unwrap();
let _unrelated_work = /* ... */;                 // don't put slow stuff here
drop(guard);                                     // explicit early drop if needed
```

## Deadlocks still possible

The type system prevents data races but not deadlocks (two threads each holding one lock, waiting for the other). Order your lock acquisitions consistently, or design with channels.

## Related
- [[threads|Threads]]
- [[channels|Channels]]
- [[send-sync|Send & Sync]]
