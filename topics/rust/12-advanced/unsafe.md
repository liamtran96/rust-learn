---
title: 12.1 Unsafe Rust
tags: [rust, unsafe]
---

# 12.1 Unsafe Rust

`unsafe` is **not** "turn off all safety." It's "I'm promising the compiler this block upholds the invariants it can't check."

## What `unsafe` lets you do

Inside an `unsafe` block, you can:
1. Dereference a raw pointer.
2. Call an `unsafe fn` or `unsafe` trait method.
3. Access or modify a `static mut` variable.
4. Implement an `unsafe` trait (e.g., `Send`, `Sync` manually).
5. Access fields of a `union`.

Everything else is still checked normally.

## Raw pointers

```rust
let mut n = 5;
let r1 = &n as *const i32;       // can have many
let r2 = &mut n as *mut i32;     // no exclusivity guarantee!

unsafe {
    println!("{}", *r1);
    *r2 = 10;
}
```

Raw pointers don't imply ownership, aliasing, or validity. **You** are responsible for: not dereferencing null / dangling, no aliasing that violates `&mut` exclusivity, keeping data alive, alignment, initialization.

## Typical legitimate uses

- **FFI** — bridging to C.
- **Implementing primitives** — custom `Vec`, arena, lock-free queue.
- **Performance micro-opts** — when profiling shows bounds checks dominating a hot loop (usually `get_unchecked`).
- **Self-referential data structures** — rare, often via `Pin` + `UnsafeCell`.

## Guidelines

- Keep `unsafe` **small**. Wrap it in a safe API that upholds invariants for its callers.
- Every `unsafe` block deserves a `// SAFETY:` comment explaining **why** it's sound.
- Run [`miri`](https://github.com/rust-lang/miri) (`cargo +nightly miri test`) — detects undefined behavior in test runs.
- Read *The Rustonomicon* before shipping unsafe code: <https://doc.rust-lang.org/nomicon/>.

## A word on `unsafe` in std

Almost every safe abstraction you use (`Vec`, `Mutex`, `String`, `HashMap`) is built on `unsafe` inside. That's fine — safety is about the **public API**, not the absence of `unsafe` anywhere.

## Going deeper

This page is a 1-screen overview. For the full deep dive — soundness, Stacked Borrows, `MaybeUninit`, `UnsafeCell`, Miri, common patterns, FFI safety, and the review checklist — see [[../14-unsafe/index|Chapter 14 — Unsafe Rust & the Memory Model]].

## Related
- [[ffi|FFI]]
- [[macros|Macros]]
- [[../14-unsafe/index|Chapter 14 — Unsafe deep dive]]
