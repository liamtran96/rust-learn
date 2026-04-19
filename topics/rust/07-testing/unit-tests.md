---
title: 7.1 Unit Tests
tags: [rust, testing]
---

# 7.1 Unit Tests

## The basic shape

```rust
// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]                       // only compiled for `cargo test`
mod tests {
    use super::*;                  // pull in parent module's items

    #[test]
    fn adds_two_positives() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn panics_on_overflow() {
        add(i32::MAX, 1);
    }

    #[test]
    #[ignore]                       // skipped by default; `cargo test -- --ignored`
    fn slow_test() { /* ... */ }
}
```

## Assertion macros

| Macro | Use |
|---|---|
| `assert!(cond)` | Boolean check |
| `assert_eq!(a, b)` | Equality with nice diff on failure |
| `assert_ne!(a, b)` | Inequality |
| `debug_assert!`/`_eq!`/`_ne!` | Only checked in debug builds |

Prefer `assert_eq!` over `assert!(a == b)` — the failure message is much better.

## Returning `Result` from tests

```rust
#[test]
fn parses_ok() -> Result<(), Box<dyn std::error::Error>> {
    let n: i32 = "42".parse()?;
    assert_eq!(n, 42);
    Ok(())
}
```

Cleaner than unwrapping all over the place.

## Helpful patterns

- **Table-driven tests**: use a slice of tuples `&[(&str, i32)]` and loop with `assert_eq!`.
- **Fixtures**: factor setup into functions called from each test.
- **Temp directories**: use the [`tempfile`](https://crates.io/crates/tempfile) crate — `tempdir()` auto-cleans.
- **Snapshots**: [`insta`](https://crates.io/crates/insta) for large-output assertions.
- **Property testing**: [`proptest`](https://crates.io/crates/proptest) or [`quickcheck`](https://crates.io/crates/quickcheck).

## Running tests efficiently

```bash
cargo test                      # all
cargo test parsing::            # only tests under that module path
cargo test -- --list            # just list names
cargo test -- --nocapture       # show println! output
cargo test --release            # run with optimizations
cargo nextest run               # much faster runner (install: `cargo install cargo-nextest`)
```

## Related
- [[integration-tests|Integration tests]]
- [[doc-tests|Doc tests]]
