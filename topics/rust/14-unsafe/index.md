---
title: 14. Unsafe Rust & the Memory Model
tags: [rust, unsafe, memory-model, advanced]
---

# 14. Unsafe Rust & the Memory Model

This chapter is the deep version of [[../12-advanced/unsafe|Ch 12.1]]. It assumes you've shipped a Tauri app or equivalent — you know the language, you've read compiler errors for hours, and now you want to read `Vec`'s source and follow it.

## What this chapter is for

You'll learn to:
- Write `unsafe` blocks that are **sound** (not just "compile and pass tests").
- Read and write a `// SAFETY:` comment that survives code review.
- Reason about **aliasing**, **provenance**, and **validity** at the level the optimizer does.
- Run [Miri](https://github.com/rust-lang/miri) and fix the UB it finds.
- Wrap C code without making the whole program unsound.

You will *not* become a memory-model researcher. You'll become a developer who can ship 200 lines of `unsafe` without bricking a million safe ones above it.

## Prerequisites — non-negotiable

| You'll need | From chapter |
|---|---|
| References, lifetimes, the borrow checker | [[../02-ownership/index\|Ch 2]] |
| `Box`, `Rc`, `Arc`, `RefCell` | [[../09-smart-pointers/index\|Ch 9]] |
| Threads, `Send`, `Sync`, `Mutex` | [[../10-concurrency/index\|Ch 10]] |
| `Drop`, `Deref`, `DerefMut` | [[../09-smart-pointers/deref\|Ch 9.4]] |

If any of those are fuzzy, fix them first. Unsafe is unforgiving about prerequisites.

## Contents
- [[why-unsafe|14.1 What `unsafe` actually means]] — soundness, safety, validity
- [[raw-pointers|14.2 Raw pointers, `NonNull`, provenance]]
- [[aliasing|14.3 Aliasing & UB — the Stacked Borrows model]]
- [[uninit-and-cells|14.4 `MaybeUninit<T>` and `UnsafeCell<T>`]]
- [[miri|14.5 Miri — running, reading, fixing]]
- [[patterns|14.6 Common patterns — arena, linked list, Vec internals]]
- [[ffi-safety|14.7 FFI safety beyond the basics]]
- [[review-checklist|14.8 The unsafe-code review checklist]]

## The mental model in one paragraph

`unsafe` doesn't disable safety — **it transfers the proof obligation from the compiler to you**. Inside an `unsafe` block you can do things the compiler can't statically verify (deref a raw pointer, call an unsafe fn). In return, you promise that the operation upholds the invariants the rest of the language assumes. If you break that promise — even in code that compiles, even in code whose tests pass — you have introduced **undefined behavior (UB)**, and the compiler is licensed to do anything, including "the program appeared to work for a year and then deleted your data."

## Two invariants you must distinguish

| Invariant | Holds when… | Example |
|---|---|---|
| **Validity invariant** | Always — including inside unsafe code. Violating it is *immediately* UB. | A `bool` is `0` or `1`. A `&T` is non-null and aligned. A `char` is a valid Unicode scalar. |
| **Safety invariant** | Holds at the boundaries of safe code. May be temporarily violated *inside* unsafe code as long as it's restored before safe code observes it. | `Vec`'s `len <= cap`. `String`'s bytes are valid UTF-8. |

Get this distinction wrong and your `unsafe` is a time bomb.

## Official resources (always cite the source)

- 📕 [The Rustonomicon](https://doc.rust-lang.org/nomicon/) — *the* official guide to writing unsafe Rust. Read it cover to cover.
- 📒 [Reference: Behavior considered undefined](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) — the canonical UB list
- 🛠️ [Miri](https://github.com/rust-lang/miri) — UB detector, official tooling
- 📘 *Rust for Rustaceans* (Jon Gjengset) — Ch 9 "Unsafe Rust" is the best paid resource
- 📚 [Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/) — practical free book on unsafe via linked lists
- 🎓 [Crust of Rust: Atomics & Memory Ordering](https://www.youtube.com/watch?v=rMGWeSjctlY) — Jon Gjengset on YouTube
- 📄 [Stacked Borrows paper (POPL 2020)](https://plv.mpi-sws.org/rustbelt/stacked-borrows/) — the aliasing model
- 📄 [Tree Borrows](https://www.ralfj.de/blog/2023/06/02/tree-borrows.html) — experimental successor (Miri flag `-Zmiri-tree-borrows`)

## Exit criteria
- [ ] You can explain the difference between **validity** and **safety** invariants with a fresh example.
- [ ] You've implemented a `MyVec<T>` from raw pointers that passes Miri (Stacked and Tree Borrows).
- [ ] You can write a `// SAFETY:` comment that names the precondition, where it's established, and why it's preserved.
- [ ] You can audit an unsafe block in someone else's PR and either approve it or articulate the soundness hole.
- [ ] You know when **not** to reach for unsafe (most of the time).
