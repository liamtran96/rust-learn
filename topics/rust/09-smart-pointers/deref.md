---
title: 9.4 Deref & Drop
tags: [rust, deref, drop]
---

# 9.4 `Deref` & `Drop`

Two small traits that power smart pointers.

## `Deref` — make your type act like a pointer

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

let b = MyBox(String::from("hi"));
let s: &str = &b;                 // deref coercion: &MyBox<String> → &String → &str
```

**Deref coercion** is why you can pass `&String` to a function expecting `&str`, or `&Vec<T>` to one expecting `&[T]`. The compiler inserts `.deref()` calls.

For mutable context, implement `DerefMut` too.

## `Drop` — RAII cleanup

```rust
struct Connection { /* ... */ }

impl Drop for Connection {
    fn drop(&mut self) { /* close socket, release lock, etc. */ }
}
```

When a value goes out of scope, Rust calls `drop()` automatically. This is how:
- `Vec` frees its heap allocation.
- `File` closes its file descriptor.
- `MutexGuard` unlocks the mutex.

You rarely call `drop()` yourself — it's invoked implicitly. To drop early, call `std::mem::drop(value)` to move it into oblivion.

### Drop order
Local variables drop in **reverse order of declaration**. Fields drop in declaration order. This matters when one field depends on another during cleanup.

## Rule of thumb
Implement `Drop` only for types holding **external resources** (files, sockets, locks, `unsafe` manual allocations). For pure Rust data, the compiler handles everything.

## Related
- [[box|Box]]
- [[rc-arc|Rc / Arc]]
