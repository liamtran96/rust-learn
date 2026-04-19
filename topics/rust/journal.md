---
title: Learning Journal
tags: [rust, journal]
---

# Learning Journal

> Log confusions, aha-moments, and open questions here. Date each entry.
> Re-read your own entries weekly — you'll be surprised what used to confuse you.

## Template

```
### YYYY-MM-DD — Topic
**Working on:** ...
**What clicked:** ...
**What didn't:** ...
**Question to answer later:** ...
**Next:** ...
```

## Entries

<!-- Start your first entry below -->

### 2026-04-19 — FizzBuzz
**Working on:** FizzBuzz, first hands-on Rust program — `code/01-fundamentals/fizzbuzz/`
**What clicked:**
- `..=` inclusive range vs `..` exclusive — used `1..=100` correctly on the first try.
- Branch ordering: the combined `FizzBuzz` check has to come **before** the single-factor checks, otherwise 15 prints `Fizz`.
- `println!` is a macro (the `!`), positional `"{}"` placeholder fills from the trailing args.
- `%` remainder works on `i32` with no annotation needed.
**What didn't:** —  (ran cleanly first time; only `cargo fmt` nits: missing space before `{` and after `,`)
**Questions asked this session:**
- *What is cargo?* — answered: Rust's build tool + package manager (npm + webpack + make); `cargo new/run/build/check/fmt/clippy/add`; `Cargo.toml` is the manifest.
- *What is FizzBuzz?* — answered: the classic 1-to-100 divisibility exercise; exercises loops, conditionals, arithmetic, output.
- *What is `for..in` in Rust?* — answered: Rust's only for-loop form; iterates over anything implementing `Iterator`; `1..=100` for inclusive, `1..100` for exclusive; no C-style counter loop.
**Question to answer later:** —
**Next:**
- Refactor FizzBuzz using `match (i % 3, i % 5)` + `"{i}"` capture syntax — see what pattern matching feels like.
- Then Week 1 · Exercise 2: **temperature converter** (F ↔ C), which introduces `stdin`, `String`, and parsing.
