---
title: 2.1 Ownership Rules
tags: [rust, ownership]
---

# 2.1 Ownership Rules

## Move semantics for heap types

```rust
let s1 = String::from("hello");
let s2 = s1;               // s1 is MOVED into s2
println!("{s1}");          // ❌ error: value borrowed after move
```

After the move, `s1` is uninitialized — the compiler will refuse further use. No double-free, no dangling, no runtime cost.

## `Copy` types are *copied*, not moved

```rust
let x = 5;
let y = x;
println!("{x} {y}");       // ✅ both usable — i32 implements Copy
```

Rules of thumb:
- Implement `Copy` **only** if your type is a small bit-for-bit copy with no resources (numbers, bools, `char`, small fixed-size structs of `Copy` types, shared references `&T`).
- `String`, `Vec`, `Box`, `File`, `Mutex<T>` — **not** `Copy`. They own heap/OS resources.

## `Clone` — explicit deep copy

```rust
let s1 = String::from("hi");
let s2 = s1.clone();       // explicit heap allocation
println!("{s1} {s2}");     // ✅
```

`.clone()` is cheap for some types, expensive for others. **It is always explicit** — Rust never deep-clones silently.

## Function calls move (or copy)

```rust
fn consume(s: String) { println!("{s}"); }

let s = String::from("hi");
consume(s);
// s is gone here
```

Same rules as assignment. If you want the caller to keep ownership, **borrow** instead (next note).

## Returning ownership

```rust
fn take_and_give(s: String) -> String { s }
```

Returning is a move back out. Common pattern for type-state transformations.

## Drop

When an owner leaves scope, Rust calls the `Drop::drop` destructor, then frees memory:

```rust
{
    let s = String::from("hi");   // allocated on heap
}                                 // s goes out of scope → drop runs → memory freed
```

You rarely implement `Drop` yourself — it's for resource cleanup (files, locks, sockets) where the standard library types handle it already.

## Why this prevents bugs for free
- **Use-after-free**: impossible — the owner can't access the value after dropping it.
- **Double-free**: impossible — only one owner.
- **Memory leaks**: *possible* but rare (`std::mem::forget`, `Rc` cycles).
- **Data races**: prevented by combining this with `Send`/`Sync` — see [[../10-concurrency/send-sync|Send & Sync]].

## Related
- [[borrowing|Borrowing — the escape hatch]]
- [[../09-smart-pointers/rc-arc|Rc/Arc for shared ownership]]
