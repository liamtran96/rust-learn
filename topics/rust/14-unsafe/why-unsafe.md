---
title: 14.1 What `unsafe` Actually Means
tags: [rust, unsafe, soundness]
---

# 14.1 What `unsafe` Actually Means

> Authoritative: [Rustonomicon — Meet Safe and Unsafe](https://doc.rust-lang.org/nomicon/meet-safe-and-unsafe.html).

`unsafe` is **not** "turn off safety." Three precise statements:

1. **`unsafe` is permission to use 5 specific superpowers** the borrow checker normally forbids.
2. **`unsafe` is a contract**: in exchange for those powers, *you* prove the operation upholds invariants the language assumes.
3. **`unsafe` is local**: code outside an `unsafe` block must remain safe, no matter what unsafe code did. That's what *soundness* means.

## The 5 superpowers

Inside an `unsafe { ... }` block — and *only* inside one — you may:
1. **Dereference a raw pointer** (`*const T`, `*mut T`).
2. **Call an `unsafe fn`** (or its method form).
3. **Implement an `unsafe` trait** (e.g. `Send`, `Sync` manually).
4. **Read or write a `static mut`** (rare; usually use `Mutex` or atomics).
5. **Access fields of a `union`**.

Everything else (the borrow checker, type checking, `match` exhaustiveness) is still enforced.

## Soundness vs safety

These two words look similar; they aren't.

| Term | Meaning |
|---|---|
| **Safe API** | The function's signature can be called by safe code with any input. |
| **Sound API** | No matter what safe code does — any input, any sequence of calls — the function never causes UB. |

A safe API can be **unsound**. That's the actual bug.

```rust
// SAFE signature, UNSOUND implementation:
pub fn read_first(slice: &[u32]) -> u32 {
    unsafe { *slice.as_ptr() } // UB if slice is empty!
}
```

The compiler accepts this. Calling it with `&[]` triggers UB. The function lied: it advertised a contract it doesn't fulfill.

**Your job when writing unsafe**: ensure that for every possible safe-code call, the unsafe block's preconditions hold.

## Validity vs safety invariants

A subtle but critical distinction. (Source: [Ralf Jung's blog](https://www.ralfj.de/blog/2018/08/22/two-kinds-of-invariants.html).)

### Validity invariant — *always* holds, even inside unsafe

If you violate one of these, **the value at that location is UB to even let exist** — never mind read or write. Examples:

- A `bool` must be `0` or `1`. (`mem::transmute::<u8, bool>(2)` is UB the moment it exists.)
- A `&T` must be non-null and aligned and point to dereferenceable memory of valid `T`.
- A `char` must be in `0..=0x10FFFF` excluding the surrogate range.
- A `NonZeroU32` must not be zero.
- An enum discriminant must be a valid variant.

> Treat validity invariants as *radioactive*. Even temporarily holding an invalid value is UB.

### Safety invariant — holds at the safe-code boundary, may be violated *inside* unsafe

These are higher-level promises a type makes. You can break them inside an `unsafe` block as long as you restore them before safe code can observe.

- `Vec<T>`: `len <= capacity`, the first `len` slots are initialized.
- `String`: bytes are valid UTF-8.
- `BTreeMap`: keys are sorted.

Inside `Vec::push`, there's a moment when `len` has been bumped but the new element hasn't been written. That's fine — only the unsafe code can see it. By the time the function returns, the invariant holds again.

## The "I'm talking to the optimizer" framing

Rust's optimizer is aggressive. It assumes:
- `&mut T` is unique (no aliases) for its lifetime → it can keep the value in a register.
- `&T` doesn't change → it can hoist the read out of a loop.
- Bounds checks fail rarely → it can unroll.
- `unreachable_unchecked()` really is unreachable → it can elide whole branches.

When you write `unsafe`, you're handing the optimizer a signed promise. Lie, and you don't get a runtime crash — you get a program that "works" until the optimizer notices it can do something faster based on your false promise.

## Common myths to unlearn

> "If it compiles and tests pass, it's fine."

False. UB is undetectable by tests in general. Use Miri.

> "I'll just put `unsafe` around the line that won't compile."

Wrong instinct. The borrow checker rejected it for a reason. Either the code is wrong, or you need `UnsafeCell` / `MaybeUninit` / `Cell` to model what you actually mean.

> "More unsafe = more performance."

Almost never. Modern Rust + LLVM optimizes safe code aggressively. Profile first; almost all the time, the bottleneck is allocation or algorithm, not bounds checks.

> "`*mut T` is just like `&mut T` without the rules."

False. They have different aliasing semantics, but pointers derived from references *inherit* those references' aliasing rules (Stacked Borrows). See [[aliasing|14.3]].

## How to introduce unsafe responsibly

1. **Need it?** No, really — is the problem actually solvable in safe Rust with `Cell`/`RefCell`/`Mutex`/`indices-instead-of-references`?
2. **Find the smallest unsafe block.** Push everything that can be safe outside the block.
3. **Write the `// SAFETY:` comment first.** If you can't articulate why it's sound, it's not sound.
4. **Wrap it in a safe API.** No `unsafe` should leak to users unless they truly need a raw primitive.
5. **Run Miri.** Then run it again with `-Zmiri-tree-borrows`.
6. **Document the invariants on the type.** A future reader reaches for the type's docs, not the unsafe block.

## Exit criteria
- [ ] You can list the 5 things `unsafe` allows without looking it up.
- [ ] You can give an example of a *safe* API that is *unsound*.
- [ ] You can give a fresh example of a validity invariant and a safety invariant.
- [ ] You can articulate why "compiles + tests pass" is not evidence of soundness.
