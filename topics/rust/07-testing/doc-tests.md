---
title: 7.3 Doc Tests
tags: [rust, testing, docs]
---

# 7.3 Doc Tests

Code examples in doc comments are **compiled and executed** by `cargo test`. This is the single best feature for keeping docs honest.

## Writing one

````rust
/// Adds two numbers.
///
/// # Examples
///
/// ```
/// use my_crate::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
````

The block runs under `cargo test`. If the assertion fails, your doc test fails.

## Useful annotations

````markdown
```ignore            → not compiled, not run
```no_run            → compiled but not executed
```should_panic      → expected to panic
```compile_fail      → must fail to compile (useful to document invariants)
```text              → not Rust at all
````

## Hiding setup

Prefix lines with `#` to hide them from the rendered docs but include them in the test:

````rust
/// ```
/// # use my_crate::Config;
/// # let cfg = Config::default();
/// assert!(cfg.is_valid());
/// ```
````

## `rustdoc` basics

```bash
cargo doc --open               # build + browse your crate's docs
cargo doc --no-deps             # skip dep docs
```

Triple-slash comments (`///`) document the **item** below; `//!` documents the **enclosing** module/crate. Markdown works. Intra-doc links: `` [`MyType`] `` resolves automatically.

## Related
- [[unit-tests|Unit tests]]
- [[../06-modules/crates-and-workspaces|Crates]]
