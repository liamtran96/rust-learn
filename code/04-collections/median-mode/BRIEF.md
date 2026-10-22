# Median & mode - Brief

> You write the implementation yourself. This brief only explains the task and unfamiliar syntax.
> When it compiles, runs, and Liam says "done," use `$journal` to log it.

## What you are building
Summarize a list of integers with its median and mode, as a reporting app might summarize response times. The median is the middle value in sorted order (the average of the two middle values for an even count); the mode is the most frequent value.

## Expected input and output
Input means a function argument of type `Vec<i32>`; output means a returned `(f64, i32)` tuple in median-then-mode order. Choose your own function name and sample data in `main`; no stdin, files, or CLI arguments are required.

Example: input `vec![4, 1, 4, 2]` returns `(3.0, 4)`. Printing the returned result is useful for checking it; stdout formatting is your choice.

The source does not specify empty-input behavior or how to select one mode when values tie. Start with non-empty examples with a unique mode, then choose and document those policies before calling the exercise complete; do not silently invent a numeric answer for empty input.

## Required Rust syntax
One possible function header matching the specification is:

```rust
fn median_mode(numbers: Vec<i32>) -> (f64, i32)
```

- `fn` declares a function; `median_mode` is its chosen name.
- `(numbers: Vec<i32>)` declares one parameter named `numbers`; `:` introduces its type.
- `Vec` is a growable list; `<i32>` says its elements are signed 32-bit integers.
- Taking `Vec<i32>` by value transfers ownership into the function when called; the caller cannot reuse that binding afterward. No clone is needed for this input contract.
- `->` introduces the return type.
- `(f64, i32)` is a tuple: two values returned together. `f64` holds the median, including fractional results; `i32` holds the mode.
- Your function body will go inside `{ ... }` and must produce that tuple; a trailing semicolon discards an expression's value.

To start before writing the function, create and inspect sample data inside the generated `main`:

```rust
let numbers: Vec<i32> = vec![4, 1, 4, 2];
println!("{numbers:?}");
```

`let` introduces a binding; `=` supplies its value. `vec!` is a macro (marked by `!`); the comma-separated values in `[...]` become the vector's elements. `;` ends each statement. `println!` prints a line, and `{numbers:?}` inserts `numbers` using debug formatting (`:?`), which can display a vector.

## Your coding steps
1. Replace Hello World with a small vector and print it; run the program before adding calculations.
2. Work out the expected median by hand, then implement median behavior for odd and even lengths.
3. Add mode behavior and return both results together. Choose your own counting approach; ask for syntax help before using unfamiliar collection APIs.
4. Check examples with one element, repeated values, negative values, and a fractional median. Document and check your tie and empty-input policies.

## Concepts in play
- Owned vectors and their elements
- Ordering, middle positions, and counting occurrences
- Returning two different numeric types in a tuple

## Watch out for
For an even count, the median can be fractional: integer arithmetic can lose the fractional part, and adding two large `i32` values can overflow.

## References (read only if stuck)
- Spec: `topics/rust/exercises/ch04-collections.md` - exercise 1, Median & mode
- Chapter: `topics/rust/04-collections/index.md`
- Specific notes: `topics/rust/04-collections/vec.md`; `topics/rust/04-collections/hashmap.md` if you choose a map for counting
- Ownership reminder: `topics/rust/02-ownership/ownership.md`
- Cargo commands: `topics/rust/cheatsheets/cargo-commands.md`
- Official syntax: [Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html), [Tuples](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type), [Debug formatting](https://doc.rust-lang.org/std/fmt/trait.Debug.html)

## Checklist
- [ ] I can explain each part of the required syntax
- [ ] `cargo run` compiles and prints the initial vector
- [ ] Implement the spec
- [ ] Document empty-input and tied-mode policies
- [ ] `cargo clippy -- -D warnings` is clean
- [ ] `cargo fmt` applied
- [ ] Verify examples, including odd/even lengths and a fractional median
- [ ] Tests pass when added
- [ ] Tell Codex "done" so `$journal` logs the session

## Run
```text
cd code/04-collections/median-mode
cargo run
```

From the repository root: `bacon median-mode` checks your work; `bacon median-mode-run` runs it.
