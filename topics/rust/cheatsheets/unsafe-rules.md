---
title: Unsafe Rust Cheatsheet
tags: [rust, unsafe, cheatsheet, miri]
---

# Unsafe Rust Cheatsheet

Quick reference. Authority: [Rustonomicon](https://doc.rust-lang.org/nomicon/) · [Reference: behavior considered undefined](https://doc.rust-lang.org/reference/behavior-considered-undefined.html).

## What `unsafe` lets you do

1. Dereference a raw pointer.
2. Call an `unsafe fn` (or unsafe method).
3. Implement an `unsafe` trait (`Send`, `Sync`, etc.).
4. Read/write a `static mut`.
5. Access fields of a `union`.

Everything else stays checked.

## Validity vs safety

| | Holds when | Violation = |
|---|---|---|
| **Validity invariant** | Always, including inside unsafe | Immediate UB |
| **Safety invariant** | At the safe-code boundary | Logical bug; UB only if observed by safe code |

`bool` must be 0/1. `&T` must be non-null + aligned + dereferenceable. `char` must be valid Unicode. `Vec.len <= cap` is a safety invariant; the others are validity.

## Reference / pointer rules

```text
While a reference is "live":
  - Through &mut T: only this path may read or write.
  - Through &T:     this path and other &T may read; nobody writes.
A pointer derived from a reference INHERITS the reference's borrow rules.
A pointer from `&raw const x` / `&raw mut x` does NOT.
```

## Pointer dereference preconditions (all must hold)

1. Non-null
2. Aligned for `T`
3. Points to allocated, readable/writable memory of size ≥ `size_of::<T>()`
4. Initialized as a valid `T` (for reads)
5. Aliasing rules upheld (Stacked Borrows / Tree Borrows)
6. Provenance permits the access

## Common UB shapes

- Dangling pointer access
- Misaligned access
- Reading uninitialized memory
- Mutable aliasing (`&mut` not unique)
- Type confusion via `transmute`
- Data race (≥1 write, no sync, not atomic)
- Invalid value at validity-typed location
- Out-of-bounds offset (more than 1-past-end)
- Panic across `extern "C"` (use `catch_unwind` or `panic = "abort"`)

## Pointer creation

```rust
let p: *const T = &x;            // immutable ref → ptr (inherits borrow)
let p: *mut T   = &mut x;        // mutable ref → ptr (inherits borrow)
let p: *const T = &raw const x;  // ptr without going through a reference
let p: *mut T   = &raw mut x;    // ptr without going through a reference
let nn: NonNull<T> = NonNull::from(&x);
let nn: NonNull<T> = NonNull::dangling();   // for ZST / placeholder
let nn = NonNull::new(p)?;       // null check
```

## Useful pointer methods

```rust
ptr.read()                 // bitwise read, no Drop
ptr.write(v)               // bitwise write, no Drop of old
ptr.read_unaligned()
ptr.write_unaligned(v)
ptr.copy_to(dst, n)        // memmove
ptr.copy_to_nonoverlapping(dst, n)  // memcpy
ptr.add(n) / ptr.sub(n) / ptr.offset(isize)
ptr.is_null() / ptr.is_aligned()
ptr.cast::<U>()
```

## `MaybeUninit<T>`

```rust
let mut slot = MaybeUninit::<T>::uninit();
slot.write(t);
let v = unsafe { slot.assume_init() };

// arrays
let mut arr: [MaybeUninit<T>; N] = [const { MaybeUninit::uninit() }; N];
for s in &mut arr { s.write(...); }
let init: [T; N] = unsafe { MaybeUninit::array_assume_init(arr) };
```

## `UnsafeCell<T>`

The only legal way to mutate behind `&T`. Building blocks for `Cell`, `RefCell`, `Mutex`, atomics.

```rust
struct Cell<T> { value: UnsafeCell<T> }
fn set(&self, v: T) { unsafe { *self.value.get() = v; } } // OK only because we never hand out &T
```

## Send / Sync (the unsafe traits)

```rust
// SAFETY: justify in detail.
unsafe impl<T: Send> Send for MyType<T> {}
unsafe impl<T: Sync> Sync for MyType<T> {}
```

`Send` = ok to move to another thread. `Sync` = `&T` is safe to share between threads.

## Run Miri

```bash
rustup +nightly component add miri
cargo +nightly miri test
MIRIFLAGS="-Zmiri-tree-borrows" cargo +nightly miri test
MIRIFLAGS="-Zmiri-strict-provenance" cargo +nightly miri test
```

## SAFETY comment template

```rust
// SAFETY:
// 1. Precondition: <X must hold>. Established by <Y>.
// 2. Aliasing: <why no overlapping live borrows>.
// 3. Drop / panic: <how partial state is handled>.
unsafe { ... }
```

For an `unsafe fn`:
```rust
/// # Safety
///
/// Caller must ensure <conditions>.
pub unsafe fn foo() { ... }
```

## Order of preference (before reaching for unsafe)

1. Indices instead of references (`Vec<usize>` of node ids).
2. `Cell` / `RefCell` / `OnceCell` / `Mutex` / atomics.
3. `Rc<RefCell<T>>` / `Arc<Mutex<T>>`.
4. A vetted unsafe-using crate (`bumpalo`, `slotmap`, `parking_lot`, `crossbeam`, `petgraph`).
5. std escape hatches (`split_at_mut`, `chunks_mut`, `swap`, `iter::zip`).
6. *Then* hand-rolled unsafe.

## Red flags

- `unsafe { transmute(...) }` — almost always wrong.
- `static mut` — use `Mutex` / `OnceLock`.
- `unsafe impl Sync` with no comment — block.
- `unsafe fn` without `# Safety` doc — block.
- `#[cfg(not(miri))]` over the unsafe path — lying.
- `unwrap()` inside an `unsafe` block — drop-guard hazard.
