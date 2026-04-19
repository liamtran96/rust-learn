---
title: 5. Error Handling
tags: [rust, errors]
---

# 5. Error Handling

Rust draws a sharp line: **recoverable** errors go through `Result<T, E>`; **unrecoverable** ones go through `panic!`. No exceptions.

## Contents
- [[result-option|5.1 Result<T, E> & Option<T>]]
- [[question-mark|5.2 The `?` operator]]
- [[custom-errors|5.3 Custom error types — thiserror, anyhow]]

## The principles

1. **Nothing is null.** Absence is `Option::None`.
2. **Nothing throws.** Failure is `Result::Err(E)`.
3. **The `?` operator** propagates errors upward with almost zero syntax.
4. **`panic!` is for bugs**, not for control flow — use when a precondition is violated.

## When to pick `panic!` vs `Result`

| Situation | Prefer |
|---|---|
| Precondition violated (caller's bug) | `panic!` via `assert!`, `unreachable!`, `debug_assert!` |
| Resource missing, network failed, bad input | `Result<_, E>` |
| Prototype / example / tests | `.unwrap()` or `.expect("…")` is fine |
| Library code | `Result<_, E>` with a meaningful `E` |
| Binary / main | `?` bubbling to a `Box<dyn Error>` or `anyhow::Error` |

## Exit criteria
- [ ] You can write a function returning `Result<T, E>` and use `?` to compose it with others.
- [ ] You can pick the right combinator (`map`, `and_then`, `or_else`, `ok_or`, `unwrap_or`) instead of matching.
- [ ] You can define an error enum with `thiserror`.
- [ ] You know when `anyhow::Error` is appropriate (binaries) and when it's not (libraries).
