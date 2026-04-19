---
title: 9.1 Box<T>
tags: [rust, box, heap]
---

# 9.1 `Box<T>`

The simplest smart pointer: a heap-allocated `T` with single ownership.

## Uses

### 1. Put large values on the heap

```rust
let huge = Box::new([0u8; 1_000_000]);       // one big buffer on heap
```

### 2. Enable recursive types

Without indirection, a recursive type would have infinite size:

```rust
enum List<T> {
    Cons(T, Box<List<T>>),                   // Box breaks the infinite recursion
    Nil,
}
```

### 3. Own trait objects

```rust
let widgets: Vec<Box<dyn Draw>> = vec![Box::new(Button), Box::new(Slider)];
```

### 4. Signal intent for heap allocation

Even when not required, `Box` communicates "this lives on the heap."

## Behavior

- `Deref` to `T` — use it like `T` for most purposes (`b.method()`, `*b`).
- Dropping a `Box` frees its contents.
- `Box::new(x)` allocates + moves `x` into the heap.
- `Box::leak(b)` turns ownership into `&'static mut T` — escape hatch for long-lived data.

## Zero-cost

A `Box<T>` is just a pointer — same size as `*const T`. The abstraction is free.

## Related
- [[rc-arc|Rc/Arc — shared heap ownership]]
- [[../03-types-and-traits/trait-objects|Trait objects]]
