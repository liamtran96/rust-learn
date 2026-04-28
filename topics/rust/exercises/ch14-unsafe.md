---
title: Ch 14 — Unsafe & Memory Model Exercises
tags: [rust, unsafe, exercises, miri]
---

# Ch 14 — Unsafe & Memory Model Exercises

A graded ladder. **Run Miri after every exercise.** If Miri fails, you're not done.

```bash
cargo +nightly miri test
# then also:
MIRIFLAGS="-Zmiri-tree-borrows -Zmiri-strict-provenance" cargo +nightly miri test
```

## Setup gate

1. **Install Miri.** `rustup +nightly component add miri`. Run `cargo +nightly miri --version` and screenshot it.
2. **Read** the [Rustonomicon Introduction](https://doc.rust-lang.org/nomicon/intro.html) and the [Reference: behavior considered undefined](https://doc.rust-lang.org/reference/behavior-considered-undefined.html). Take notes; you'll re-read these.

## Mental model warm-up

3. **Validity vs safety** — write a one-page note (in `journal.md`) with a *fresh* example of each invariant. Bonus: pick a type from std (`Vec`, `String`, `BTreeMap`) and list its safety invariants from the docs.
4. **Unsoundness in 5 lines** — write a *safe* function whose implementation contains UB. Have someone else read its signature and predict whether they'd trust it. Document the lesson.
5. **Read 3 SAFETY comments in std** — find them via `rg "// SAFETY:" $(rustc --print sysroot)/lib/rustlib/src` and write down what each one is upholding.

## Raw pointers

6. **Round-trip references** — write `fn double(x: &mut u32)` using only raw pointers internally (and an `unsafe` block). Run Miri. Confirm clean.
7. **`&raw mut` vs `&mut x as *mut _`** — construct an example where one is sound and the other is UB; show Miri catches the UB version.
8. **NonNull / dangling** — implement `Empty<T>(NonNull<T>)` that holds no T but is properly aligned. Make it `Send` if `T: Send`. Justify with a `// SAFETY:` impl.
9. **Pointer arithmetic boundary** — write a function that walks a slice with `ptr.add(i)`. Test that walking exactly to one-past-end is fine; walking beyond is UB (Miri will catch).

## Aliasing

10. **Trigger Stacked Borrows UB** — write code that creates `&mut x`, casts to `*mut`, reuses `&mut x`, then writes through the pointer. Run Miri. Read its output. Then fix it using `&raw mut x`.
11. **Two non-overlapping mut slices** — implement your own `split_at_mut` for `[T]`. Cover odd/even/empty cases. Miri-clean.
12. **Spot the bug** — read the snippet below. Predict whether it's UB. Run under Miri to confirm.
    ```rust
    fn aliased() {
        let mut x = 0u32;
        let r1: &u32 = &x;
        let r2: *mut u32 = &mut x; // <-- here?
        unsafe { *r2 = 1; }
        println!("{}", *r1);
    }
    ```

## `MaybeUninit` / `UnsafeCell`

13. **Array of MaybeUninit** — initialize a `[u32; N]` from a closure `f(i: usize) -> u32` without ever holding an uninit `[u32; N]`. Handle a panic in `f` by dropping any partially-initialized prefix.
14. **Custom `Cell<T>`** — implement a working `Cell<T: Copy>` from scratch using `UnsafeCell`. Provide `get`, `set`, `replace`. Justify why your unsafe is sound. Miri-clean under both Stacked and Tree Borrows.
15. **Read `Cell` and `RefCell` source** — open `core/src/cell.rs` in your toolchain and read the `RefCell` impl. Note where `UnsafeCell::get()` is called and what each `// SAFETY:` says.

## Miri

16. **Make Miri yell** — write code with a deliberate, subtle aliasing bug that compiles and passes regular tests. Confirm Miri catches it.
17. **Diagnose a Miri error** — copy a Miri error from one of your earlier exercises (or from `https://github.com/rust-lang/miri/issues`). Walk through what each phrase means in your journal.
18. **CI integration** — add a Miri job to one of your projects (one of your earlier crates, or your Tauri capstone's pure-Rust crate). PR or branch is fine. Confirm it runs green.

## Patterns

19. **Bump arena, no-Drop variant** — implement the [[../14-unsafe/patterns|14.6]] arena. Use it to allocate 100 `u64`s. Verify Miri-clean.
20. **Bump arena, with Drop** — extend the arena to track destructors so non-Copy types are dropped on arena drop. Test with `String`. Miri-clean.
21. **`MyVec<T>`** — implement the `MyVec<T>` skeleton from [[../14-unsafe/patterns|14.6]]. Verify Miri-clean for `T = u32`, `T = String`, `T = ()` (a ZST). Stress: push 10k, pop 5k, drop.
22. **Singly-linked stack** — implement [[../14-unsafe/patterns|14.6]]'s stack. Add an `IntoIter` that consumes the stack node by node. Miri-clean.
23. **Compare to std** — implement `<[T]>::split_first_mut` from scratch. Compare your version to std's source. What did std do differently?

## FFI safety

24. **Wrap `libc::malloc`/`free`** — write a `pub struct CBuf { ptr: *mut u8, len: usize }` that allocates via `libc::malloc` and frees via `libc::free`. Implement `Drop`. Provide a safe `as_slice` and `as_mut_slice`. Verify under Miri (with `-Zmiri-disable-isolation` so libc is allowed).
25. **`catch_unwind` boundary** — write an `extern "C"` function that calls a Rust closure that may panic. Use `catch_unwind` to convert panics to `i32` error codes. Test by panicking deliberately.

## Audit

26. **Audit your Tauri capstone** — find every `unsafe` block (if any). Write or improve the `// SAFETY:` comment. Run Miri on the pure-Rust crate (Miri can't run Tauri itself, but it can run your domain logic).
27. **Audit a real crate** — pick one with `unsafe` you depend on (e.g. `bytes`, `parking_lot`, `crossbeam-queue`). Read three of its `unsafe` blocks. Write down how each `// SAFETY:` matches the operation. Are you convinced?

## Capstone

28. **Pick one and ship it Miri-clean:**
    - **A.** A typed arena that returns `&mut T` and runs destructors on drop, in the style of `typed-arena` but yours.
    - **B.** A bounded SPSC ring buffer using `MaybeUninit` + atomics. Sound under TSan and Miri.
    - **C.** A FFI binding to a small C library you choose (under 1 KLOC). Provide a fully safe Rust API; document each `unsafe` block.

29. **Retrospective** — write 5 bullets in `journal.md`: which UB shape surprised you most; which Miri error took longest to fix; which std API you started reading; what unsafe you wrote that you're keeping vs deleting; and what specific question you'd ask Ralf Jung if you could.

## Checkpoint

You're done with Ch 14 when:
- You've shipped exercise 28 (capstone) and it's Miri-clean under Stacked + Tree Borrows + strict provenance.
- You can review someone else's unsafe code using the [[../14-unsafe/review-checklist|review checklist]] without notes.
- You **automatically** reach for `Cell`/`RefCell`/`Mutex`/`bumpalo`/`slotmap` before considering raw `unsafe`.
