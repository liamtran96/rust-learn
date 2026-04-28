---
title: 14.2 Raw Pointers, NonNull, Provenance
tags: [rust, unsafe, pointers, provenance]
---

# 14.2 Raw Pointers, `NonNull`, Provenance

> Authoritative: [Rustonomicon — Working with Unsafe](https://doc.rust-lang.org/nomicon/working-with-unsafe.html), [`std::ptr` docs](https://doc.rust-lang.org/std/ptr/).

References (`&T`, `&mut T`) come with the borrow checker's full enforcement. Raw pointers (`*const T`, `*mut T`) come with none. They're the escape hatch — and the rope to hang yourself.

## What raw pointers do (and don't) guarantee

| Property | Reference (`&T`/`&mut T`) | Raw pointer (`*const T`/`*mut T`) |
|---|---|---|
| Non-null | yes | no |
| Aligned | yes | no |
| Points to valid `T` | yes | no |
| Has a lifetime | yes | no |
| Aliasing tracked | yes (`&mut` is unique) | not by the type system |
| Safe to deref | yes | only inside `unsafe` |

**Raw pointers are addresses with a type label.** Nothing more.

## Creating raw pointers

```rust
let mut n = 5;

let r1: *const i32 = &n;          // coerce from &i32
let r2: *mut i32   = &mut n;      // coerce from &mut i32
let r3: *const i32 = &raw const n; // newer syntax (Rust 1.82+)
let r4: *mut i32   = &raw mut n;
```

> **Use `&raw const` / `&raw mut`** when you need a pointer *without* going through a reference. Going `&mut x as *mut _` first creates a `&mut x`, which already imposes uniqueness — bad if `x` aliases something else. `&raw mut x` skips the reference.

You can also create pointers from integers (`0xDEAD_BEEF as *mut u8`). Rarely correct outside MMIO / FFI / kernel work.

## Dereferencing rules

Every `*ptr` (read), `*ptr = ...` (write), `ptr.read()`, `ptr.write()` has these preconditions. **All must hold** or it's UB:

1. **Non-null.**
2. **Aligned** for `T` (use `.is_aligned()` or `align_of::<T>()` to check).
3. **Dereferenceable**: points to readable / writable allocated memory of size `size_of::<T>()`.
4. **Initialized** for reads (use [[uninit-and-cells|`MaybeUninit`]] for uninit memory).
5. **Aliasing rules** are upheld (see [[aliasing|14.3]]).
6. The pointer's **provenance** still permits the access.

```rust
unsafe {
    let v = *r1;       // read i32
    *r2 = 42;          // write i32
    let copy = r1.read();        // explicit read, supports unaligned variants
    r2.write(99);                // explicit write
}
```

## Pointer arithmetic

`offset`, `add`, `sub`, `wrapping_offset` — move pointers around. **Must stay within (or one-past-end of) the same allocation**.

```rust
let arr = [10u32, 20, 30, 40];
let p = arr.as_ptr();

unsafe {
    let third = *p.add(2);          // valid: index 2
    let _end  = p.add(arr.len());   // valid: one-past-end (no deref)
    // *p.add(arr.len())            // UB: out of bounds deref
    // p.add(arr.len() + 1)         // UB: too far past end (no deref needed for this to be UB)
}
```

`add(n)` advances by `n * size_of::<T>()` bytes. `wrapping_add` is the same but lets you cross allocation boundaries arithmetically (still UB to deref the result outside the original allocation).

## `NonNull<T>` — the type-system upgrade

`*mut T` admits null. `NonNull<T>` doesn't. Use it whenever you store a pointer that you maintain non-null:

```rust
use std::ptr::NonNull;

struct MyBox<T> {
    ptr: NonNull<T>,  // never null; layout-equivalent to *mut T
}
```

Benefits:
- `Option<NonNull<T>>` is the same size as `*mut T` (niche optimization).
- Communicates intent.
- Provides `as_ptr()`, `as_ref()`, `as_mut()` helpers.

`NonNull::dangling()` returns a "well-aligned but bogus" address — useful for ZSTs (zero-sized types) and as a placeholder before allocation.

## Provenance — the modern mental model

A pointer is not just an address. It carries **provenance**: information about which allocation it came from and what it's allowed to access. Two pointers with the same numeric address can have different provenance and therefore different validity.

```rust
let a = [0u8; 4];
let b = [0u8; 4];
let pa = a.as_ptr();
let pb = b.as_ptr();

// Even if you cast pa to an integer and back, you can only use it
// to access `a`'s allocation — not `b`'s, even at the same address.
```

Practical consequences:
- `int → ptr` casts have **wildcard provenance** (`std::ptr::with_exposed_provenance`). They work but inhibit optimizations and are hard to verify.
- Prefer to derive pointers from existing pointers (`add`, `cast`), not from integers.
- The future-stable API: [`ptr::with_exposed_provenance`](https://doc.rust-lang.org/std/ptr/fn.with_exposed_provenance.html) and `expose_provenance` make the model explicit.

You don't need to fully understand provenance to write correct code. You do need to know it exists — Miri will yell at you when you violate it.

## Useful `*mut T` methods worth memorizing

```rust
ptr.read()              // *ptr without running Drop
ptr.write(value)        // *ptr = value without dropping the old value
ptr.read_unaligned()    // for unaligned addresses
ptr.write_unaligned(v)
ptr.copy_to(dst, n)     // memmove, n elements
ptr.copy_to_nonoverlapping(dst, n)  // memcpy
ptr.swap(other)         // swap *self and *other
ptr.is_null()
ptr.is_aligned()
ptr.add(n) / ptr.sub(n) / ptr.offset(isize)
ptr.cast::<U>()         // change type, same address
NonNull::new(p)         // Some(NonNull) or None if null
```

`ptr::read` vs `*ptr` matter: dereferencing reads the value *and* asserts the location is valid for reads as `T`. `read` is a bitwise copy that doesn't run `Drop` — important when ownership is being transferred.

## ZSTs — the silent gotcha

For `T: Sized` with `size_of::<T>() == 0`:
- The pointer has no meaningful address; only alignment matters.
- `NonNull::dangling()` is the canonical pointer for "we have N elements of a ZST."
- `ptr.add(n)` for ZSTs is a no-op address-wise but still must satisfy provenance.

Allocating zero bytes from the global allocator is **UB**. Always check ZST and skip allocation:

```rust
let cap = 4;
let layout = std::alloc::Layout::array::<T>(cap).unwrap();
if layout.size() == 0 {
    NonNull::dangling()
} else {
    NonNull::new(unsafe { std::alloc::alloc(layout) as *mut T }).expect("OOM")
}
```

## Exit criteria
- [ ] You can list the 6 preconditions for dereferencing a pointer.
- [ ] You can explain when to use `&raw mut x` instead of `&mut x as *mut _`.
- [ ] You know why `NonNull<T>` is preferred over `*mut T` in struct fields.
- [ ] You can articulate "provenance" in one sentence.
- [ ] You handle ZST in your allocation code.
