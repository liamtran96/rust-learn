---
title: 10.4 Send & Sync
tags: [rust, send, sync, concurrency]
---

# 10.4 `Send` & `Sync`

These two marker traits are how Rust decides at compile time what's safe across threads.

## Definitions

- `Send` — a type is safe to **transfer ownership** to another thread.
- `Sync` — a type is safe to **share a reference** (`&T`) with another thread. Equivalently: `T: Sync` iff `&T: Send`.

## Auto-derivation

Most types are `Send + Sync` automatically. The compiler infers from the type's components:
- A struct is `Send` if all its fields are `Send`.
- A struct is `Sync` if all its fields are `Sync`.

You rarely `impl Send` / `impl Sync` yourself — when you do, it's always `unsafe`.

## Types that are **not** `Send`

- `Rc<T>` — non-atomic refcount would race.
- `RefCell<T>` — non-atomic borrow counter.
- Raw pointers `*const T` / `*mut T`.

The parallel thread-safe versions are `Arc<T>`, `Mutex<T>` / `RwLock<T>` (with `Sync` bounds on inner data).

## Types that are `Send` but **not** `Sync`

- `Cell<T>` — safe to move to another thread, but giving out `&Cell<T>` would let multiple threads mutate concurrently.
- `RefCell<T>` — same reasoning.

## Why you see `T: Send + 'static` a lot

Thread spawns require the closure to be `Send + 'static`:
- `Send` — transferable to the thread.
- `'static` — contains no borrowed references (or only `'static` ones), because the thread may outlive any non-static scope.

Scoped threads (`thread::scope`) drop the `'static` requirement because the lifetime is bounded.

## Practical takeaway

If something *compiles*, it's data-race free. If it doesn't, the compiler is telling you which type bound is missing — usually a pointer to "you need `Arc` not `Rc`" or "you need `Mutex` around this."

## Related
- [[threads|Threads]]
- [[../09-smart-pointers/rc-arc|Rc vs Arc]]
