# count_digits — Brief

> Liam writes this himself. Claude only sets up the runway.
> When it compiles, runs, and you say "done", run `/journal` to log it.

## Spec

Write `fn count_digits(n: u32) -> u32` using a `loop` with `break value`.
The function should return the number of decimal digits in `n` (e.g. `count_digits(1234)` → `4`, `count_digits(0)` → `1`).
Call it from `main` with a few test values and print the results.

## Concepts in play

- `loop { break value }` — a loop is an expression; `break <expr>` is its return value
- `mut` parameters — `mut n: u32` lets you modify your local copy of the argument
- Integer division (`/=`) as the digit-stripping mechanism

## Watch out for

`loop` without `break value` returns `()`, not a number — if you write `let result = loop { ... }` and forget to put `break count` (not just `break`), the compiler will complain that the types don't match.

## References (read first if stuck)

- Chapter: `topics/rust/01-fundamentals/index.md`
- Specific notes: `topics/rust/01-fundamentals/control-flow.md`
- Answers (only after you've tried): `topics/rust/exercises/ch01-fundamentals.md` → Q7

## Checklist

- [ ] Read the control-flow notes above (or skim if already read)
- [ ] `cargo run` compiles and prints something — even a stub
- [ ] Implement `count_digits` using `loop { break value }` — no `.to_string().len()` shortcut
- [ ] Handle the `n == 0` edge case (should return `1`)
- [ ] `cargo clippy -- -D warnings` is clean
- [ ] `cargo fmt` applied
- [ ] Tell Claude "done" → `/journal` logs it and ticks the roadmap

## Run

```
cd code/01-fundamentals/count-digits
cargo run
```
