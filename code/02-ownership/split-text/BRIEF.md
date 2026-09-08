# Hand-written string splitting - Brief

> You write the implementation yourself. This brief only explains the task and unfamiliar syntax.
> When it compiles, runs, and Liam says "done," use `$journal` to log it.

## What you are building

The Week 3 **Ship** bullet in `topics/rust/study-plan.md` asks you to re-implement `str::split` and `Vec::dedup` by hand. This crate covers the splitting half, useful for separating fields in a simple text record.

The plan leaves the interface open; this brief scopes a first version to one `char` separator and a vector of borrowed pieces. A custom iterator and other pattern types are later extensions. Implement the splitting yourself without calling the standard splitting methods.

## Expected input and output

Input means function arguments; output means the returned vector. No stdin, files, or exact stdout format is required.

- Input: text `"red,blue"`, separator `','`. Output pieces: `["red", "blue"]`.
- Preserve empty pieces: `",red,,"` produces `["", "red", "", ""]`.
- With no separator present, return the whole input as one piece; empty input produces `[""]`.
- Support Unicode text and separators. The returned pieces borrow the original text.

These behaviors follow the single-character case of [standard `str::split`](https://doc.rust-lang.org/std/primitive.str.html#method.split); returning a vector is this brief's learning interface.

## Required Rust syntax

Suggested signature for this scoped version (write the body yourself):

```rust
fn split_text(text: &str, separator: char) -> Vec<&str>
```

- `fn` declares a function; `split_text` is its name.
- Parentheses contain parameters, separated by a comma; `:` separates each name from its type.
- `text: &str` borrows text through a shared string slice; it does not take ownership of a caller's `String`.
- `separator: char` accepts one Unicode scalar value, written with single quotes, such as `','`.
- `->` introduces the return type.
- `Vec<&str>` is a growable vector whose elements are borrowed string slices; angle brackets specify the element type. The vector owns its list of references, not the source text.
- Because `text` is the only borrowed input, the returned slices' lifetime is tied to it automatically. No written lifetime parameter is needed here.
- Add a body in braces after the signature. Its final expression supplies the return value.

For displaying a vector, `println!("{pieces:?}");` uses `:?` for debug formatting of a variable named `pieces`.

## Your coding steps

1. Warm up: explain how borrowing a `String` differs from moving it. Run the generated greeting, then write a function stub and a call in `main` that compile.
2. Implement a simple two-piece example and display the result. Choose your own algorithm.
3. Add checks for no separator, repeated/leading/trailing separators, empty input, and Unicode text and separators. Explain where the returned text lives.

## Concepts in play

- Borrowed string slices and returned lifetimes
- A vector holding references
- UTF-8 positions and character boundaries

## Watch out for

String slice positions are byte offsets, and both ends must be UTF-8 character boundaries.

## References (read only if stuck)

- Source task: `topics/rust/study-plan.md` - Week 3, **Ship**
- Chapter exercises: `topics/rust/exercises/ch02-ownership.md` - ownership background; no separate splitting prompt
- Chapter: `topics/rust/02-ownership/index.md`
- Specific notes: `topics/rust/02-ownership/slices.md`, `topics/rust/02-ownership/lifetimes.md`
- Pitfalls: `topics/rust/pitfalls.md`
- Official explanation: [The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html)

## Checklist

- [ ] I can explain each part of the required syntax
- [ ] `cargo run` compiles and prints a stub
- [ ] Implement the scoped behavior above
- [ ] `cargo clippy -- -D warnings` is clean
- [ ] `cargo fmt` applied
- [ ] My behavior checks pass
- [ ] Tell Codex "done" so `$journal` logs the session

## Run

```text
cd code/02-ownership/split-text
cargo run
```
