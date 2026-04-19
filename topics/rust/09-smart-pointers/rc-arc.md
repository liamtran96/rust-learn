---
title: 9.2 Rc<T> & Arc<T>
tags: [rust, rc, arc, shared-ownership]
---

# 9.2 `Rc<T>` & `Arc<T>`

When multiple parts of your program need to **own** the same value.

## `Rc` — single-threaded

```rust
use std::rc::Rc;

let a = Rc::new(String::from("shared"));
let b = Rc::clone(&a);                   // cheap — bumps refcount, no deep copy
let c = Rc::clone(&a);

println!("count = {}", Rc::strong_count(&a));    // 3
```

All `Rc` handles see the same underlying data. When the last one drops, the value is freed.

Key properties:
- **Not thread-safe** — the refcount uses non-atomic ops.
- Contents are **immutable by default**. Combine with `RefCell` for mutation.
- `Rc::clone` is a refcount bump, not a deep copy — very cheap.

## `Arc` — multi-threaded

Same API, but the refcount uses atomics. Pay a tiny performance cost for thread safety.

```rust
use std::sync::Arc;

let data = Arc::new(vec![1, 2, 3]);

let handles: Vec<_> = (0..4).map(|_| {
    let data = Arc::clone(&data);
    std::thread::spawn(move || {
        println!("{:?}", data);
    })
}).collect();

for h in handles { h.join().unwrap(); }
```

## `Weak` — cycle-breaker

`Rc<T>` / `Arc<T>` can leak memory via reference cycles. Use `Weak<T>` for the **back-edge** in a cyclic structure (child → parent, observers, caches):

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

let parent = Rc::new(Node { parent: RefCell::new(Weak::new()), children: Default::default() });
let child  = Rc::new(Node { parent: RefCell::new(Rc::downgrade(&parent)), children: Default::default() });
parent.children.borrow_mut().push(Rc::clone(&child));
```

Upgrading a `Weak` gives `Option<Rc<T>>` — `None` if the target has been dropped.

## Decision table

| Scenario | Pick |
|---|---|
| Single-thread, shared read | `Rc<T>` |
| Single-thread, shared read+write | `Rc<RefCell<T>>` |
| Multi-thread, shared read | `Arc<T>` |
| Multi-thread, shared write | `Arc<Mutex<T>>` or `Arc<RwLock<T>>` |

## Related
- [[refcell|RefCell]]
- [[../10-concurrency/shared-state|Mutex / RwLock]]
