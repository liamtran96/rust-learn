# Typed IDs - Brief

> You write the implementation yourself. This brief only explains the task and unfamiliar syntax.
> When it compiles, runs, and Liam says "done," use `$journal` to log it.

## What you are building

Create two distinct ID types, `UserId` and `OrderId`, that both store a `u64`. Then write a function whose parameter accepts one of those ID types so the compiler rejects the other type.

This models a common application bug: a user and an order may both have the numeric ID `7`, but they do not mean the same thing.

## Expected input and output

The input is a function argument, not stdin or a file. The observable behavior is primarily whether the program type-checks.

Example: a function that asks for `UserId` must accept the user ID containing `7`. Passing an `OrderId` containing the same number must produce a compiler type-mismatch error. What the accepted function call prints or returns is intentionally open-ended.

## Required Rust syntax

```rust
struct TypeName(InnerType);
fn function_name(parameter: TypeName) { /* body */ }
```

- `struct` declares a new type.
- `TypeName` is the distinct name Rust checks; your exercise needs one name for users and another for orders.
- `(InnerType)` makes this a tuple struct with one unnamed field. Here, the inner type is `u64`.
- The trailing `;` completes a tuple-struct declaration because there is no `{ ... }` body.
- `fn` declares a function.
- `parameter: TypeName` says the argument must have that exact wrapper type, even if another wrapper contains the same inner type.
- `{ /* body */ }` is behavior you will choose and write.

## Your coding steps

1. Run the generated program unchanged once, then replace it with the smallest stub that still compiles.
2. Declare the two one-field ID wrapper types and construct one value of each in `main`.
3. Add a function that accepts the user-ID type, call it with the matching value, and confirm the program compiles.
4. Temporarily try the order-ID value at that call site, read the full compiler error, then restore compiling code.

## Concepts in play

- Tuple structs
- The newtype pattern
- Compile-time type safety

## Watch out for

Do not replace both wrappers with plain `u64` parameters; that removes the distinction this exercise is designed to enforce.

## References (read only if stuck)

- Chapter: `topics/rust/03-types-and-traits/index.md`
- Specific notes: `topics/rust/03-types-and-traits/structs.md`

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
cd code/03-types-and-traits/typed-ids
cargo run
```
