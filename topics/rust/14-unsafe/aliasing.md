---
title: 14.3 Aliasing & UB — Stacked Borrows
tags: [rust, unsafe, aliasing, ub, stacked-borrows]
---

# 14.3 Aliasing & UB — Stacked Borrows

> Authoritative: [Stacked Borrows paper (POPL 2020)](https://plv.mpi-sws.org/rustbelt/stacked-borrows/) · [Reference: behavior considered undefined](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) · [Tree Borrows](https://www.ralfj.de/blog/2023/06/02/tree-borrows.html) (experimental successor).

The borrow checker prevents aliasing violations *statically* in safe code. In unsafe code you can sidestep it by going through pointers — but the **aliasing model** still applies at runtime. Violate it and Miri will tell you. The optimizer will tell your users.

## The cardinal rule

> **For the duration that any reference (`&T` or `&mut T`) is "live", every access to its referent must go through that reference (or a child of it). A `&mut T` that is live must be the *only* path that can read or write the referent.**

That's the whole game. Stacked Borrows is the formal model that makes it precise.

## What "live" means

A reference is live from creation until **the last use of any pointer derived from it**. NLL (non-lexical lifetimes) for the borrow checker; "borrow stack" entries for Stacked Borrows.

```rust
let mut x = 0u32;
let p1 = &mut x;     // borrow A is born
*p1 = 1;             // last use of A's family — A can die here
let p2 = &mut x;     // borrow B is born; A is no longer live → fine
*p2 = 2;
```

If you use `p1` *after* `p2` was created, you've violated aliasing — even if the code compiles via raw pointers.

## What raw pointers do (and don't) escape

Casting a reference to `*mut T` and back **inherits the borrow**. The pointer is a *child* of the reference it came from. When the parent reference dies, the pointer can no longer access the underlying memory — even if it's still in scope.

```rust
let mut x = 5u32;
let r = &mut x;
let p: *mut u32 = r;   // p is a child of r
*r = 10;               // last use of r
unsafe { *p = 20; }    // ⚠️ UB: r's borrow is dead, so p can no longer access x
```

Miri catches this. The compiler does not.

To get a pointer that **outlives** the reference, derive it from `&raw mut x` (or from the original allocation), not from a reference:

```rust
let mut x = 5u32;
let p: *mut u32 = &raw mut x;  // not a child of any reference
{
    let r = &mut x;
    *r = 10;
}
unsafe { *p = 20; }            // OK — p has its own provenance
```

## The `&mut` exclusivity rule, restated for unsafe code

While a `&mut T` is live, **no other read or write** through any other path is allowed — not via a different reference, not via a pointer, not via the original variable. This is what lets the optimizer keep `*x` in a register.

Consequence: if you `transmute<&T, &mut T>`, you've already done UB the moment you make the call — the compiler can assume the original `&T` is observed nowhere.

## Common UB shapes you'll trip over

The full list: [Reference: behavior considered undefined](https://doc.rust-lang.org/reference/behavior-considered-undefined.html). The ones you'll actually hit:

| UB shape | How to spot it |
|---|---|
| **Dangling pointer access** | Pointer outlived its allocation (e.g., into a freed `Vec`'s buffer) |
| **Misaligned access** | `*(p as *const u64)` where `p` was a `*const u8` from arbitrary bytes |
| **Reading uninitialized memory** | `Box::new(MaybeUninit::<u32>::uninit()).assume_init_read()` without writing first |
| **Mutable aliasing** | Two `&mut T` to the same place; or `&T` and `&mut T` overlapping |
| **Type confusion** | `transmute` between types with incompatible validity invariants |
| **Data race** | Two threads access same memory, ≥1 write, no synchronization, not atomic |
| **Calling undefined function** | C function with wrong signature; FFI ABI mismatch |
| **Unwinding through `extern "C"`** | Panic crosses an FFI boundary that doesn't expect it |
| **Invalid value at validity-typed location** | `transmute<u8, bool>(2)`; null `&T` |
| **Out-of-bounds offset** | `ptr.add(big)` past one-past-end |

## Stacked Borrows in 60 seconds

When you create a reference (or borrow-derived pointer), Stacked Borrows pushes a "tag" onto a per-location *borrow stack*. Each access must use the tag at the top of the stack (or a child). Creating a new borrow pops/invalidates everything above it.

Key consequences:
- A child can be used while the parent is on the stack.
- Re-using the parent invalidates all children above it.
- Two siblings (peer borrows) cannot both be used; reusing one invalidates the other.

You don't write code thinking about the stack. You write code following the cardinal rule. The stack is what Miri is checking.

## Tree Borrows — what's coming

[Tree Borrows](https://www.ralfj.de/blog/2023/06/02/tree-borrows.html) replaces the stack with a tree, fixing some over-strict rejections (≈54% fewer rejections on real crates). It's **experimental** as of 2026 — Miri exposes it via `cargo +nightly miri test -- -Zmiri-tree-borrows`.

If your code passes both Stacked and Tree Borrows under Miri, you're in great shape regardless of which becomes the official model.

## How to keep aliasing sane in your unsafe code

1. **Don't hold references and pointers to the same thing in scope.** Pick one model.
2. **Never have two live `&mut T` to the same data.** Period.
3. **Use `&raw const` / `&raw mut`** when you need a pointer that outlives nearby references.
4. **Don't `transmute` between mutability classes.** Use `UnsafeCell` for shared mutation.
5. **Run Miri**, then `-Zmiri-tree-borrows` Miri. Both must be clean.
6. **Document**: every type with raw-pointer fields should explain in its module docs which pointers may alias and why.

## A concrete worked example: aliasing two slices

```rust
// Sound: split_at_mut returns two non-overlapping &mut slices
let mut v = vec![1, 2, 3, 4];
let (a, b) = v.split_at_mut(2);
a[0] = 10;
b[0] = 20;
```

How does std implement this without UB? It uses raw pointers + `&raw mut`, derived directly from the buffer's allocation, never aliasing a single `&mut [T]` over both halves. Read `<[T]>::split_at_mut`'s source — it's an excellent unsafe example.

## Exit criteria
- [ ] You can state the cardinal rule from memory.
- [ ] You can explain why `let p = &mut x as *mut _;` followed by reusing `&mut x` and then `*p` is UB.
- [ ] You know the difference between `&raw mut x` and `&mut x as *mut _`.
- [ ] You can list 5 UB shapes without looking them up.
- [ ] You're comfortable saying "Miri must pass" as a hard rule for unsafe code.
