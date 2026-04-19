---
title: 12.4 Advanced Traits
tags: [rust, traits, advanced]
---

# 12.4 Advanced Traits

A quick map of features you'll encounter as you read real-world crates.

## Associated types vs generic parameters

```rust
trait Iterator { type Item; /* ... */ }   // associated
trait From<T>  { fn from(t: T) -> Self; } // generic
```

Pick associated when there's one natural answer per impl; generic when multiple make sense.

## Supertraits

```rust
trait Printable: std::fmt::Debug { fn print(&self); }
```

Any implementer must also implement `Debug`.

## Higher-ranked trait bounds (HRTBs)

```rust
fn call<F>(f: F) where F: for<'a> Fn(&'a str) -> &'a str { /* ... */ }
```

"for any lifetime `'a`". You'll see this in closure APIs dealing with short-lived borrows.

## Marker traits

Traits with no methods, used to label capabilities: `Send`, `Sync`, `Copy`, `Sized`, `Unpin`, `FusedIterator`.

## Operator overloading

Implement `std::ops::Add`, `Sub`, etc., to give your types arithmetic operators. Same for `Index`, `Deref`, `Display`, etc.

```rust
use std::ops::Add;
struct V2 { x: f64, y: f64 }
impl Add for V2 {
    type Output = V2;
    fn add(self, other: V2) -> V2 { V2 { x: self.x + other.x, y: self.y + other.y } }
}
```

## `PhantomData`

A zero-sized marker that tells the compiler "this struct *semantically* owns/borrows a T" even if it doesn't literally hold one. Used in unsafe code and typestate patterns:

```rust
struct MyVec<T> {
    ptr: *mut T,
    len: usize,
    _marker: std::marker::PhantomData<T>,
}
```

## Specialization (unstable)

The ability to have a more specific impl override a blanket impl. Currently nightly-only (`min_specialization`). When it stabilizes, watch the std library get faster.

## Dyn compatibility

Object-safe traits can be `dyn Trait`. Non-object-safe ones can only be used as bounds. See [[../03-types-and-traits/trait-objects|trait objects]].

## Related
- [[../03-types-and-traits/traits|Traits]]
- [[../03-types-and-traits/trait-objects|Trait objects]]
