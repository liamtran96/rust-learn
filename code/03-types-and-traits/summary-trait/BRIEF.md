# Summary Trait — Brief

> You write the implementation yourself. This brief only explains the task and unfamiliar syntax.
> When it compiles, runs, and Liam says "done," use `$journal` to log it.

## What you are building

Define a `Summary` trait with one required method. Implement that trait for at least two different types, then write one notification function that uses a generic trait bound and another that accepts a mixed collection through trait objects.

## Expected input and output

The inputs are function arguments, not stdin or file data. `summarize` returns an owned `String`. If one of your types represents content whose summary is `Rust traits`, calling its `summarize` method should return a `String` containing `Rust traits`.

`notify` receives a shared reference to one concrete value that implements `Summary`. `notify_dyn` receives a borrowed slice of boxed, possibly different concrete types that all implement `Summary`. The specification does not prescribe exact stdout text or a return value for either notification function; choose a small observable behavior, such as printing each produced summary.

## Required Rust syntax

`trait Summary { fn summarize(&self) -> String; }`

- `trait Summary` declares a shared behavior named `Summary`.
- `fn summarize` declares a required method; each implementing type must provide its body.
- `&self` immutably borrows the value on which the method is called.
- `-> String` means the method returns an owned string.
- The semicolon declares the method signature without supplying a default implementation.

`fn notify<T: Summary>(s: &T)`

- `<T: Summary>` introduces a type parameter `T` and requires `T` to implement `Summary`.
- `s: &T` is a shared reference to a value of that concrete type.
- This is static dispatch: Rust knows the concrete `T` while compiling each call.
- With no return arrow, the function returns unit, `()`.

`fn notify_dyn(items: &[Box<dyn Summary>])`

- `items:` names the parameter.
- `&[...]` borrows a slice, so the function does not take ownership of the collection.
- `Box<...>` owns a value on the heap.
- `dyn Summary` means "some value implementing `Summary`" whose concrete type is selected at runtime.
- A slice of `Box<dyn Summary>` can therefore hold different implementing types together.

## Your coding steps

1. Declare `Summary`, add one small data type, and give it an `impl Summary` whose method returns a stub `String`; call it from `main` so the crate compiles early.
2. Add a second implementing type and write `notify<T: Summary>`; demonstrate it with a borrowed value.
3. Write `notify_dyn`, build a mixed collection of your two types, and verify that both summaries are observed. Add focused tests if they help confirm the behavior.

## Concepts in play

- Required trait methods and trait implementations
- Generic trait bounds and static dispatch
- Trait objects, `Box`, and dynamic dispatch

## Watch out for

Do not call `summarize` in a way that moves a boxed value out of the borrowed slice; iterate by shared reference.

## References (read only if stuck)

- Chapter: `topics/rust/03-types-and-traits/index.md`
- Specific notes: `topics/rust/03-types-and-traits/traits.md`
- Specific notes: `topics/rust/03-types-and-traits/trait-objects.md`

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
cd code/03-types-and-traits/summary-trait
cargo run
```
