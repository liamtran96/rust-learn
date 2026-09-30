# Owned string and borrowed view — Brief

> You write the implementation yourself. Say “done” for `$journal` closeout.

## What you are building
A small review example: own some text in a `String`, borrow it as `&str`, and print both. This is the practice task agreed in chat, following the latest first move in `topics/rust/progress.md`; it is not a numbered Chapter 4 exercise.

## Expected input and output
Input is a string literal you choose in the source; output is stdout. For example, choose `hello Rust` and print that text once through its owner and once through its borrowed view. Formatting is your choice. No stdin or command-line arguments are required.

## Required Rust syntax
`fn main()` is the program's entry point: `fn` declares a function, `main` names it, `()` takes no arguments, and `{ ... }` holds its statements.

`let text = String::from("hello Rust");` creates an owned string: `let` binds a name, `=` supplies its value, `String::from` converts the literal into owned text, and `;` ends the statement.

Your next declaration starts with `let view: &str = ...;`. The colon introduces the type, `&str` means a shared string slice, and an `&` before a value borrows it. Choose the expression yourself. `println!` prints to stdout; `{name}` inside its format string inserts a named variable.

## Your coding steps
1. Open `src/main.rs` and create one owned `String` inside the existing function.
2. Add a shared `&str` view without cloning the owner.
3. Print both values, run the program, and explain who owns the text.

## Concepts in play
- Owned text versus a borrowed view
- Explicit type annotations and shared borrowing
- Printing values without transferring ownership

## Watch out for
Do not use `.clone()` to create the view; the task is to borrow existing text.

## References (read only if stuck)
- Concept: `topics/rust/04-collections/strings.md`
- Borrowing: `topics/rust/02-ownership/slices.md`
- Upcoming exercises: `topics/rust/exercises/ch04-collections.md`
- Session routine: `WORKFLOW.md`

## Checklist
- [ ] Explain the owner and borrowed view
- [ ] Create, borrow, and print the text
- [ ] `cargo run` prints both values
- [ ] `cargo clippy -- -D warnings` is clean
- [ ] `cargo fmt` applied
- [ ] Tell Codex “done” to record the session

## Run
```text
cd code/03-types-and-traits/string-views
cargo run
```

From the repository root, watch with `bacon string-views` or `bacon string-views-run`.
