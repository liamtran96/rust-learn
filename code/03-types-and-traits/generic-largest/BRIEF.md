# Generic `largest` Functions — Brief

> You write the implementation yourself. This brief only explains the task and unfamiliar syntax.
> When it compiles, runs, and Liam says "done," use `$journal` to log it.

## What you are building

Write three generic functions that find the largest item in a slice:

1. The required `Copy` version.
2. An empty-safe version that returns `Option<T>`.
3. A non-`Copy` version that borrows the winning item and returns `Option<&T>`.

## Expected input and output

The input and output are function arguments and return values—there is no required stdin or file data. Use `main` to print demonstrations, and add tests for non-empty and empty slices.

Concrete example: given `&[3, 8, 5]`, each version should identify `8`; an empty-safe version given `&[]` should return `None`. The specification does not prescribe names for the second and third functions, so choose clear names.

## Required Rust syntax

`fn largest<T: PartialOrd + Copy>(xs: &[T]) -> T`

- `fn largest` declares a function named `largest`.
- `<T: ...>` introduces a generic type named `T`.
- `PartialOrd` requires `T` values to support comparisons such as `>`.
- `+ Copy` also requires values to be cheaply copied instead of moved.
- `xs` is the parameter name.
- `&[T]` is a shared borrow of a slice containing `T` values. The function may inspect the items without owning the collection.
- `-> T` says this first version returns an owned `T` value.
- `Option<T>` represents either `Some(value)` or `None`, which lets a function handle an empty slice without panicking.
- `Option<&T>` returns either a shared reference to an item in the input slice or `None`; it does not copy or take ownership of that item.

## Your coding steps

1. Run the generated program unchanged so you know the crate starts clean, then declare the required `largest` signature with a temporary compiling body.
2. Implement the non-empty `Copy` behavior and demonstrate it with one numeric slice.
3. Add the empty-safe owned version, then test both its `Some` and `None` cases.
4. Add the borrowing version using a non-`Copy` element type, then test that the original collection remains usable.

## Concepts in play

- Generic type parameters
- Trait bounds with `PartialOrd + Copy`
- Borrowed slices and borrowed return values
- Modeling an optional result with `Option`

## Watch out for

Do not add `.clone()` merely to make the non-`Copy` version compile; that version should return a reference into the input slice.

## References (read only if stuck)

- Chapter: `topics/rust/03-types-and-traits/index.md`
- Specific notes: `topics/rust/03-types-and-traits/generics.md`

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
cd code/03-types-and-traits/generic-largest
cargo run
```
