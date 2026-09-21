# Generic Pair Swap — Brief

> You write the implementation yourself. This brief only explains the task and unfamiliar syntax.
> When it compiles, runs, and Liam says "done," use `$journal` to log it.

## What you are building

Define a `Pair<T, U>` struct that can hold two values of different types. Give it a consuming `swap` method that returns a new pair with both the values and their type positions reversed.

## Expected input and output

The input and output are Rust values passed through a method; this exercise does not require stdin or file data.

Example: a pair whose `first` value is `"north"` and whose `second` value is `7` becomes a pair whose `first` value is `7` and whose `second` value is `"north"`.

## Required Rust syntax

`struct Pair<T, U>`

- `struct` declares a type with named fields.
- `Pair` is the type's name.
- `<T, U>` declares two generic type parameters. `T` is the type of one field and `U` is the type of the other; they may be different.

`fn swap(self) -> Pair<U, T>`

- `fn` declares a function or method.
- `swap` is the method name.
- `self` means the method takes ownership of the pair it is called on. The old pair cannot be used afterward.
- `->` introduces the return type.
- `Pair<U, T>` says the returned pair has the generic type positions reversed.

The method belongs in an `impl<T, U> Pair<T, U>` block. The first `<T, U>` declares the generics for the implementation; `Pair<T, U>` identifies the type receiving the method.

## Your coding steps

1. Declare `Pair<T, U>` with two named fields, then run `cargo check`.
2. Add the generic `impl` block and a compiling `swap` method with the required signature.
3. Construct one pair with two visibly different types, call `swap`, and demonstrate the reversed result. Then add a focused test.

## Concepts in play

- Generic structs with more than one type parameter
- Consuming methods and ownership transfer
- Returning a related type with generic parameters in a different order

## Watch out for

Because `swap` takes `self` by value, do not expect to use the original pair after calling it, and do not add `.clone()` merely to avoid that move.

## References (read only if stuck)

- Chapter: `topics/rust/03-types-and-traits/index.md`
- Specific notes: `topics/rust/03-types-and-traits/generics.md` and `topics/rust/03-types-and-traits/structs.md`

## Checklist

- [x] I can explain each part of the required syntax
- [x] `cargo run` compiles and prints a stub
- [x] Implement the spec
- [x] `cargo clippy -- -D warnings` is clean
- [x] `cargo fmt` applied
- [x] Tests pass when the exercise requires them
- [x] Tell Codex "done" so `$journal` logs the session

## Run

```text
cd code/03-types-and-traits/pair-swap
cargo run
```
