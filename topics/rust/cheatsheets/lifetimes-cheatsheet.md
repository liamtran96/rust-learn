---
title: Lifetimes Cheatsheet
tags: [rust, cheatsheet, lifetimes]
---

# Lifetimes Cheatsheet

## Syntax

```rust
fn f<'a>(x: &'a str) -> &'a str                   // input & output share lifetime
fn f<'a, 'b>(x: &'a str, y: &'b str) -> &'a str   // output borrows from x
struct Ref<'a> { r: &'a i32 }
impl<'a> Ref<'a> { fn get(&self) -> &'a i32 { self.r } }
```

## Elision rules — when you can omit
1. Each elided input ref gets its own lifetime.
2. One input lifetime → applies to all outputs.
3. A `&self` or `&mut self` → its lifetime applies to all outputs.

Otherwise, annotate.

## `'static`

Means "could live for the whole program." String literals are `&'static str`. `T: 'static` means T contains no non-`'static` borrowed references — **not** that the value *does* live forever.

## Struct with borrowed data
```rust
struct Parser<'a> { src: &'a str, pos: usize }
impl<'a> Parser<'a> { /* methods auto-elide */ }
```

A `Parser<'a>` cannot outlive its source. Often it's cleaner to own — `src: String`.

## Lifetime subtyping
`'a: 'b` means "`'a` outlives `'b`". You'll see this where one reference must outlive another.

## Common errors
- "missing lifetime specifier" → compiler can't elide; annotate.
- "X does not live long enough" → shorten borrow scope or extend owner's scope.
- "borrowed value does not live long enough" → storing a reference to something about to drop.
