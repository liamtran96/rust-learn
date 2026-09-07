# Borrowed Scanner — Brief

> You write the implementation yourself. This brief only explains the task and unfamiliar syntax.
> When it compiles, runs, and Liam says "done," use `$journal` to log it.

## What you are building

Design a `Scanner` that borrows some text and remembers a cursor position. Its `peek` method observes the next character without moving the cursor, while `advance` returns the next character and moves forward. Write three tests.

## Expected input and output

The input is a borrowed string passed into a `Scanner`; the observable outputs are the `Option<char>` values returned by its methods. There is no required stdin, file input, or exact stdout format.

For a scanner over `"rust"`, an initial `peek` should return `Some('r')`; an `advance` should then return `Some('r')` and move the cursor. At the end of the source, both methods should return `None`.

## Required Rust syntax

```rust
struct Scanner<'a> {
    source: &'a str,
    cursor: usize,
}

impl<'a> Scanner<'a> {
    fn peek(&self) -> Option<char>;
    fn advance(&mut self) -> Option<char>;
}
```

- `struct Scanner<'a>` declares a type with a lifetime parameter named `'a`.
- `source: &'a str` is a borrowed string slice that must stay valid for at least `'a`.
- `cursor: usize` stores a non-negative position suitable for indexing or counting.
- `impl<'a> Scanner<'a>` says these methods belong to `Scanner` for any valid `'a`.
- `&self` temporarily borrows the scanner without allowing mutation.
- `&mut self` temporarily borrows it exclusively and allows the cursor to change.
- `-> Option<char>` means a method returns either `Some(character)` or `None` when no character is available.
- The semicolons above show signatures only. Your method definitions will need bodies in braces.

## Your coding steps

1. Declare the struct, create one in `main`, and print a simple stub message so `cargo run` compiles.
2. Add `peek` with a temporary compiling body, call it, then replace the stub with the required behavior.
3. Add `advance`, then write three tests covering peeking, advancing, and reaching the end.

## Concepts in play

- A struct borrowing data with a named lifetime
- Shared `&self` versus exclusive `&mut self` method receivers
- Unicode characters, cursor positions, and `Option`

## Watch out for

Decide what your cursor measures and keep it consistent: Rust string slices use UTF-8 byte boundaries, while a `char` can occupy more than one byte.

## References (read only if stuck)

- Chapter: `topics/rust/02-ownership/index.md`
- Specific notes: `topics/rust/02-ownership/lifetimes.md`
- Cheatsheet: `topics/rust/cheatsheets/lifetimes-cheatsheet.md`

## Checklist

- [ ] I can explain each part of the required syntax
- [x] `cargo run` compiles and prints the scanner demonstration
- [x] Implement the spec
- [x] `cargo clippy -- -D warnings` is clean
- [x] `cargo fmt` applied
- [x] Tests pass when the exercise requires them
- [x] Tell Codex "done" so `$journal` logs the session

Completed 2026-09-07 with guided syntax support. Independent explanation of all syntax remains a retrieval target; the owned-String variant is a separate exercise.

## Run

```text
cd code/02-ownership/scanner
cargo run
```
