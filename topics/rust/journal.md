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

### 2026-05-05 — `const` vs `static`, paper exercises #4–#7 reviewed
**Working on:** Ch 1 paper exercises #4–#7 (`topics/rust/exercises/ch01-fundamentals.md`) + scratch crate `code/01-fundamentals/scratch-static-counter/` exploring `let mut` vs `const` vs `static mut` vs `static + Mutex`.
**What clicked:**
- **`const` is a value, not a variable.** It's inlined at every use site like a compile-time find-and-replace; there's no storage to mutate, which is why `_CONST_COUNTER += 1;` produces `error[E0070]: invalid left-hand side of assignment` rather than a "missing `mut`" hint.
- **`static` is a variable** — one fixed memory address, lives the entire program. `static mut` works but every access needs `unsafe` because two threads could race; the 2024 edition makes implicit refs to `static mut` a hard error, forcing `&raw mut`.
- **`static FOO: Mutex<T> = Mutex::new(0)` is the idiomatic global mutable.** The `static` itself is immutable; mutability lives *inside* the `Mutex`. `lock()` returns a `MutexGuard` that derefs to `&mut T` and auto-releases on drop.
- **Mermaid is built into Obsidian** — flowcharts, sequence diagrams, gantt for lifetimes, all without installing anything; render in Reading view (Cmd-E).
- **Shadowing arithmetic chain** — `let x=5; let x=x+1; let x=x*2;` is `5 → 6 → 12`, not `10`. The right side evaluates with the *current* `x` *before* the new binding is made.
- **Block-scoped shadowing** — inner `let y = y + 1;` creates a new `y` that dies at `}`, leaving the outer `y` untouched. Got this one right.
**What didn't:**
- **Type annotation ≠ type cast.** Tried to "fix" `i32 + i64` by adding `let a: i32 = 100`, which is a no-op against `100_i32`. The error is at the `+`, not the `let`s — Rust never widens silently; need an `as` cast.
- **Expressions vs statements** — couldn't write the no-`return` version of `sign()` ("i dont know"). The whole idea that `if/else` is an expression and a function body is a block whose final expression (no `;`) is the return value hasn't internalized yet. This is the foundational Rust idiom; everything in Weeks 2–3 will lean on it.
- **`loop` is not `match`** — wrote `loop { Ok() => ..., Err() => ... }` mashing `match` arm syntax into a `loop` body. Also reached for `Result` / `Ok` / `Err` on a `count_digits(u32) -> u32` problem that has no failure mode.
- **Keyword salad** — wrote `static const result: i32 mut = 0`. Three mutually exclusive variable kinds welded together with `mut` in the wrong slot.
**Questions asked this session:**
- **Q:** What is `.len()` in `let spaces = spaces.len();`?
  - **Technical answer:** `.len()` is a **method call** — a function defined on a specific type, called via `value.method()` syntax. On `&str` (a string slice) it returns the number of **bytes** in the string as a `usize` (Rust's pointer-sized unsigned integer). The method exists separately on `&str`, `String`, `Vec<T>`, `&[T]`, `HashMap`, etc. — there's no universal `len` function, just a convention. Watch out: `.len()` returns bytes, not characters; for character count use `.chars().count()`.
  - **Plain-English analogy / example:**
    ```rust
    let s = "héllo";
    println!("{}", s.len());           // 6 — 'é' is 2 UTF-8 bytes
    println!("{}", s.chars().count()); // 5 — actual character count
    ```
  - **See also:** `topics/rust/01-fundamentals/data-types.md`, `topics/rust/04-collections/strings.md` (Week 5)
- **Q:** What does "creates" mean in "shadowing creates a new variable"?
  - **Technical answer:** Plain English, not a Rust keyword. When you `let` a name a second time in the same scope, the compiler allocates a fresh binding (a new variable slot) and points the name at it; the old variable still exists in memory until its scope ends, but the *name* no longer reaches it. Different from `mut`, which reuses the same slot and overwrites the value in place.
  - **Plain-English analogy / example:**
    ```rust
    let x = 5;        // create variable #1, name "x" points to it
    let x = "hi";     // create variable #2 (different type!), name "x" now points to #2
    let mut y = 5;
    y = 6;            // no new variable — same slot, new value
    ```
  - **See also:** `topics/rust/01-fundamentals/variables.md`
- **Q:** What is `static`?
  - **Technical answer:** A keyword that declares a global variable with a fixed memory address that lives for the entire program. Unlike `const` (which is inlined as a value at every use site), `static` has a stable address you can take a `&'static T` reference to. `static mut` exists but requires `unsafe` for every access because globals are reachable from multiple threads, opening data races. The idiomatic safe alternative is `static FOO: Mutex<T> = Mutex::new(...)` — the `static` is immutable but the `Mutex` allows safe interior mutability.
  - **Plain-English analogy / example:**
    ```rust
    use std::sync::Mutex;
    static COUNTER: Mutex<u32> = Mutex::new(0);
    fn bump() {
        let mut c = COUNTER.lock().unwrap();
        *c += 1;  // safe; lock auto-releases when c drops
    }
    ```
  - **See also:** `topics/rust/01-fundamentals/variables.md`, `topics/rust/01-fundamentals/const-vs-static.md` (new visual companion)
- **Q:** What's the difference between `static` and `const` and when/why use each?
  - **Technical answer:** `const` is a **value** (compile-time, inlined at each use, no address); `static` is a **variable** (one fixed address, lives the whole program). Default to `const` for named numeric/boolean/string-literal constants — it's what you'd reach for in 95% of cases. Reach for `static` when you need (1) a stable address, e.g. for FFI; (2) shared mutable state behind a `Mutex`/`RwLock`/`OnceLock`/atomic; (3) a large lookup table that shouldn't be inlined; or (4) a `&'static T` reference to share around.
  - **Plain-English analogy / example:**
    ```rust
    const MAX_RETRIES: u32 = 3;          // compile-time value, inlined
    static APP_NAME: &str = "rustlearn"; // one address, &'static &str
    static CALLS: Mutex<u64> = Mutex::new(0); // safe global mutable
    ```
  - **See also:** `topics/rust/01-fundamentals/const-vs-static.md` (decision flowchart + memory layout + race-vs-Mutex sequence diagram)
- **Q:** Show me a real example using `static` — this is hard.
  - **Technical answer:** The textbook case is a global counter that must survive across function calls. A local `let mut` inside the function resets every call (always prints 1). `const` can't be assigned to (it's a value). `static mut` works but needs `unsafe` everywhere. The idiomatic answer is `static COUNTER: Mutex<u32> = Mutex::new(0)` — call `.lock().unwrap()` to get a `MutexGuard`, deref-assign with `*count += 1`, and the lock auto-releases when the guard drops.
  - **Plain-English analogy / example:** see `code/01-fundamentals/scratch-static-counter/src/main.rs` — runs all four cases (local, const, static mut, static+Mutex) side by side. Output shows local always prints 1; the two statics correctly count 1, 2, 3.
  - **See also:** `code/01-fundamentals/scratch-static-counter/`, `topics/rust/01-fundamentals/const-vs-static.md`
- **Q:** Hmm this is very tricky right (re: `_CONST_COUNTER += 1;`)?
  - **Technical answer:** Yes — the trick is that `let mut`, `const`, and `static` *look* like three flavors of "declare a thing called X" but they're three different mechanisms. `let mut` is a runtime variable, `const` is a compile-time value with no runtime storage, `static` is a fixed-address global. Writing `CONST += 1` is effectively `0 += 1` because by runtime the name is gone, only the literal remains — that's why the error says "invalid left-hand side of assignment" rather than "missing `mut`."
  - **Plain-English analogy / example:**
    ```rust
    const X: u32 = 0;
    // X += 1;  // ❌ "invalid left-hand side of assignment"
    // ↑ at runtime this is `0 += 1` — no place to assign into
    ```
  - **See also:** `topics/rust/01-fundamentals/const-vs-static.md`
- **Q:** Please check my answers to #4–#7 and tell me what I don't understand.
  - **Technical answer:** Reviewed all four. #4 was 2/3 right (arithmetic slip on the first snippet, block-scoped shadowing fully understood). #5 confused type *annotations* with type *casts* — Rust has no implicit numeric coercion, so `i32 + i64` needs an explicit `as` cast at the operator. #6 wasn't attempted ("i dont know") — it's the keystone "blocks/`if`/`loop` are expressions" idiom. #7 mashed `match` arm syntax into a `loop`, threw in `Result` for a non-failing function, and welded `static`+`const`+`mut` into invalid syntax — three separate gaps surfaced in one answer.
  - **Plain-English analogy / example:**
    ```rust
    fn sign(n: i32) -> &'static str {
        if n > 0 { "positive" }       // last expression of branch, no `;`
        else if n < 0 { "negative" }
        else { "zero" }                // no `;` after final brace either
    }
    ```
  - **See also:** `topics/rust/01-fundamentals/control-flow.md`, `topics/rust/01-fundamentals/functions.md`, `topics/rust/01-fundamentals/mistakes.md` (new — see below)
- **Q:** Add mistake-tracking to `/journal`.
  - **Technical answer:** Updated `.claude/commands/journal.md` to take an additional step: when run, it now also appends any session mistakes to `topics/rust/<NN-chapter>/mistakes.md` in a fixed format (**What I wrote** / **Why it's wrong** / **The rule** / **Status: 🔴 fresh**). Status emoji ladder is 🔴 → 🟡 → 🟢; you upgrade them yourself during reviews. Each chapter gets its own file.
  - **Plain-English analogy / example:** Like a doctor's chart per body system — Ch 1 mistakes go in `01-fundamentals/mistakes.md`, Ch 2 mistakes in `02-ownership/mistakes.md`, etc. The chart is for *you* to scan before each session.
  - **See also:** `topics/rust/01-fundamentals/mistakes.md`, `.claude/commands/journal.md`
**Question to answer later:**
- After fixing #6 and writing the `if/else`-expression version of `sign`: does the same "block as expression" rule extend to `match` arms (yes, but worth feeling)?
- Is `static FOO: Mutex<T> = Mutex::new(0)` always the right answer for a global counter, or should I reach for `AtomicU32` instead? (Atomics avoid the lock; useful when you only need primitive ops.)
- For `count_digits(0)`: should it return 0 (zero-iteration loop) or 1 ("zero is one digit")? Spec doesn't say — pick a convention and document it in a doc comment.
**Next:** Fix #4's wrong guess (`10 → 12`), apply one `as` cast for #5, write the `if/else`-as-expression version of `sign` for #6, write `count_digits` with `let mut count`, `loop`, and `break count;` for #7. Then `/journal` again with the cleanup notes and any new mistakes that surfaced.

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

### 2026-08-10 - Shadowing and integer-type review
**Working on:** Ch 1 paper exercises #4-#5 - `topics/rust/exercises/ch01-fundamentals.md` (no crate)
**What clicked:** Every repeated `let` creates a new binding through shadowing; braces limit the inner binding's scope. Assignment omits `let` and requires a mutable binding. Rust also requires both operands of arithmetic to have the same numeric type and does not silently widen integers.
**What didn't:** The predictions were correct, but the reason the inner `y` disappears after its block and the difference between shadowing and assignment initially needed clarification.
**Questions asked this session:**
- **Q:** Why are the inner `y` and outer `y` different?
  - **Technical answer:** A scope is the region where a binding is visible. `let y = y + 1` inside the braces reads the outer `y`, then creates a new inner binding that shadows it; when the block ends, that inner binding leaves scope and the unchanged outer `y` becomes visible again.
  - **Plain-English analogy / example:**
    ```rust
    let y = 5;
    { let y = y + 1; println!("{y}"); } // 6
    println!("{y}");                    // 5
    ```
  - **See also:** `topics/rust/01-fundamentals/variables.md`
- **Q:** Which one is shadowing and which one is assignment?
  - **Technical answer:** Repeating `let x = ...` is shadowing: it creates a new binding with the same name. Writing `x = ...` without `let` is assignment: it changes an existing binding, which must have been declared with `mut`.
  - **Plain-English analogy / example:**
    ```rust
    let x = 5;
    let x = x + 1; // shadowing: new x
    let mut y = 5;
    y = y + 1;     // assignment: same y
    ```
  - **See also:** `topics/rust/01-fundamentals/variables.md`
- **Q:** Why can't `i32` and `i64` be added directly, and what is the fix?
  - **Technical answer:** `i32` and `i64` are distinct types, and Rust performs no implicit numeric coercion (automatic conversion) between them. Make the operands the same type explicitly; converting `i32` to `i64` with `i64::from` is lossless.
  - **Plain-English analogy / example:**
    ```rust
    let a = 100_i32;
    let b = 200_i64;
    let c = i64::from(a) + b;
    println!("{c}");
    ```
  - **See also:** `topics/rust/01-fundamentals/data-types.md`
**Question to answer later:** -
**Next:** Ch 1 exercise #6 - rewrite `sign` without `return` by using the value of an `if` expression.

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
