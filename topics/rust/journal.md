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

### 2026-05-01 — stdin & the temperature converter
**Working on:** Temperature converter — `code/01-fundamentals/temp-converter/`
**What clicked:**
- `stdin` is a stream the OS opens for the process (fd 0); `io::stdin()` returns a Rust handle to it, doesn't create it.
- `read_line(&mut s)` appends bytes **including the trailing `\n`** — that's why `.trim()` is needed before `parse()`.
- `read_line` returns `io::Result<usize>`, so it must be handled (`.expect(...)` for now, `?` later).
- Pipe redirection (`echo ... | cargo run`, `cargo run < file`) works transparently because stdin is a stream, not specifically the keyboard.
- Variable shadowing used cleanly: `let unit = unit.trim().to_uppercase();` rebinds the same name to a new owned `String`.
- `match unit.as_str()` — needed because `String` doesn't pattern-match against `&str` literals directly.
**What didn't:** —
**Questions asked this session:**
- *What is stdin?* — answered: standard input, fd 0, a byte stream the OS hands to every process; `io::stdin()` is just a handle to it; works with keyboard, pipes, or file redirection.
**Question to answer later:**
- The `celsius_to_fahrenheit` fn at the bottom is dead code — should it be removed, or is it kept on purpose? (also `f_to_c`/`c_to_f` duplicate it.)
- Why does each `io::stdin()` call lock the stream — when does that matter in practice?
**Next:**
- Clean up the commented-out first-pass code and the unused `celsius_to_fahrenheit` duplicate.
- Week 1 · Exercise 3: **guessing game** — introduces `rand`, `Ordering`, looping until correct, and `match` on enum variants.

### 2026-05-03 — Guessing game (in progress) — `Result`, `match`, and reading the compiler
**Working on:** Guessing game — `code/01-fundamentals/guessing-game/` (loop logic working; `rand` integration mid-debug)
**What clicked:**
- `read_line` **appends** to its buffer — that's why `guess.clear()` is needed before each iteration of the input loop. Reading once before `loop` and reparsing the same string forever was the cause of the "infinite Too small!" bug.
- `match` is a **language keyword** (control-flow expression), not a function — it's an expression that evaluates to a value, enforces exhaustiveness at compile time, and uses *patterns* on the left of `=>`, not values.
- `Ok(...)` / `Err(...)` aren't function calls — they're **variants of the `Result<T, E>` enum**, used in two directions: as constructors (`return Ok(42)`) and as destructuring patterns (`Ok(n) => …`). Same syntax, opposite meaning depending on side of `=>`.
- `Result<T, E>` mental model: a sealed box labelled either `Ok` (holds a `T`) or `Err` (holds an `E`); the compiler refuses to let you use the inner value until you've handled both labels.
- `Err(error)` with an unused name triggers a clippy/unused-variable error — the convention is `Err(_)` (or `_error`) to mark the binding as intentionally discarded.
- **Trait methods need the trait in scope.** `random_range` lives on the `RngExt` trait in `rand` 0.10, so even with the right RNG type you need `use rand::RngExt;`. Same shape as `Iterator`, `Write`, `AsyncReadExt` — extension traits are invisible until imported.
- Tutorial drift: `rand` 0.9 renamed `thread_rng()` → `rng()` and `gen_range` → `random_range`; the Rust book still pins 0.8 so its examples don't compile against 0.10. Lesson: check `Cargo.toml` version against docs.rs, not blog posts.
**What didn't:**
- The "single read, looped parse" bug took a while to spot — instinct was that `loop` re-runs everything inside it, but `read_line` was outside the loop.
- The `RngExt` import wasn't obvious from outside; rustc's "perhaps you want to import it" hint with the literal `1 + use rand::RngExt;` suggestion was the thing that made it fall into place. Reinforces: read the **whole** error, not just the first red line.
**Questions asked this session:**
- *What is `Ok` — is it a function?* — answered: no, it's a variant of the `Result` enum; in `match` it's a destructuring pattern, in normal expression position it's a constructor.
- *Is `match` a built-in function?* — answered: no, it's a language keyword (control-flow expression); enforces exhaustiveness, takes patterns not values, returns a value.
- *What is `guess.clear()`?* — answered: a method call on `String` that empties the buffer in place (length 0, capacity preserved); needed because `read_line` appends rather than overwrites.
- *What is `enum Result<T, E> { Ok(T), Err(E) }`?* — answered: a generic tagged-union (sum type) from std; `<T, E>` are type parameters filled in per use site; both variants must be handled.
**Question to answer later:**
- Once `random_range` compiles — does the `use rand::Rng;` line become redundant, or is `Rng` still needed alongside `RngExt`? (Drop it and let clippy decide.)
- Worth revisiting after Ch 3 (enums) and Ch 5 (error handling) — the `Result` story should feel obvious in hindsight rather than memorized.
**Next:**
- Finish guessing game: add `use rand::RngExt;`, confirm `cargo run` plays a full round end-to-end, run `cargo fmt` + `cargo clippy -- -D warnings` clean. Then tick the Week 1 guessing-game milestone.
- Owed from earlier: iterator-`map` version of FizzBuzz, and the `25C`-style single-CLI-arg refactor of temp-converter.
- Then Ch 1 paper exercises #4–#7 (predict shadowing, type-mismatch fix, eliminate `return`, `loop`-with-`break value` for `count_digits`).

### 2026-05-03 — Guessing game shipped (closeout)
**Working on:** Guessing game — `code/01-fundamentals/guessing-game/` — runs end-to-end against a real random target, clippy clean.
**What clicked:** The `RngExt` import resolved instantly once read in the rustc hint — the lesson "trait methods need the trait in scope" now has an episodic memory attached to it. End-to-end the program does what the spec asked: prompt → parse → compare → loop until correct.
**What didn't:** Code is functional but not yet *clean* — see review notes below; left open as cleanup pass before closing out Week 1.
**Questions asked this session:** —
**Question to answer later:**
- Is the `let _result = loop { … }` binding ever idiomatic, or always a smell when the `break` value is unused? (Probably the latter.)
**Next:** Cleanup pass on guessing-game (delete dead comments, drop unused `_result`, run `cargo fmt`), then Ch 1 #4 paper exercise.
