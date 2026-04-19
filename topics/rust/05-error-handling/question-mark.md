---
title: 5.2 The ? Operator
tags: [rust, errors, question-mark]
---

# 5.2 The `?` Operator

The single most useful piece of syntax in Rust.

## What it does

`expr?` means: "if `expr` is `Err(e)`, return `Err(e.into())` from the current function; otherwise, unwrap the `Ok`."

```rust
fn read_number(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let s = std::fs::read_to_string(path)?;    // io::Error → Box<dyn Error>
    let n: i32 = s.trim().parse()?;             // ParseIntError → Box<dyn Error>
    Ok(n)
}
```

Without `?` the function would be a staircase of nested `match`es.

## Works on `Option` too

```rust
fn first_word_first_char(s: &str) -> Option<char> {
    s.split_whitespace().next()?.chars().next()
}
```

## Error conversion via `From`

`?` calls `From::from` on the error. If your function's `Err` type is `MyError` and the inner error is `io::Error`, you need:

```rust
impl From<std::io::Error> for MyError { /* ... */ }
```

The `thiserror` crate generates these for you — see [[custom-errors|Custom errors]].

## Where it can be used

- Inside a function whose return type is `Result<_, _>` (propagates as `Result`).
- Inside a function whose return type is `Option<_>` (propagates as `Option`).
- Inside async functions with the appropriate return.
- **Not** inside `main` by default — but `main` can return `Result<(), E>`:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = read_number("input.txt")?;
    println!("{n}");
    Ok(())
}
```

## Mixed `Option` and `Result`?

Convert:
```rust
let s: &str = data.first().ok_or("empty")?;    // Option → Result via ok_or
let n: i32 = s.parse().map_err(|_| "bad")?;    // different Err — remap
```

## Related
- [[result-option|Result and Option]]
- [[custom-errors|Custom error types]]
