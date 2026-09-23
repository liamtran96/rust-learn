# Point derives — Brief

> You write the implementation yourself. This brief only explains the task and unfamiliar syntax.
> When it compiles, runs, and Liam says "done," use `$journal` to log it.

## What you are building

Define a two-dimensional `Point` with integer `x` and `y` fields. Ask Rust to generate implementations of `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, and `Hash`, then write tests that exercise the behavior supplied by every derived trait.

In real applications, these traits let value types work naturally with logging, duplication, equality checks, and hash-based collections such as `HashSet` and `HashMap`.

## Expected input and output

This exercise has no stdin or required command-line output. Its observable behavior is in the tests: construct `Point` values and verify that formatting, copying/cloning, equality, and hashing behave as expected.

Example behavior: two points created with `x = 2` and `y = 3` compare as equal, while a point with a different coordinate does not.

## Required Rust syntax

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
```

- `#[...]` is an attribute: extra instructions attached to the item below it.
- `derive(...)` asks the compiler to generate standard trait implementations.
- `Debug` permits developer-facing formatting with `{:?}`.
- `Clone` permits an explicit duplication operation.
- `Copy` makes assignment and argument passing duplicate this small value instead of moving it.
- `PartialEq` permits `==` and `!=`; `Eq` states that equality is fully reflexive for this type.
- `Hash` lets the value produce a hash for collections such as `HashSet`.
- `struct Point` declares a named-field type; `x: i32` means the `x` field stores a signed 32-bit integer.

Tests normally live in a `#[cfg(test)]` module. Each `#[test]` function should demonstrate one trait behavior with an assertion or a successful operation requiring that trait.

## Your coding steps

1. Add the derived `Point` declaration while leaving `main` as a compiling stub.
2. Add one small test at a time, beginning with debug formatting and equality.
3. Add focused checks for `Clone`, `Copy`, and use in a hash-based collection; run the tests after each addition.

## Concepts in play

- Named-field structs
- Compiler-generated trait implementations
- Value semantics with `Clone` and `Copy`
- Trait-enabled formatting, comparison, and hashing

## Watch out for

Because `Point` is also `Copy`, directly calling `.clone()` may trigger Clippy's `clone_on_copy` warning; if that happens, verify `Clone` through a small generic trait-bound check rather than suppressing the warning.

## References (read only if stuck)

- Chapter: `topics/rust/03-types-and-traits/index.md`
- Specific notes: `topics/rust/03-types-and-traits/structs.md`
- Trait details: <https://doc.rust-lang.org/std/marker/trait.Copy.html>

## Checklist

- [ ] I can explain each part of the required syntax
- [ ] `cargo run` compiles and prints a stub
- [ ] Implement the spec
- [ ] `cargo clippy -- -D warnings` is clean
- [ ] `cargo fmt` applied
- [ ] Tests pass when the exercise requires them
- [ ] Tell Codex "done" so `$journal` logs the session

## Run

```text
cd code/03-types-and-traits/point-derives
cargo run
```
