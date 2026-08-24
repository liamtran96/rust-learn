# Ownership drills — Brief

> Liam writes every fix and implementation himself. The compiler is the grader.
> When all 12 are green and the checkpoint is done, say "done" so `$journal` logs it.

## Spec

Twelve tiny drills under `tests/`, one file each, ordered easy → hard. Two kinds:

- **Fix-it** (d01–d08, d11): the file intentionally does not compile, or an assert fails. Make the *minimal* fix.
- **Implement** (d09, d10, d12): a `todo!()` body plus tests. Make the tests pass.

Each drill compiles as its own target, so one broken drill never blocks another.

## The loop (this is the whole method)

1. Pick the next drill file. Read it — but **before running anything**, fill in the `PREDICT:` line: will it compile? why?
2. Run exactly one drill: `cargo test --test d01_move` (use the file name).
3. Read the compiler error yourself before fixing. The error *is* the lesson.
4. Fix minimally, re-run until green.
5. Fill in the `WHY:` line — the rule your fix relied on — **before** moving to the next drill. A green drill with an empty WHY doesn't count.

## Rules

- No `.clone()` unless you justify it in the WHY line (see `topics/rust/pitfalls.md`).
- No `unwrap()` outside what the drill already contains.
- Minimal fixes — don't rewrite the drill so the conflict disappears.

## Concepts in play

- Move vs `Copy` semantics
- Shared (`&`) vs exclusive (`&mut`) borrows and their scopes
- `&str` / `&[T]` over `&String` / `&Vec<T>`
- Lifetimes: elision, `<'a>`, dangling references
- Expressions vs statements (the trailing-`;` trap)

## Watch out for

Don't reach for `.clone()` to silence the borrow checker — sit with the error first.

## References (read first if stuck)

- Chapter: `topics/rust/02-ownership/index.md`
- Exercise source: `topics/rust/exercises/ch02-ownership.md`
- Common bugs: `topics/rust/pitfalls.md`

## Checklist

- [ ] d01–d12 all green: `cargo test` passes at crate level
- [ ] Every PREDICT and WHY line filled in
- [ ] `cargo clippy --all-targets -- -D warnings` is clean
- [ ] `cargo fmt` applied
- [ ] Checkpoint: explain the four errors at the bottom of `topics/rust/exercises/ch02-ownership.md` without notes
- [ ] Tell Codex/Claude "done" so `$journal` logs the session

## Run

```text
cd code/02-ownership/drills-ownership
cargo test --test d01_move     # one drill
cargo test                     # everything (the finish line)
```
