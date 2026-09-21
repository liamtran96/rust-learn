# Animal trait — Brief

> You write the implementation yourself. This brief only explains the task and unfamiliar syntax.
> When it compiles, runs, and Liam says "done," use `$journal` to log it.

## What you are building

Define an `Animal` trait shared by `Dog` and `Cat`. Each type supplies its own name and sound, while the trait supplies a default `describe` method that combines them.

## Expected input and output

The inputs and outputs are method calls and return values—there is no required stdin or file input. Use `main` to print a small demonstration.

Example: for an animal whose `name()` returns `"Rex"` and whose `sound()` returns `"Woof"`, `describe()` returns `"Rex says Woof"`.

The fields or constructors used by `Dog` and `Cat` are intentionally unspecified; choose a simple representation yourself.

## Required Rust syntax

`trait Animal { fn name(&self) -> &str; fn sound(&self) -> String; fn describe(&self) -> String { format!("{} says {}", self.name(), self.sound()) } }`

- `trait Animal` declares a set of behavior that other types can implement.
- `fn name(...)` and `fn sound(...)` are required methods because the trait gives them no body.
- `&self` immutably borrows the current animal, so calling the method does not consume it.
- `-> &str` returns borrowed string text. Its lifetime is tied to the borrowed `self` by Rust's lifetime-elision rules.
- `-> String` returns owned string text.
- `describe` is a default method: its body is available to every implementor unless that type replaces it.
- `self.name()` and `self.sound()` call the methods required by the same trait.
- `impl Animal for Dog` means that `Dog` promises to provide the required `Animal` behavior. Use the same form for `Cat`.

## Your coding steps

1. Type the `Animal` trait, including the two required method declarations and the supplied default method, then compile.
2. Define `Dog`, implement the required methods for it, and print one description from `main`.
3. Define and implement `Cat`, then add focused tests for both types and the default description behavior.

## Concepts in play

- Traits as shared behavior
- Required methods versus default methods
- Implementing one trait for multiple concrete types
- Borrowed `&str` versus owned `String` return values

## Watch out for

Do not add a trailing semicolon to the final value-producing expression of a method that returns `String` or `&str`, because that would make the body return `()`.

## References (read only if stuck)

- Chapter: `topics/rust/03-types-and-traits/index.md`
- Specific notes: `topics/rust/03-types-and-traits/traits.md`

## Checklist

- [ ] I can explain each part of the required syntax
- [ ] `cargo run` compiles and prints a stub
- [ ] Implement the spec
- [ ] `cargo clippy -- -D warnings` is clean
- [ ] `cargo fmt` applied
- [ ] Tests pass when the exercise requires them
- [ ] Tell Codex “done” so `$journal` logs the session

## Run

```text
cd code/03-types-and-traits/animal-trait
cargo run
```
