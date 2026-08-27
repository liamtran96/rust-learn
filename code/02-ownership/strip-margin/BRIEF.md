# Strip Margin - Brief

> You write the implementation yourself. This brief only explains the task and unfamiliar syntax.
> When it compiles, runs, and you say "done," use `$journal` to log it.

## What you are building

Write a function named `strip_margin`. It receives some text and a prefix character. For every line, remove the leading whitespace through the first prefix character, and return the cleaned text.

Example behavior:

```text
input text:     "  |hello\n    |world"
prefix:         '|'
returned text:  "hello\nworld"
```

## The required function signature

```rust
fn strip_margin(s: &str, prefix: char) -> String
```

Read it from left to right:

- `fn` declares a function.
- `strip_margin` is the function's name.
- `s: &str` means the parameter `s` borrows some text; the function may read it without owning it.
- `prefix: char` means `prefix` is one Unicode character, such as `'|'`. A `char` uses single quotes.
- `-> String` means the function must return newly owned text. A `String` can be built and returned independently of `s`.

The signature ends with a function body in braces. You decide what goes inside those braces.

## Your coding steps

1. Replace the generated program with the required function and an empty starting body of your choice.
2. Make the function handle one line first.
3. Extend it to process every line in the input.
4. Call it from `main` with the example above and print the returned value.
5. Add tests for normal input, empty input, and more than one line.

Pause and ask about any syntax you cannot read. You do not need to design the whole function before compiling.

## Concepts in play

- Borrowing input as `&str`
- Working with lines and characters
- Building and returning an owned `String`

## Watch out for

Do not index a Rust string by a numeric position: strings are UTF-8. Use character-aware string methods when you need to inspect text.

## References (read only if stuck)

- Chapter overview: `topics/rust/02-ownership/index.md`
- String slices: `topics/rust/02-ownership/slices.md`
- Common string pitfalls: `topics/rust/pitfalls.md` sections 11 and 12

## Checklist

- [x] I can explain each part of the function signature
- [x] `cargo run` compiles and prints a stub
- [x] The example produces the expected text
- [ ] I added tests for the important cases
- [x] `cargo clippy -- -D warnings` is clean
- [x] `cargo fmt` applied
- [x] Tell Codex "done" so `$journal` logs the session

## Run

```text
cd code/02-ownership/strip-margin
cargo run
```
