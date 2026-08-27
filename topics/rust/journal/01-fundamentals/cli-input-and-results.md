---
title: Fundamentals Journal - Cli Input And Results
tags: [rust, journal, fundamentals]
---

# Cli Input And Results

> Fundamentals topic journal. Return to [[../../journal|Journal index]].

## Entries

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

### 2026-08-17 — Temperature converter CLI arguments
**Working on:** Temperature converter CLI follow-up — `code/01-fundamentals/temp-converter/`
**What clicked:** `std::env::args()` exposes command-line arguments; the executable path is element 0 and the first user argument is element 1. Length checks prevent indexing panics, `split_at` separates the ASCII unit suffix, `parse()` returns a `Result`, `match` selects conversion behavior, and `:.2` formats a float to two decimal places.
**What didn't:** The old stdin flow initially remained after argument collection, so the program still prompted for two values. The purpose of collecting into `Vec<String>`, borrowing `&args[1]`, `std::env` provenance, and the difference between `Display` and `Debug` formatting all needed explanation; cleanup and `cargo fmt` remain unfinished.
**Questions asked this session:**
- **Q:** What should I do next?
  - **Technical answer:** Replace the stdin flow incrementally: first collect command-line arguments, inspect them, then extract and validate the first user argument. `env::args()` is an iterator, meaning it yields arguments one at a time, and `collect()` gathers them into a `Vec<String>`.
  - **Plain-English analogy / example:**
    ```rust
    use std::env;
    let args: Vec<String> = env::args().collect();
    println!("{args:?}");
    ```
  - **See also:** `topics/rust/01-fundamentals/data-types.md`, `topics/rust/exercises/ch01-fundamentals.md`
- **Q:** Why did the program still print `Conver C to F`, prompt for a temperature, and then say `Invalid unit`?
  - **Technical answer:** Collecting `args` did not remove or bypass the old stdin statements below it. The entered `12` became the old program's unit, did not match `"C"` or `"F"`, and therefore selected the wildcard invalid-unit arm.
  - **Plain-English analogy / example:**
    ```rust
    let args = std::env::args().collect::<Vec<_>>();
    println!("{args:?}"); // new flow runs
    old_stdin_flow();     // old flow still runs afterward
    ```
  - **See also:** `topics/rust/01-fundamentals/control-flow.md`
- **Q:** How do I format output like `25.00 C = 77.00 F`?
  - **Technical answer:** Rust format specifications go after a colon inside a placeholder. `:.2` requests fixed-point display with two digits after the decimal, and an unnamed placeholder consumes the next value passed to `println!`.
  - **Plain-English analogy / example:**
    ```rust
    let c = 25.0;
    let f = 77.0;
    println!("{c:.2} C = {f:.2} F");
    ```
  - **See also:** `topics/rust/01-fundamentals/functions.md`
- **Q:** What is the `args` length/input validation block doing?
  - **Technical answer:** `args` is a vector whose first element is the executable path, so the user's temperature is at index 1. Checking `args.len() < 2` before indexing prevents a panic, `&args[1]` borrows the `String` rather than moving it, and `is_empty()` handles an explicitly empty argument.
  - **Plain-English analogy / example:**
    ```rust
    if args.len() < 2 {
        println!("Usage: temp-converter <temperature>");
        return;
    }
    let input = &args[1];
    ```
  - **See also:** `topics/rust/01-fundamentals/data-types.md`
- **Q:** Where does `use std::env;` come from?
  - **Technical answer:** `std` is Rust's standard library and is available without adding a Cargo dependency. `env` is its environment module; `use std::env` brings that module into scope so `std::env::args()` can be written as `env::args()`.
  - **Plain-English analogy / example:**
    ```rust
    use std::env;
    let short = env::args();
    let full = std::env::args();
    // Both call the same standard-library function.
    ```
  - **See also:** `topics/rust/01-fundamentals/toolchain.md`
- **Q:** How can I see the real value of `args`, and why does `println!("{}", args)` fail?
  - **Technical answer:** `{}` requires `Display`, the trait for intentional user-facing text, and `Vec<String>` does not define one unambiguous display representation. Vectors implement `Debug`, the trait for inspecting structure, so `{:?}` prints the elements and `{:#?}` pretty-prints them across lines.
  - **Plain-English analogy / example:**
    ```rust
    let args = vec!["program".to_string(), "77F".to_string()];
    println!("{args:?}");
    println!("{args:#?}");
    ```
  - **See also:** `topics/rust/01-fundamentals/functions.md`
**Question to answer later:** How can the program extract the final unit character without relying on a one-byte ASCII boundary?
**Next:** Remove the temporary `args` debug print and stale commented code, run `cargo fmt`, then verify `fmt`, `check`, `test`, and Clippy before beginning Phase 2 ownership.

### 2026-08-17 — Temperature converter verification and Phase 1 closeout
**Working on:** Temperature converter CLI follow-up — `code/01-fundamentals/temp-converter/`
**What clicked:** A crate can pass formatting, compilation, tests, and Clippy even when optional cleanup comments remain; those checks verify defined formatting and code-quality rules, not whether every comment should be removed. Phase 1's seven exercises and three shipped projects are now closed out in the learning trackers.
**What didn't:** The existing comments and temporary argument debug print remain by choice; they did not block verification.
**Questions asked this session:**
- **Q:** “Please don't care about the comment; run check and move to the next phase.”
  - **Technical answer:** Comments do not affect compilation, so they can remain without preventing verification. `cargo fmt --check`, `cargo check`, `cargo test`, and `cargo clippy -- -D warnings` all passed, so the official tracker can advance to Phase 2, Week 2 while keeping the completed exercise and project counts unchanged.
  - **Plain-English analogy / example:**
    ```text
    Comments kept     -> no compiler effect
    Verification      -> all four checks passed
    Official tracker  -> Phase 2, Week 2
    Next concept      -> ownership
    ```
  - **See also:** `topics/rust/cheatsheets/cargo-commands.md`, `topics/rust/02-ownership/ownership.md`
**Question to answer later:** How can the program extract the final unit character without relying on a one-byte ASCII boundary?
**Next:** Read `topics/rust/02-ownership/ownership.md`, focusing on move, copy, and drop.

