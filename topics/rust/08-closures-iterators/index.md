---
title: 8. Closures & Iterators
tags: [rust, closures, iterators]
---

# 8. Closures & Iterators

The point at which Rust feels *fun*. Iterator pipelines are concise, lazy, and compile to tight loops.

## Contents
- [[closures|8.1 Closures — Fn, FnMut, FnOnce]]
- [[iterators|8.2 Iterators — the lazy pipeline]]

## Key ideas
- Closures are anonymous functions that can **capture** variables.
- The `Fn`/`FnMut`/`FnOnce` trait family encodes *how* a closure uses its captures.
- `Iterator` is a trait with **one required method** (`next()`), but dozens of default combinators.
- Iterators are **lazy** — nothing runs until you call a *consumer* (`collect`, `sum`, `for_each`, …).
- Iterator adapter chains often compile to equivalent machine code as hand-written loops.

## Exit criteria
- [ ] You can write a closure with and without `move`, and explain what changed.
- [ ] You can chain `.iter().filter(...).map(...).collect::<Vec<_>>()`.
- [ ] You know `fold`, `reduce`, `any`, `all`, `find`, `position`, `count`, `sum`, `product`.
- [ ] You can implement `Iterator` for your own type.
