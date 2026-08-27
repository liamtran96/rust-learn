---
title: Fundamentals Journal - Control Flow And Iterators
tags: [rust, journal, fundamentals]
---

# Control Flow And Iterators

> Fundamentals topic journal. Return to [[../../journal|Journal index]].

## Entries

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

### 2026-08-14 — Iterator `map` FizzBuzz
**Working on:** FizzBuzz iterator follow-up — `code/01-fundamentals/fizzbuzz/`
**What clicked:** `map` takes a closure and transforms every input into an output; the closure's final expression determines the iterator's item type. `println!` performs a side effect and returns `()`, while `format!` and `to_string()` produce owned `String` values. Removing a trailing semicolon preserves a block's tail-expression value.
**What didn't:** At first the range, `map`, and closure syntax were combined incorrectly. Then printing inside the closure—and later terminating each `String` expression with `;`—made the iterator produce `()` instead of printable strings.
**Questions asked this session:**
- **Q:** “Wait, what do I need to write—the FizzBuzz again?”
  - **Technical answer:** No full rewrite was required. The original specification requested three implementations, and only the third approach—transforming the range with iterator `map`—remained as follow-up work.
  - **Plain-English analogy / example:**
    ```text
    Version 1: for loop + if
    Version 2: for loop + match
    Version 3: iterator + map
    ```
  - **See also:** `topics/rust/exercises/ch01-fundamentals.md`
- **Q:** “The hell what u mean?”
  - **Technical answer:** The records were inconsistent: official progress already said 7/7 exercises and three shipped projects, while the latest daily note still named iterator FizzBuzz as the next follow-up. The official completion status stayed unchanged; only the unfinished variant needed clarification.
  - **Plain-English analogy / example:**
    ```text
    Official tracker: exercise complete
    Session note: one optional variant remains
    Result: keep completion; finish variant as follow-up
    ```
  - **See also:** `topics/rust/progress.md`
- **Q:** “Finish the iterator-map version of FizzBuzz—what does this mean?”
  - **Technical answer:** An iterator yields values one at a time, and `map` applies a closure—an unnamed function—to transform each value. Here, each number from 1 through 100 becomes a `String` containing the number, `Fizz`, `Buzz`, or `FizzBuzz`.
  - **Plain-English analogy / example:**
    ```rust
    let doubled = (1..=3).map(|n| n * 2);
    for value in doubled {
        println!("{value}");
    }
    ```
  - **See also:** `topics/rust/exercises/ch01-fundamentals.md`, `topics/rust/08-closures-iterators/iterators.md`
- **Q:** “Check it” (`1..=100.map(i) >= { ... }`).
  - **Technical answer:** A range should be grouped as `(1..=100)`, and `map` expects a closure written `|i| { ... }`. The token `>=` is the greater-than-or-equal comparison operator and does not introduce a closure body.
  - **Plain-English analogy / example:**
    ```rust
    let values = (1..=3).map(|i| {
        i * 2
    });
    ```
  - **See also:** `topics/rust/08-closures-iterators/iterators.md`
- **Q:** “Still error: cannot be formatted with default formatter.”
  - **Technical answer:** `println!` returns the unit type `()`, Rust's “no meaningful value” type, so printing inside every `map` branch made `result` equal to `()`. Unit supports debug formatting with `{:?}`, but the intended fix was for `map` to return `String` values.
  - **Plain-English analogy / example:**
    ```rust
    let result = println!("hello"); // result is ()
    println!("{result:?}");         // prints ()
    // println!("{result}");        // Display is not implemented
    ```
  - **See also:** `topics/rust/01-fundamentals/functions.md`
- **Q:** “Still error” after changing branches to `.to_string();`.
  - **Technical answer:** A trailing semicolon turns a value-producing expression into a statement whose value is `()`. Each branch needed its `String` as the final expression without `;`, allowing the entire `if` expression—and therefore the closure—to return `String`.
  - **Plain-English analogy / example:**
    ```rust
    let text = {
        "Fizz".to_string()
    };
    println!("{text}");
    ```
  - **See also:** `topics/rust/01-fundamentals/control-flow.md`, `topics/rust/01-fundamentals/functions.md`
- **Q:** “Is there any way to format it?”
  - **Technical answer:** The debug formatter `{:?}` can format `()`, but it would only print `()` and would hide the real data-flow problem. `format!` is different: it constructs and returns a `String` instead of printing immediately.
  - **Plain-English analogy / example:**
    ```rust
    let debug_unit = ();
    println!("{debug_unit:?}");
    let text = format!("number: {}", 15);
    ```
  - **See also:** `topics/rust/01-fundamentals/functions.md`
- **Q:** “Any built-in lib?”
  - **Technical answer:** No external crate is needed. `format!` is a standard macro, and `to_string()` comes from the standard `ToString` trait, which Rust's prelude automatically brings into scope.
  - **Plain-English analogy / example:**
    ```rust
    let a = 15.to_string();
    let b = format!("{}", 15);
    assert_eq!(a, b);
    ```
  - **See also:** `topics/rust/04-collections/strings.md`
**Question to answer later:** When should iterator code use `map` to transform values versus `for_each` for side effects?
**Next:** Refactor `code/01-fundamentals/temp-converter/` to accept one CLI argument such as `25C`, then begin Phase 2 ownership.

