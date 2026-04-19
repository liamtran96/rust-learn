---
title: 2.4 Lifetimes
tags: [rust, lifetimes]
---

# 2.4 Lifetimes

> Lifetimes are **names for scopes** — they don't control anything, they just let the compiler express a relationship between references.

## The problem they solve

```rust
fn longest(x: &str, y: &str) -> &str {  // ❌ which input does the return borrow from?
    if x.len() > y.len() { x } else { y }
}
```

The compiler can't guess. You must name the input lifetimes and say the return borrows from both:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

Read as: "for any lifetime `'a`, given two refs that both live at least `'a`, return a ref that lives at least `'a`." The caller picks `'a` to be the *shorter* of the two inputs' actual lifetimes.

## What `'a` does **not** mean
- It does not *extend* anyone's life.
- It does not allocate or copy.
- It's purely a **constraint** the compiler checks.

## Elision rules

Most function signatures don't need annotations. Rust fills them in:

1. Each input reference gets its own lifetime: `fn f(x: &i32, y: &i32)` → `fn f<'a, 'b>(x: &'a i32, y: &'b i32)`.
2. If there is **exactly one** input lifetime, it is assigned to all output lifetimes.
3. If there is a `&self` or `&mut self`, `self`'s lifetime is assigned to all outputs.

This is why idiomatic Rust looks lifetime-free most of the time.

## Structs holding references

```rust
struct Parser<'a> {
    source: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str) -> Self { Self { source, pos: 0 } }
    fn peek(&self) -> Option<char> { self.source[self.pos..].chars().next() }
}
```

A struct with borrowed fields **cannot outlive** the data it references. That's often not what you want — in which case own the data (`String`) instead of borrowing (`&str`).

## `'static`

`'static` is the lifetime of **the whole program**. String literals are `&'static str`. A function bound `T: 'static` means "contains no references, or only `'static` ones."

**Common confusion:** `T: 'static` does **not** mean the value *lives* the whole program — just that it *could*, because it owns its data.

## When lifetimes bite

- Returning a reference to a local variable → compile error (good — it would dangle).
- Storing references in long-lived structures → often cleaner to own instead.
- Callbacks / closures that outlive their captures → usually want `move` or `Arc`.

## Advanced previews
- **Higher-ranked trait bounds**: `for<'a> Fn(&'a str) -> &'a str` — we'll meet these when reading iterator adapters.
- **`PhantomData<&'a T>`**: tells the compiler a struct semantically borrows something it doesn't literally hold — advanced unsafe/FFI.

## Exit criteria
- [ ] You can read `fn split<'a>(s: &'a str, sep: char) -> Vec<&'a str>` and say what the `'a` means.
- [ ] You know when elision applies and when it doesn't.
- [ ] You prefer owned fields over borrowed fields unless you have a real reason to borrow.

## Related
- [[borrowing|Borrowing]]
- [[../03-types-and-traits/generics|Generics]]
