---
title: 3. Structs, Enums, Generics, Traits
tags: [rust, types, traits, generics]
---

# 3. Structs, Enums, Generics, Traits

Where Rust gets *expressive*. Sum types, ad-hoc polymorphism, zero-cost generics.

## Contents
- [[structs|3.1 Structs]]
- [[enums|3.2 Enums — algebraic data types]]
- [[pattern-matching|3.3 Pattern matching]]
- [[generics|3.4 Generics]]
- [[traits|3.5 Traits]]
- [[trait-objects|3.6 Trait objects (dyn Trait)]]

## Key ideas
- **Structs** are product types (records). **Enums** are sum types (tagged unions).
- **Pattern matching** is *exhaustive* — the compiler requires you to handle every variant.
- **Generics** are monomorphized — zero runtime cost, each concrete type gets its own compiled version.
- **Traits** are Rust's interface/typeclass system — they can be **static** (generics) or **dynamic** (`dyn Trait`).
- Rust has **no inheritance** — compose via traits and embedding.

## Exit criteria
- [ ] You can represent a typed payload like `Shape = Circle | Square | Triangle` as an enum and compute its area with `match`.
- [ ] You can write a generic function `fn largest<T: PartialOrd>(xs: &[T]) -> &T`.
- [ ] You can define your own trait with a default method and implement it for multiple types.
- [ ] You know when to pick `impl Trait` vs `Box<dyn Trait>` for a return type.
