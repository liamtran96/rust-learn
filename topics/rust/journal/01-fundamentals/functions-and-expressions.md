---
title: Fundamentals Journal - Functions And Expressions
tags: [rust, journal, fundamentals]
---

# Functions And Expressions

> Fundamentals topic journal. Return to [[../../journal|Journal index]].

## Entries

### 2026-05-24 — count_digits: loop { break value } in practice
**Working on:** Ch 1 Exercise #7 — `count_digits` — `code/01-fundamentals/count-digits/`
**What clicked:**
- `mut number: i32` in the parameter means "give me a mutable *copy* of the caller's value" — the caller's variable is unaffected.
- `break count` is what makes `loop { ... }` into an expression — the value after `break` is what the whole loop evaluates to.
- Early `return 1` before the loop is the clean way to handle edge cases that don't fit the main loop logic (the zero-digit problem).
**What didn't:**
- Tried `let result_count = loop { result_count += 1; }` — can't reference a name inside the loop that's *defining* it (it doesn't exist yet at that point).
- `result_count = loop { ... }` without `let` is a statement that returns `()`, leaving the function with no return value — caused a "mismatched types, found `()`" error.
- `break 1` hardcoded instead of `break count` — worked for the zero case but returned 1 for every input.
- `break count+1` as a fix for the zero case — off by one for all non-zero inputs.
**Questions asked this session:**
- **Q:** Why does `let result_count = loop { result_count += 1; }` fail with "cannot find value `result_count` in this scope"?
  - **Technical answer:** `let result_count = loop { ... }` is a single binding declaration. The name `result_count` is not yet in scope *inside* the right-hand side (the loop body), because the binding is only created once the whole expression on the right finishes evaluating. This is the same rule as `let x = x + 1` being valid (the old `x` is in scope on the right) but `let x = x` where `x` was never declared before — you can't reference something that hasn't been defined yet at all. The fix is to declare a *separate* counter variable before the loop: `let mut count = 0;`, then `loop { count += 1; break count; }`.
  - **Plain-English analogy / example:**
    ```rust
    // ❌ can't use result_count inside the loop that defines it
    let result_count = loop { result_count += 1; break result_count; };

    // ✅ separate counter declared before the loop
    let mut count = 0;
    let result_count = loop {
        count += 1;
        break count;
    };
    ```
  - **See also:** `topics/rust/01-fundamentals/variables.md`, `topics/rust/01-fundamentals/control-flow.md`
- **Q:** Why does `result_count = loop { ... }` cause "expected `i32`, found `()`"?
  - **Technical answer:** An assignment in Rust (`x = value`) is a **statement**, not an expression — it evaluates to `()` (called "unit", the empty tuple), not to the value being assigned. So when the function body ends with `result_count = loop { ... }`, its last "expression" is `()`, which doesn't match the `-> i32` return type. The fix is either to add `result_count` on the next line (making *that* the last expression), or to drop the assignment entirely and let the bare `loop { ... }` be the last expression — its `break count` value flows out as the return value directly.
  - **Plain-English analogy / example:**
    ```rust
    // ❌ assignment is a statement → returns ()
    fn f() -> i32 {
        let mut x = 0;
        x = loop { break 42; }; // this line returns ()
    }

    // ✅ loop as last expression → its break value is the return
    fn f() -> i32 {
        let mut x = 0;
        loop { break 42; } // no assignment; 42 flows out
    }
    ```
  - **See also:** `topics/rust/01-fundamentals/functions.md`, `topics/rust/01-fundamentals/control-flow.md`
- **Q:** Why does `break count+1` fix zero but break everything else?
  - **Technical answer:** The counter only increments in the `else` branch (when `number ≠ 0`). By the time `number` reaches 0, `count` already equals the number of digits — so `break count` is correct. `break count+1` adds an extra 1 to every result, not just the zero case. The right fix for zero is to catch it *before* the loop with `if number == 0 { return 1; }`, keeping the loop logic clean.
  - **Plain-English analogy / example:**
    ```rust
    fn count_digits(mut number: i32) -> i32 {
        if number == 0 { return 1; }   // edge case out front
        let mut count = 0;
        loop {
            if number == 0 { break count; } // count is already correct here
            count += 1;
            number /= 10;
        }
    }
    ```
  - **See also:** `topics/rust/01-fundamentals/control-flow.md`
**Question to answer later:** Is `if number == 0 { return 1; }` (early `return`) or handling it inside the loop idiomatic? When does Rust style prefer early `return` over restructuring the logic?
**Next:** Paper exercises #4-#6 are reviewed (answers in spec file) - update progress.md to reflect. Then Week 2: ownership & borrowing.

### 2026-08-12 — `sign`: `if` as an expression
**Working on:** Ch 1 Exercise #6 — `code/01-fundamentals/sign-expression/`
**What clicked:** A connected `if / else if / else` is one expression; each branch produces a `&'static str`, and the function implicitly returns that final expression without `return` or a trailing semicolon.
**What didn't:** The first attempt used two separate `if` expressions, discarded the first branch's string with `;`, reversed the positive/negative labels, and ignored the returned string in `main`. Each issue was corrected during review.
**Questions asked this session:**
- **Q:** What should I do next?
  - **Technical answer:** The progress log's explicit next move took priority: revisit Ch 1 Exercise #6 and implement `sign` using an `if` expression. This targeted the remaining expressions-versus-statements gap before returning to unfinished Week 1 project variants.
  - **Plain-English analogy / example:**
    ```text
    progress.md → Exercise #6
    exercise spec → sign without return
    completed crate → journal reflection
    ```
  - **See also:** `topics/rust/progress.md`, `topics/rust/exercises/ch01-fundamentals.md`
- **Q:** Help me create the exercise.
  - **Technical answer:** A standalone binary crate was scaffolded with Cargo, a `BRIEF.md` was derived from Exercise #6, and Bacon jobs were registered without implementing the solution. A crate is an independently buildable Rust package described by `Cargo.toml`.
  - **Plain-English analogy / example:**
    ```text
    code/01-fundamentals/sign-expression/
    ├── Cargo.toml
    ├── BRIEF.md
    └── src/main.rs
    ```
  - **See also:** `topics/rust/01-fundamentals/toolchain.md`
- **Q:** Run `bacon sign-expression-run`; help me install it.
  - **Technical answer:** Bacon is a file-watching development tool that reruns a configured Cargo job whenever watched source files change. It was installed with `cargo install bacon`; the repository job then maps `sign-expression-run` to `cargo run` in this exercise crate.
  - **Plain-English analogy / example:**
    ```text
    cargo install bacon
    bacon sign-expression-run
    # edit src/main.rs; Bacon reruns the job
    ```
  - **See also:** `topics/rust/cheatsheets/cargo-commands.md`
- **Q:** Check this `sign` implementation — is the expression structure correct?
  - **Technical answer:** A semicolon turns a value-producing expression into a statement whose value is `()`, called the unit type. Separate `if` statements do not form one conditional value; connecting the branches makes the entire conditional the function's final `&'static str` expression.
  - **Plain-English analogy / example:**
    ```rust
    fn sign(n: i32) -> &'static str {
        if n < 0 { "negative" }
        else if n > 0 { "positive" }
        else { "zero" }
    }
    ```
  - **See also:** `topics/rust/01-fundamentals/functions.md`, `topics/rust/01-fundamentals/control-flow.md`
**Question to answer later:** —
**Next:** Finish the iterator-`map` version of FizzBuzz, then refactor the temperature converter to accept a single argument such as `25C`.

### 2026-08-17 — Semicolons and block values review
**Working on:** FizzBuzz iterator review — `code/01-fundamentals/fizzbuzz/`
**What clicked:** The practical rule is “no final semicolon gives the value back; a final semicolon performs the expression and discards its value.” A block without a returned value evaluates to unit, written `()`.
**What didn't:** The difference between an expression that produces a value and a statement that discards it still feels abstract, so the explanation needed a “box gives back versus throws away” analogy.
**Questions asked this session:**
- **Q:** “What is the difference when I add `;` at the end of each line for this logic?”
  - **Technical answer:** An expression produces a value, while adding `;` turns it into a statement and discards that value. In these `if` branches, no semicolon returns a `String`; adding one makes the branch return unit `()`, Rust's “no useful value” type.
  - **Plain-English analogy / example:**
    ```rust
    let word = { "Fizz".to_string() };  // word: String
    let empty = { "Fizz".to_string(); }; // empty: ()
    println!("{word}");
    ```
  - **See also:** `topics/rust/01-fundamentals/functions.md`, `topics/rust/01-fundamentals/control-flow.md`
- **Q:** “It is hard to understand it.”
  - **Technical answer:** The simplest mental model is that braces form a box: the final expression without `;` is handed out of the box. With `;`, the box performs the work but throws its result away, so it hands out `()` instead.
  - **Plain-English analogy / example:**
    ```text
    { make_string()  }  -> box gives String back
    { make_string(); }  -> box throws String away
    Empty result         -> ()
    ```
  - **See also:** `topics/rust/01-fundamentals/functions.md`
**Question to answer later:** How does the same tail-expression rule determine the return value of functions and `match` arms?
**Next:** Refactor `code/01-fundamentals/temp-converter/` to accept one CLI argument such as `25C`, then begin Phase 2 ownership.

