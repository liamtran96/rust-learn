---
title: 9.3 RefCell & Cell
tags: [rust, refcell, cell, interior-mutability]
---

# 9.3 `RefCell<T>` & `Cell<T>`

Rust's borrow rules are *usually* enforced at compile time. Sometimes you legitimately need **interior mutability** — the ability to mutate through an `&T`. These types shift the check to **runtime**.

## `RefCell<T>` — for non-`Copy` data

```rust
use std::cell::RefCell;

let cell = RefCell::new(vec![1, 2, 3]);

cell.borrow_mut().push(4);                // mutates through &cell
println!("{:?}", cell.borrow());          // reads through &cell
```

At runtime, `RefCell` tracks active borrows and **panics** if the rules are violated (e.g., two `borrow_mut()`s).

## `Cell<T>` — for `Copy` data only

```rust
use std::cell::Cell;

let c = Cell::new(5);
c.set(c.get() + 1);
```

No borrowing, just get/set — so no runtime panic risk. Useful for small primitives inside structs with shared ownership.

## When is this useful?

- **Mock objects / observers**: a shared struct that needs to record calls.
- **Memoization / lazy init**: fill a field on first access (or use `OnceCell`).
- **Graph-like data**: combined with `Rc`, lets nodes link and mutate each other.

## Anti-patterns

- Using `RefCell` to "silence" a borrow checker error you don't understand. Usually means the design is wrong — compile-time rules usually have a reason.
- Spreading `Rc<RefCell<T>>` through the codebase for convenience. It's infectious; prefer owning data through a clear hierarchy.

## Thread-safe cousins

| Single-thread | Thread-safe |
|---|---|
| `Cell<T>` | atomic types (`AtomicUsize`, …) |
| `RefCell<T>` | `Mutex<T>` / `RwLock<T>` |
| `OnceCell<T>` | `OnceLock<T>` |

## Related
- [[rc-arc|Rc / Arc]]
- [[../10-concurrency/shared-state|Mutex / RwLock]]
