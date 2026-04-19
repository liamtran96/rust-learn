---
title: 9. Smart Pointers
tags: [rust, smart-pointers]
---

# 9. Smart Pointers

Types that *behave* like pointers but add capabilities: heap allocation, shared ownership, runtime-checked borrowing, custom drop behavior.

## Contents
- [[box|9.1 Box<T> — heap allocation]]
- [[rc-arc|9.2 Rc<T> / Arc<T> — shared ownership]]
- [[refcell|9.3 RefCell / Cell — interior mutability]]
- [[deref|9.4 Deref & Drop]]

## Decision tree

```
Need heap?              → Box
Need shared ownership?  → Rc  (single-threaded)
                         Arc (cross-thread)
Need mutation through a shared reference? → RefCell (runtime checked, single-thread)
                                            Mutex / RwLock (cross-thread)
Need runtime-sized or self-referential data? → Box / Rc / Arc
```

## Key ideas
- **Box<T>** — puts a value on the heap. You still have single ownership.
- **Rc<T>** — reference-counted; many owners, same-thread only.
- **Arc<T>** — atomic reference counter; thread-safe version of `Rc`.
- **RefCell<T>** — defers Rust's borrow checking to **runtime**. Panics on violation.
- **Cell<T>** — for `Copy` types; interior mutation via get/set.

## Combinations you'll see

| Pattern | Meaning |
|---|---|
| `Rc<RefCell<T>>` | Multiple owners, each can mutate (single-thread) |
| `Arc<Mutex<T>>` | Multiple threads, exclusive access to mutate |
| `Arc<RwLock<T>>` | Multiple threads, many readers or one writer |
| `Box<dyn Trait>` | Owned trait object |

## Exit criteria
- [ ] You can use `Box` to enable a recursive enum like a linked list or expression tree.
- [ ] You can explain why `Rc<T>` is not thread-safe.
- [ ] You can use `Rc<RefCell<T>>` to model a shared-mutable graph.
- [ ] You know why `Arc<Mutex<T>>` is the go-to for cross-thread mutation.
