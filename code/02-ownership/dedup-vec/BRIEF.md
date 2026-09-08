# Hand-written `Vec::dedup` — Brief

> You write the implementation yourself. This brief only explains the task and unfamiliar syntax.
> When it compiles, runs, and Liam says "done," use `$journal` to log it.

## What you are building

Implement the consecutive-duplicate removal half of `Vec::dedup` without calling
the standard `dedup` method. The function must modify the original vector in
place: neighboring equal values collapse to one value, but equal values separated
by another value remain separate.

## Expected input and output

Input and output are function arguments and the mutated vector; no stdin, files,
or exact stdout format are required. For example, `[1, 1, 2, 1, 2, 2]` becomes
`[1, 2, 1, 2]`. An empty vector and a vector with no adjacent duplicates should
remain unchanged.

## Required Rust syntax

```rust
fn dedup_in_place<T: PartialEq>(values: &mut Vec<T>)
```

- `fn` declares a function and `dedup_in_place` names it.
- `T` is a generic type placeholder, so the function can work with many element types.
- `: PartialEq` requires `T` to support equality comparisons such as `==`.
- `values: &mut Vec<T>` exclusively borrows the caller's growable vector, allowing the
  function to change it without taking ownership of the vector itself.
- There is no `->` return type: the observable result is the changed vector.

## Your coding steps

1. Add the signature and a small `main` example that compiles and prints a vector.
2. Handle a vector containing one adjacent duplicate while preserving the original allocation choice.
3. Test repeated runs of duplicates, edge positions, no duplicates, and an empty vector.
4. Add at least one non-integer element type if your comparison bound supports it naturally.

## Concepts in play

- Mutable borrowing with `&mut`
- Generic functions and the `PartialEq` trait bound
- In-place mutation and vector indexing/length
- Ownership while iterating over a collection

## Watch out for

Do not hold a reference into the vector while changing its length; also avoid
`for value in values` when you still need to use the vector, because that iteration
form consumes it.

## References (read only if stuck)

- Chapter: `topics/rust/02-ownership/index.md`
- Specific notes: `topics/rust/02-ownership/ownership.md`, `topics/rust/02-ownership/borrowing.md`
- Pitfalls: `topics/rust/pitfalls.md`

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
cd code/02-ownership/dedup-vec
cargo run
```
