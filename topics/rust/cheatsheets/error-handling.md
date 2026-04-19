---
title: Error Handling Combinators Cheatsheet
tags: [rust, cheatsheet, errors]
---

# Error Handling Combinators Cheatsheet

## `Option<T>`

| From → To | Method |
|---|---|
| `Option<T>` → `Option<U>` | `map(\|t\| u)` |
| `Option<T>` → `Option<U>` (flatten) | `and_then(\|t\| Option<U>)` |
| `Option<T>` → `T` | `unwrap()` / `expect(msg)` — ⚠ panics |
| `Option<T>` → `T` | `unwrap_or(default)` |
| `Option<T>` → `T` | `unwrap_or_else(\|\| default)` |
| `Option<T>` → `T` | `unwrap_or_default()` |
| `Option<T>` → `Option<T>` | `or(other)` / `or_else(\|\| other)` |
| `Option<T>` → `Option<T>` | `filter(\|t\| pred)` |
| `Option<T>` → `Result<T, E>` | `ok_or(err)` / `ok_or_else(\|\| err)` |
| `Option<T>` → `bool` | `is_some()` / `is_none()` |
| `Option<T>` → `Option<U>` (chain 2) | `and(other)` |

## `Result<T, E>`

| From → To | Method |
|---|---|
| `Result<T, E>` → `Result<U, E>` | `map(\|t\| u)` |
| `Result<T, E>` → `Result<T, F>` | `map_err(\|e\| f)` |
| `Result<T, E>` → `Result<U, E>` (flatten) | `and_then(\|t\| Result<U, E>)` |
| `Result<T, E>` → `Result<T, F>` (recover) | `or_else(\|e\| Result<T, F>)` |
| `Result<T, E>` → `Option<T>` | `ok()` |
| `Result<T, E>` → `Option<E>` | `err()` |
| `Result<T, E>` → `T` | `unwrap()` / `expect(msg)` — ⚠ panics |
| `Result<T, E>` → `T` | `unwrap_or(default)` / `unwrap_or_else(\|e\| default)` |
| `Result<T, E>` → `T` (propagate Err up) | `?` |

## Useful macros & idioms

```rust
// Early return on Err
let value = some_op()?;

// Recover with default
let value = some_op().unwrap_or_default();

// Add context (anyhow)
let value = some_op().with_context(|| "while doing X")?;

// Map multiple errors into a shared enum (thiserror + From)
let value: MyError = io_op()?;      // via #[from] std::io::Error

// Consume many Results into one
let all: Result<Vec<_>, _> = items.iter().map(|i| i.parse::<i32>()).collect();

// Ignore Err, keep Ok
let ok_items: Vec<_> = items.iter().filter_map(|i| i.parse::<i32>().ok()).collect();
```

## `anyhow::Error`
- Accepts any `E: std::error::Error + Send + Sync + 'static`.
- Carries a **context chain** you build via `.context(...)` or `.with_context(||...)`.
- Perfect for binaries. In libraries, define your own enum with `thiserror`.

## Panics you *can* reach for
- `unwrap()` / `expect("msg")` — in tests, examples, and truly unreachable branches.
- `assert!` / `assert_eq!` — invariant checks.
- `unreachable!()` — for arms the compiler can't prove are dead.
- `todo!()` / `unimplemented!()` — placeholders.
