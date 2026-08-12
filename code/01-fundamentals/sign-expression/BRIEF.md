# Sign expression — Brief

> Liam writes this himself. Codex only sets up the runway.
> When it compiles, runs, and Liam says “done,” use `$journal` to log it.

## Spec
Rewrite `fn sign(n: i32) -> &'static str` to return `"positive"`, `"negative"`, or `"zero"` without using the `return` keyword.

## Concepts in play
- Functions can return their final expression implicitly.
- `if`/`else if`/`else` is an expression in Rust.
- Every branch of an `if` expression must produce the same type.

## Watch out for
A trailing semicolon turns a value-producing expression into a statement returning `()`.

## References (read first if stuck)
- Chapter: `topics/rust/01-fundamentals/index.md`
- Specific notes: `topics/rust/01-fundamentals/functions.md`
- Specific notes: `topics/rust/01-fundamentals/control-flow.md`

## Checklist
- [ ] Read or skim the chapter notes
- [x] `cargo run` compiles and prints a stub
- [x] Implement the spec
- [x] `cargo clippy -- -D warnings` is clean
- [x] `cargo fmt` applied
- [x] Tests pass when the exercise requires them
- [x] Tell Codex “done” so `$journal` logs the session

## Run
```text
cd code/01-fundamentals/sign-expression
cargo run
```
