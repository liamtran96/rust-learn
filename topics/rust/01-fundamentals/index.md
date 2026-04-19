---
title: 1. Fundamentals
tags: [rust, fundamentals]
---

# 1. Fundamentals

Everything in this chapter is *non-negotiable* before moving on. If any concept feels fuzzy, loop back.

## Contents
- [[toolchain|1.1 Toolchain — rustup, cargo, rustc, clippy, fmt]]
- [[variables|1.2 Variables, mutability, shadowing, constants]]
- [[data-types|1.3 Data types — scalars, compounds, inference]]
- [[functions|1.4 Functions, expressions, statements]]
- [[control-flow|1.5 Control flow — if, loop, while, for]]

## Key ideas
- Rust is **expression-oriented**: almost everything is an expression that yields a value.
- Variables are **immutable by default** — `mut` is an opt-in signal to readers.
- The type system is **strict but helpful** — let inference carry you, annotate on boundaries.
- There are **no implicit conversions** — even between integer types.

## Exit criteria
You can stop working on this chapter when you can:
- [ ] Create a new cargo project, add a dependency, run tests, and build a release binary.
- [ ] Explain the difference between `let x = 5;` and `let mut x = 5;` and why shadowing (`let x = x + 1;`) is not mutation.
- [ ] Write a function that returns a value without using `return`.
- [ ] Use a `loop` with `break value` to compute a result.
- [ ] Read a compiler error about type mismatch and fix it by converting with `as`, `.into()`, or `TryFrom`.
