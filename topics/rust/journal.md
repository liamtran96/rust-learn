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

### 2026-08-12 — `&'static str`, owned `String`, and memory
**Working on:** Follow-up to Ch 1 Exercise #6 — `code/01-fundamentals/sign-expression/`
**What clicked:** `&'static str` is a reference to text valid for the rest of the program run; it is not the `static` item keyword and does not mean permanent disk storage. `String` owns runtime text, normally allocates heap memory, moves ownership to the caller when returned, and frees that memory when its owner is dropped.
**What didn't:** The word “static” first sounded like “saved on the computer forever,” and the distinction between reusing a literal and constructing an owned `String` needed several analogies.
**Questions asked this session:**
- **Q:** Why do we use `&'static str`?
  - **Technical answer:** `&str` is a borrowed view of UTF-8 text, and a lifetime tells Rust how long that borrow is valid. `sign` returns string literals rather than borrowing from an input, and literals are embedded in the program, so their references have the `'static` lifetime.
  - **Plain-English analogy / example:**
    ```rust
    let label: &'static str = "negative";
    let result = sign(-2);
    println!("{result}");
    ```
  - **See also:** `topics/rust/02-ownership/lifetimes.md`, `topics/rust/04-collections/strings.md`
- **Q:** Can you explain `&'static str` like I am five?
  - **Technical answer:** The reference `&` points to text owned somewhere else, while `'static` promises that the referenced text remains valid for the program run. The variable holding that reference can still go out of scope earlier; only the referenced literal has the long lifetime.
  - **Plain-English analogy / example:**
    ```text
    "negative" = a book kept in the program's library
    &           = a note pointing to that book
    'static     = the book stays until the library closes
    ```
  - **See also:** `topics/rust/02-ownership/lifetimes.md`
- **Q:** Why do we need the lifetime, and what is it for?
  - **Technical answer:** Rust uses lifetimes to reject dangling references—references that point to data already destroyed. Because `sign` has no borrowed input from which Rust could infer the output lifetime, `&'static str` explicitly states that the output points to program-lifetime data.
  - **Plain-English analogy / example:**
    ```rust
    fn valid() -> &'static str { "hello" }
    // A reference to a local String would be invalid:
    // the String would be dropped when the function ends.
    ```
  - **See also:** `topics/rust/02-ownership/lifetimes.md`
- **Q:** Does returning `String` mean those values are saved on my computer forever?
  - **Technical answer:** No. `String` is owned text, usually backed by heap memory allocated while the process runs; returning it moves ownership to the caller. Rust frees that allocation when the final owner leaves scope, and the operating system reclaims process memory when the program exits.
  - **Plain-English analogy / example:**
    ```rust
    {
        let label = String::from("negative");
        println!("{label}");
    } // label is dropped and its heap allocation is freed
    ```
  - **See also:** `topics/rust/04-collections/strings.md`, `topics/rust/02-ownership/ownership.md`
- **Q:** Can using `String` affect performance?
  - **Technical answer:** Constructing a `String` from a literal normally allocates heap memory, copies the bytes, and later deallocates them. Returning `&'static str` reuses an existing literal and returns only a reference, so the difference can matter in a hot loop but is negligible for one small call.
  - **Plain-English analogy / example:**
    ```rust
    for n in 0..1_000_000 {
        let label = sign(n); // reuse one of the existing literals
        std::hint::black_box(label);
    }
    ```
  - **See also:** `topics/rust/04-collections/strings.md`
- **Q:** What is a real-life example of the performance difference?
  - **Technical answer:** Reusing `&'static str` is like pointing to one of three permanent restaurant signs; creating a `String` is like printing a fresh paper sign for every customer and throwing it away afterward. The extra work is invisible for a few customers but wasteful for millions.
  - **Plain-English analogy / example:**
    ```text
    &'static str × 1,000,000 → point at existing signs
    String × 1,000,000       → print 1,000,000 new signs
    Same message; different amount of work.
    ```
  - **See also:** `topics/rust/04-collections/strings.md`
**Question to answer later:** How does Rust connect a returned `&str` lifetime to an input `&str` when the text is not a literal?
**Next:** Finish the iterator-`map` version of FizzBuzz; revisit named lifetimes during Phase 2 ownership.

### 2026-08-12 — Measuring `&'static str` versus `String`
**Working on:** Performance follow-up — `code/01-fundamentals/sign-expression/examples/performance.rs`, `benches/sign_performance.rs`, and `borrowed-vs-owned.html`
**What clicked:** A fair microbenchmark measures many calls in an optimized build, hides predictable inputs and unused outputs from the optimizer, and reports an average or statistical estimate rather than timing one tiny call. Returning `String` includes allocation, copying, ownership, and eventual deallocation; returning a literal reference reuses existing program data.
**What didn't:** The first handwritten median selected the upper-middle result for an even number of samples; it was corrected to average the two middle durations. The exact speed ratio initially sounded universal, but it is local to the machine, compiler, benchmark design, and current system load.
**Questions asked this session:**
- **Q:** Can we calculate the performance of both return types and see and run the code?
  - **Technical answer:** A separate release-mode example now calls both implementations repeatedly and measures elapsed wall-clock time with `Instant`. It uses the same changing inputs for both cases and reports total duration, nanoseconds per call, and their ratio without altering the finished exercise binary.
  - **Plain-English analogy / example:**
    ```text
    cargo run --release --example performance
    &'static str → point to an existing sign
    String       → make and discard a new paper sign
    ```
  - **See also:** `code/01-fundamentals/sign-expression/examples/performance.rs`
- **Q:** How do you measure it?
  - **Technical answer:** `Instant::now()` records the starting clock point and `start.elapsed()` returns a `Duration`, an amount of elapsed time. The benchmark measures the whole loop and divides its total nanoseconds by the number of calls; it does not time each call separately.
  - **Plain-English analogy / example:**
    ```rust
    let start = Instant::now();
    for _ in 0..10_000_000 { do_work(); }
    let total = start.elapsed();
    ```
  - **See also:** `code/01-fundamentals/sign-expression/examples/performance.rs`
- **Q:** What are `black_box`, `Instant`, and `Duration`?
  - **Technical answer:** `black_box` is an optimizer hint that makes inputs look unpredictable and outputs potentially useful, discouraging removal of the measured work. `Instant` is a monotonic stopwatch point, while `Duration` stores the amount of time between two points.
  - **Plain-English analogy / example:**
    ```rust
    let start = Instant::now();       // press stopwatch start
    black_box(sign(black_box(-2)));   // make Rust really do the work
    let elapsed: Duration = start.elapsed();
    ```
  - **See also:** `code/01-fundamentals/sign-expression/examples/performance.rs`
- **Q:** How do `median` and `nanos_per_call` work, and are you sure?
  - **Technical answer:** The median sorts repeated durations and selects the center, reducing the effect of unusually slow noisy samples. For an even count, the correct implementation averages the two center values; `nanos_per_call` converts the total to nanoseconds and divides by the iteration count to calculate average throughput.
  - **Plain-English analogy / example:**
    ```text
    sorted: 9ms, 10ms, 11ms, 12ms
    median: (10ms + 11ms) / 2 = 10.5ms
    ns/call: total nanoseconds / number of calls
    ```
  - **See also:** `code/01-fundamentals/sign-expression/examples/performance.rs`
- **Q:** Is there another way to measure this?
  - **Technical answer:** Criterion is a statistics-driven microbenchmarking library that automatically warms up, chooses iteration counts, collects samples, estimates a confidence interval, and identifies outliers. Allocation counters and profilers answer different questions, such as how many heap allocations occurred or which CPU instructions were expensive.
  - **Plain-English analogy / example:**
    ```text
    cargo bench --bench sign_performance
    manual Instant → homemade stopwatch
    Criterion      → repeated lab measurement
    ```
  - **See also:** `code/01-fundamentals/sign-expression/benches/sign_performance.rs`
- **Q:** Is the Criterion comparison definitely measuring the right thing?
  - **Technical answer:** Yes, for the question “what does obtaining and then discarding each result cost?” Criterion's `iter` loop includes destruction of the returned value. That fairly includes allocation, copying, and freeing for `String`, while dropping a borrowed reference has essentially no cleanup work; the measured ratio remains a local estimate rather than a universal constant.
  - **Plain-English analogy / example:**
    ```text
    borrowed result: choose → point → discard pointer
    owned result:    allocate → copy → own → free
    both paths include their complete cleanup
    ```
  - **See also:** `code/01-fundamentals/sign-expression/benches/sign_performance.rs`
- **Q:** Can you show both cases visually so a five-year-old can understand?
  - **Technical answer:** The interactive comparison shows borrowing as pointing to one reusable program sign and ownership as producing a new paper sign for each call. It can switch between one and one million calls and step through preparation, return, and cleanup.
  - **Plain-English analogy / example:**
    ```text
    Borrow: “Look at the sign already on the wall.”
    Own:    “Print a new sign, give it away, then recycle it.”
    ```
  - **See also:** `code/01-fundamentals/sign-expression/borrowed-vs-owned.html`
- **Q:** What is the heap, how does it work, and can it be added visually?
  - **Technical answer:** The heap is a region of runtime memory used for dynamically sized or long-lived allocations. A `String` owns a small handle containing a pointer, length, and capacity; its pointer leads to bytes on the heap, and when the `String` is dropped Rust returns that space to the allocator for reuse.
  - **Plain-English analogy / example:**
    ```text
    allocate → reserve an empty shelf
    use      → String owns the shelf address
    drop     → empty the shelf so it can be reused
    ```
  - **See also:** `code/01-fundamentals/sign-expression/borrowed-vs-owned.html`, `topics/rust/02-ownership/ownership.md`
**Question to answer later:** Where does the `String` handle itself live, and how are its pointer, length, and capacity represented?
**Next:** Finish the iterator-`map` version of FizzBuzz; revisit stack-versus-heap and ownership in Phase 2.

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

### 2026-08-17 — Ownership moves and shared borrowing
**Working on:** Ch 2 predict-and-fix snippets A–B — `topics/rust/exercises/ch02-ownership.md` (paper exercises; no crate)
**What clicked:** Assigning a `String` moves ownership, while `&` creates a non-owning shared reference. A shared borrow lasts through its final use; after that, the owner may be mutably borrowed again. References are pointer-like but compiler-checked for validity and lifetime.
**What didn't:** The ownership transfer in snippet A initially had no explanation or fix. In snippet B, the first prediction was that the code compiled and that `&v[0]` referred to the later value `4`; the correction connected indexing, `Vec` reallocation, and the rule that shared and mutable borrows cannot overlap.
**Questions asked this session:**
- **Q:** What does “borrow when you only need to read; move when ownership should transfer” mean?
  - **Technical answer:** Borrowing gives temporary access through a reference while the original variable remains the owner. Moving transfers ownership—and therefore responsibility for eventually dropping the value—to a new variable.
  - **Plain-English analogy / example:**
    ```rust
    let s = String::from("hi");
    let borrowed = &s; // s still owns the String
    let moved = s;     // moved now owns it
    ```
  - **See also:** `topics/rust/02-ownership/ownership.md`
- **Q:** What does the symbol `&` mean?
  - **Technical answer:** `&` creates a shared reference, which is a non-owning, read-only borrow of an existing value. The referenced value must remain valid for as long as that reference is used.
  - **Plain-English analogy / example:**
    ```rust
    let text = String::from("hello");
    let reference: &String = &text;
    println!("{reference}");
    ```
  - **See also:** `topics/rust/02-ownership/borrowing.md`
- **Q:** Is a reference like a pointer?
  - **Technical answer:** Yes: a reference identifies an existing value by address, but Rust checks that it is non-null, valid, and does not outlive its target. A shared reference `&T` also cannot be used to mutate the referenced value.
  - **Plain-English analogy / example:**
    ```text
    owner: String ──owns──> heap text
    &String       ──points─> same String
    compiler      ──checks─> pointer stays valid
    ```
  - **See also:** `topics/rust/02-ownership/borrowing.md`
- **Q:** What comes next after learning references and completing snippets A–B?
  - **Technical answer:** Continue with string slices and snippet D during Week 2. Snippet C uses explicit lifetime annotations and stays parked for the Week 3 lifetime lesson.
  - **Plain-English analogy / example:**
    ```text
    completed: move ownership → shared borrowing
    next:      string slices → snippet D
    later:     named lifetimes → snippet C
    ```
  - **See also:** `topics/rust/02-ownership/slices.md`, `topics/rust/02-ownership/lifetimes.md`
**Question to answer later:** Why is `&str` usually preferred over `&String` in function parameters?
**Next:** Read `topics/rust/02-ownership/slices.md`, then predict and improve Ch 2 snippet D.

### 2026-08-21 — Retrieval homework: expressions, bindings, and safe CLI input
**Working on:** `topics/rust/homework/2026-08-18-retrieval-mixed.md` with saved code in `code/01-fundamentals/homework-practice/`
**What clicked:** Completed all six retrieval questions after guided retries. Tail expressions pass values out of blocks, `()` means no meaningful value, shadowing creates a new binding while assignment changes an existing mutable binding, `break value` gives a `loop` its value, string literals have program-long storage, and safe CLI input validates length before borrowing and parsing.
**What didn't:** Semicolon placement and `println!` returning `()` still caused repeated type-flow mistakes. The first explanations tied shadowing to memory addresses, treated string literals as if they died with the function scope, and indexed `args[1]` before validating the vector length; assembling `match` syntax also required several focused retries.
**Questions asked this session:**
- **Q:** What is the difference between a statement and an expression, what does a trailing semicolon do, and what is `()`?
  - **Technical answer:** An expression evaluates to a value. A trailing semicolon can turn a value-producing expression into a statement that discards that value, so a block with no tail expression evaluates to unit, written `()`.
  - **Plain-English analogy / example:**
    ```rust
    let number = { 1 + 1 };  // i32: 2
    let unit = { 1 + 1; };   // (): value discarded
    println!("{number} {unit:?}");
    ```
  - **See also:** `topics/rust/01-fundamentals/functions.md`, `topics/rust/01-fundamentals/control-flow.md`
- **Q:** What does “binding” mean, and how does shadowing differ from assignment?
  - **Technical answer:** A binding connects a name to a value. Repeating `let` creates a new binding that hides the old one; assignment omits `let`, changes an existing binding, and requires that binding to be mutable. Rust does not promise separate memory addresses for shadowed bindings.
  - **Plain-English analogy / example:**
    ```rust
    let x = 5;
    let x = "five"; // new binding; type may change
    let mut y = 5;
    y = 6;          // same binding; value changes
    ```
  - **See also:** `topics/rust/01-fundamentals/variables.md`
- **Q:** Why does a type annotation not convert `i32` into `i64` for arithmetic?
  - **Technical answer:** An annotation constrains or checks a type; it does not perform a numeric conversion. `i32` and `i64` are distinct types, so both operands must be made the same type explicitly before addition.
  - **Plain-English analogy / example:**
    ```rust
    let a = 100_i32;
    let b = 200_i64;
    let total = i64::from(a) + b;
    println!("{total}");
    ```
  - **See also:** `topics/rust/01-fundamentals/data-types.md`
- **Q:** Why did `direction` return `()` instead of `String`, and did I describe semicolon suppression incorrectly?
  - **Technical answer:** Each `.to_string();` was an expression statement whose `String` result was discarded, making each branch produce `()`. Removing the semicolons lets both branches and the complete `if` expression produce `String`; the correction was right, but “cannot suppress” stated the rule backwards.
  - **Plain-English analogy / example:**
    ```rust
    let direction = if true {
        "up".to_string()
    } else {
        "down".to_string()
    };
    ```
  - **See also:** `topics/rust/01-fundamentals/control-flow.md`
- **Q:** Should I add `let total =+ 1`, and where must the loop counter be declared?
  - **Technical answer:** `=+` is not an increment operator, and the `total` being declared does not exist while Rust evaluates its right-hand side. A separate mutable counter must exist before the loop; `break counter` then becomes the loop expression’s final value.
  - **Plain-English analogy / example:**
    ```rust
    let mut counter = 0;
    let total = loop {
        counter += 1;
        break counter;
    };
    ```
  - **See also:** `topics/rust/01-fundamentals/variables.md`, `topics/rust/01-fundamentals/control-flow.md`
- **Q:** How can `1_000` and `u32::MAX` be passed when I thought the argument was `i32`?
  - **Technical answer:** The function parameter was `u32`, not `i32`. Underscores only improve literal readability, and an unsuffixed literal such as `1_000` is inferred as `u32` from the function parameter; `u32::MAX` is already a `u32` and equals `4_294_967_295`.
  - **Plain-English analogy / example:**
    ```rust
    let inferred: u32 = 1_000;
    let explicit = 1_000_u32;
    assert_eq!(inferred, explicit);
    assert_eq!(u32::MAX, 4_294_967_295);
    ```
  - **See also:** `topics/rust/01-fundamentals/data-types.md`
- **Q:** Why can `shipping_band` return `&'static str`, and do its literals disappear when the function leaves scope?
  - **Technical answer:** A string literal is embedded in the compiled program and has the type `&'static str`; `'static` means the referenced bytes remain valid for the program run. A local reference variable can leave scope, but the literal data does not disappear then. Creating a `String` would instead create owned runtime text, normally with a heap allocation.
  - **Plain-English analogy / example:**
    ```rust
    fn label() -> &'static str {
        "standard" // embedded literal; valid for the program run
    }
    println!("{}", label());
    ```
  - **See also:** `topics/rust/01-fundamentals/functions.md`, `topics/rust/04-collections/strings.md`
- **Q:** How do we know the literals are stored in the binary, what is a compiled binary, and what does it look like?
  - **Technical answer:** `rustc` translates source into a machine-readable executable containing CPU instructions and program data. Searching the compiled `.exe` found `heavy`, `standard`, and `light` at concrete byte offsets; a hex dump showed the Windows `MZ` header and the literals’ UTF-8 byte values.
  - **Plain-English analogy / example:**
    ```text
    inspect-static.rs --rustc--> inspect-static.exe
    light -> 6C 69 67 68 74
    4D 5A -> MZ, the Windows executable signature
    ```
  - **See also:** `topics/rust/01-fundamentals/toolchain.md`
- **Q:** Why could I not open `findstr.exe /M /C:"light" program.exe`, and what was the `.pdb` file?
  - **Technical answer:** `findstr.exe` is a terminal command, and `program.exe` was only a placeholder that had to be replaced by a real compiled filename. A `.pdb` is a Windows debug-symbol file produced alongside some builds; it maps machine code back to source information and can be regenerated.
  - **Plain-English analogy / example:**
    ```text
    rustc inspect-static.rs -o inspect-static.exe
    findstr.exe /M /C:"light" inspect-static.exe
    inspect-static.pdb -> debugger map, not the program itself
    ```
  - **See also:** `topics/rust/01-fundamentals/toolchain.md`
- **Q:** How do I create a folder, save the homework code, and move the code into it?
  - **Technical answer:** A plain directory was created under the Chapter 1 code area, and each code answer was saved as its own `.rs` file. Git does not track an empty directory, so a placeholder made it visible until real files existed; no existing Cargo crate source was moved or broken.
  - **Plain-English analogy / example:**
    ```text
    code/01-fundamentals/homework-practice/
    |-- question-04.rs
    |-- question-05.rs
    `-- question-06.rs
    ```
  - **See also:** `topics/rust/cheatsheets/cargo-commands.md`
- **Q:** What should I do next? I mean my homework.
  - **Technical answer:** Repository progress still named temperature-converter cleanup as the roadmap action, but the immediate conversational task was the unfinished retrieval set. Question 5 (`shipping_band`) came next, followed by Question 6’s safe CLI flow; this did not change the official 7/7 Chapter 1 count.
  - **Plain-English analogy / example:**
    ```text
    Immediate task: retrieval homework Q5 -> Q6
    Official progress: Chapter 1 remains 7/7
    Roadmap next: temperature-converter cleanup
    ```
  - **See also:** `topics/rust/homework/2026-08-18-retrieval-mixed.md`, `topics/rust/progress.md`
- **Q:** I do not know how to do Exercise 6, and I do not remember the syntax—how should safe CLI parsing flow?
  - **Technical answer:** Collect arguments, validate that index 1 exists, borrow it, then call `parse::<u32>()`. Parsing returns `Result`; `Ok(value)` yields the number, while `Err(_)` prints an error and returns early. The complete flow was rebuilt incrementally instead of copied from the temperature converter.
  - **Plain-English analogy / example:**
    ```rust
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 { return; }
    let input = &args[1];
    let parsed = input.parse::<u32>();
    ```
  - **See also:** `topics/rust/01-fundamentals/data-types.md`
- **Q:** Why use `args.len()` before `args[1]`, and what does `&args[1]` do?
  - **Technical answer:** A vector with length 1 has only index 0, so indexing at 1 first can panic. `args.len() < 2` checks the collection before access, and `&args[1]` borrows the `String`—temporarily refers to it—without moving it out of the vector.
  - **Plain-English analogy / example:**
    ```rust
    if args.len() < 2 {
        return; // index 1 does not exist
    }
    let input = &args[1]; // safe borrow after guard
    ```
  - **See also:** `topics/rust/01-fundamentals/data-types.md`
- **Q:** What do `Ok` and `Err` mean here, and why can the `Ok` arm not just call `println!`?
  - **Technical answer:** `Ok(value)` and `Err(error)` are the two variants of `Result`; matching them either extracts the parsed `u32` or handles failure. Because `let quantity: u32 = match ...` requires the match expression to produce `u32`, the `Ok` arm must yield the number; `println!` returns `()`, so printing belongs after the match has created `quantity`.
  - **Plain-English analogy / example:**
    ```rust
    let quantity: u32 = match parsed {
        Ok(value) => value,
        Err(_) => return,
    };
    println!("{quantity}");
    ```
  - **See also:** `topics/rust/01-fundamentals/control-flow.md`, `topics/rust/01-fundamentals/functions.md`
**Question to answer later:** —
**Next:** Resume Phase 2 by reading `topics/rust/02-ownership/slices.md`, then predict and improve Ch 2 snippet D.

### 2026-08-24 — String slices and borrowed return values
**Working on:** Ch 2 predict-and-fix snippet D — `topics/rust/exercises/ch02-ownership.md` (paper exercise; no crate)
**What clicked:** `first_word` compiles and returns a borrowed `&str` rather than owning a new string. Changing its parameter from `&String` to `&str` lets it accept string literals directly while still accepting `&String` through deref coercion. A returned slice remains valid only while the input text it points into is alive.
**What didn't:** Initially predicted that `word` could be printed after the inner block dropped its owning `String`. The lifetime relationship became clear after moving the print into the scope where both the owner and its borrowed slice were still valid.
**Questions asked this session:**
- **Q:** “And then?”
  - **Technical answer:** After improving the parameter to `&str`, the next check was whether the returned slice could outlive its source. A lifetime is the span during which a reference is valid; Rust rejects code that stores a slice from a local `String` and uses it after that owner has been dropped.
  - **Plain-English analogy / example:**
    ```rust
    let text = String::from("hello world");
    let word = first_word(&text);
    println!("{word}"); // owner and borrow are both alive
    ```
  - **See also:** `topics/rust/02-ownership/slices.md`, `topics/rust/02-ownership/lifetimes.md`
- **Q:** “`println!("{word}");` — this one, right?”
  - **Technical answer:** Yes. Moving the print into the inner block uses `word` before `text` is dropped, so the reference still points to valid string data. Moving the reference itself does not extend the lifetime of the value it borrows.
  - **Plain-English analogy / example:**
    ```rust
    {
        let text = String::from("hello world");
        let word = first_word(&text);
        println!("{word}");
    }
    ```
  - **See also:** `topics/rust/02-ownership/borrowing.md`, `topics/rust/02-ownership/slices.md`
**Question to answer later:** How does Rust infer that the returned `&str` is tied to the input `&str` without a written lifetime annotation?
**Next:** Implement `strip_margin` from `topics/rust/exercises/ch02-ownership.md`.

### 2026-08-25 - Ownership moves and shared borrowing
**Working on:** Ownership drill d01 - `code/02-ownership/drills-ownership/tests/d01_move.rs`
**What clicked:** Assigning an owned `String` with `let t = s` moves ownership from `s` to `t`, so `s` is no longer usable. Creating `t` with `&s` instead makes a shared reference: `t` can read the same `String` without taking ownership from `s`.
**What didn't:** The first explanation reversed the direction of the move (`t` to `s`) and needed correction before the drill counted as complete. The first Cargo command was also run from the parent directory instead of the crate directory.
**Questions asked this session:**
- **Q:** Why did Cargo report that it could not find `Cargo.toml`?
  - **Technical answer:** Cargo searches the current directory and its parents for a package manifest named `Cargo.toml`. The command ran from `code/02-ownership/`, but this independent crate's manifest is one level deeper in `drills-ownership/`.
  - **Plain-English analogy / example:**
    ```text
    code/02-ownership/                    # no Cargo.toml here
    `-- drills-ownership/
        |-- Cargo.toml                    # run Cargo here
        `-- tests/d01_move.rs
    ```
  - **See also:** `topics/rust/cheatsheets/cargo-commands.md`
- **Q:** What does `let t = s` do to ownership, and why does `let t = &s` leave `s` usable?
  - **Technical answer:** Because `String` does not implement `Copy`, assigning `s` by value moves ownership from `s` to `t` and invalidates `s`. The `&` operator creates a shared reference, so `t` borrows the value without becoming its owner and both names can read it while the borrow is valid.
  - **Plain-English analogy / example:**
    ```rust
    let s = String::from("hi");
    let t = &s;
    assert_eq!(s, "hi");
    assert_eq!(t, "hi");
    ```
  - **See also:** `topics/rust/02-ownership/ownership.md`, `topics/rust/02-ownership/borrowing.md`
**Question to answer later:** Which common Rust types implement `Copy`, and why does `String` not implement it?
**Next:** Complete ownership drill d02 in the same crate: fill `PREDICT:`, run `cargo test --test d02_copy_vs_move`, fix minimally, then fill `WHY:`.

### 2026-08-25 - Copy, move, borrow, and clone
**Working on:** Ownership drill d02 - `code/02-ownership/drills-ownership/tests/d02_copy_vs_move.rs`
**What clicked:** Assignment copies an `i32` implicitly because it implements `Copy`, leaving the original usable. A `String` moves on assignment, `&String` borrows without taking ownership, and `String::clone` explicitly allocates an independently owned copy; the drill only needed shared reading, so borrowing was the appropriate choice.
**What didn't:** Initially treated `String` as a `Copy` type, described `Copy` as merely having ownership, and repeatedly used `.clone()` to make the test green. After switching to a borrow, the assertion still needed operands at the same reference level because `String` and `&String` were not directly comparable in that expression.
**Questions asked this session:**
- **Q:** Why did direct `assert_eq!(a, b)` fail with a borrowed string while `format!("{s} {t}")` passed?
  - **Technical answer:** With `a: String` and `b: &String`, the direct equality expression asked for a comparison between different operand types that was not implemented. `format!` does not compare the two values; it formats each through `Display` into a new `String`, which can then be compared with the expected text.
  - **Plain-English analogy / example:**
    ```rust
    let s = String::from("hi");
    let t = &s;
    let rendered = format!("{s} {t}");
    assert_eq!(rendered, "hi hi");
    ```
  - **See also:** `topics/rust/02-ownership/borrowing.md`
- **Q:** When should `&` be used instead of `.clone()`?
  - **Technical answer:** Use `&` or `&mut` for temporary access when another variable should remain the owner. Use `.clone()` only when the program genuinely needs a second independently owned value that can outlive or be changed separately from the original; cloning a `String` copies its heap data.
  - **Plain-English analogy / example:**
    ```rust
    let owner = String::from("notes");
    let reader = &owner;          // temporary access, no text copied
    let backup = owner.clone();   // independent owned text
    assert_eq!(reader, &backup);
    ```
  - **See also:** `topics/rust/02-ownership/borrowing.md`, `topics/rust/02-ownership/ownership.md`
- **Q:** What is the difference between borrowing, copying, and moving?
  - **Technical answer:** A move transfers ownership and invalidates the source; a `Copy` assignment implicitly duplicates a small copyable value and leaves the source usable. A borrow creates a reference without transferring ownership, and its validity is limited by the owner's lifetime and Rust's aliasing rules.
  - **Plain-English analogy / example:**
    ```text
    Move   -> give away the book
    Copy   -> duplicate a small page; both copies remain
    Borrow -> lend the book; the original person stays owner
    ```
  - **See also:** `topics/rust/02-ownership/ownership.md`, `topics/rust/02-ownership/borrowing.md`
**Question to answer later:** How do mutable borrows change the rules compared with shared borrows?
**Next:** Complete ownership drill d03: fill `PREDICT:`, run `cargo test --test d03_borrow_then_mutate`, fix minimally, then fill `WHY:`.

### 2026-08-25 - Shared borrow across `Vec` mutation
**Working on:** Ownership drill d03 - `code/02-ownership/drills-ownership/tests/d03_borrow_then_mutate.rs`
**What clicked:** A reference to `v[0]` points into the vector's heap buffer. `Vec::push` requires mutable access and may reallocate that buffer, so Rust rejects a shared reference that remains live across the push; performing the mutation before creating the reference removes the overlap.
**What didn't:** The first `WHY:` only said mutation might change the original and did not connect `push`, capacity, reallocation, and the risk of invalidating the element reference.
**Questions asked this session:** -
**Question to answer later:** How does non-lexical lifetime analysis determine the exact point where a borrow ends?
**Next:** Complete ownership drill d04: fill `PREDICT:`, run `cargo test --test d04_two_mut_borrows`, fix minimally, then fill `WHY:`.

### 2026-08-25 - Sequential mutable borrows and last use
**Working on:** Ownership drill d04 - `code/02-ownership/drills-ownership/tests/d04_two_mut_borrows.rs`
**What clicked:** Only one mutable reference to `score` may be live at a time, but the borrows can occur sequentially. Rust's non-lexical lifetime analysis ends `a`'s borrow after `*a += 1`, its final use, allowing `b` to borrow `score` before the surrounding function scope ends.
**What didn't:** Initially said a borrow ends when it returns a value, then assumed it lasts until the reference variable goes out of scope. The key distinction is that lexical scope controls where a name may appear, while the borrow can become inactive earlier after its last use.
**Questions asked this session:**
- **Q:** Does a borrow end when it goes out of scope?
  - **Technical answer:** Leaving scope always ends a borrow, but modern Rust can end it earlier after the reference's last use. This is called non-lexical lifetimes: the compiler infers the portion of the scope where the reference is actually needed.
  - **Plain-English analogy / example:**
    ```rust
    let first = &mut score;
    *first += 1;              // first's last use
    let second = &mut score;  // allowed before function scope ends
    *second += 1;
    ```
  - **See also:** `topics/rust/02-ownership/borrowing.md`, `topics/rust/02-ownership/lifetimes.md`
**Question to answer later:** When is an explicit inner block preferable to relying on the compiler to infer a borrow's last use?
**Next:** Complete ownership drill d05: fill `PREDICT:`, run `cargo test --test d05_for_consumes`, fix minimally, then fill `WHY:`.

### 2026-08-26 - Borrowed iteration over a vector
**Working on:** Ownership drill d05 - `code/02-ownership/drills-ownership/tests/d05_for_consumes.rs`
**What clicked:** `for x in v` consumes the owned `Vec<i32>` and gives the loop `x: i32`; `for x in &v` borrows the vector and gives the loop `x: &i32`. Borrowing provides temporary access without moving ownership or cloning the vector, so `v` remains usable after the loop.
**What didn't:** Initially described `x` as taking ownership of the whole vector and focused on whether types were `Copy`. The `WHY:` needed several revisions before it named the actual type change from `i32` to `&i32` and separated borrowing from copying.
**Questions asked this session:**
- **Q:** Which type is `Copy`, and which is not?
  - **Technical answer:** `Vec<i32>` is not `Copy`, while `i32` and shared references such as `&i32` are `Copy`. Nevertheless, `for x in v` consumes `v` because the owned-vector iterator takes the `Vec` by value; the fact that its elements are `Copy` does not preserve the vector binding.
  - **Plain-English analogy / example:**
    ```rust
    let n: i32 = 3;
    let copied = n;          // i32 is Copy; n remains usable
    let values = vec![n];    // Vec<i32> is not Copy
    let borrowed = &values;  // borrow instead of moving values
    ```
  - **See also:** `topics/rust/02-ownership/ownership.md`, `topics/rust/02-ownership/borrowing.md`
- **Q:** What does "iterating through `&v` borrows the vector instead of consuming it" mean?
  - **Technical answer:** Consuming means transferring ownership of `v` into its iterator, after which the original `v` binding cannot be used. Passing `&v` gives the iterator only a shared reference, so `v` remains the owner and is usable after that temporary borrow ends.
  - **Plain-English analogy / example:**
    ```rust
    let values = vec![1, 2, 3];
    for value in &values {
        println!("{value}");
    }
    println!("{}", values.len()); // still owned here
    ```
  - **See also:** `topics/rust/02-ownership/ownership.md`, `topics/rust/02-ownership/borrowing.md`
- **Q:** What is the difference between `i32` and `&i32`?
  - **Technical answer:** An `i32` is an integer value; an `&i32` is a shared reference that temporarily points to an integer owned elsewhere. Dereferencing with `*` accesses the referred-to value, while creating the reference does not copy or transfer ownership of that value.
  - **Plain-English analogy / example:**
    ```rust
    let number: i32 = 42;
    let reference: &i32 = &number;
    assert_eq!(number, 42);
    assert_eq!(*reference, 42);
    ```
  - **See also:** `topics/rust/02-ownership/borrowing.md`
- **Q:** Why do references exist; what root problem led to using pointer-like `&` values?
  - **Technical answer:** Ownership prevents multiple values from independently managing the same resource, but always moving ownership would make temporary access awkward and cloning could be expensive. A reference is a compiler-checked, non-owning handle that permits temporary access while Rust verifies that the owner stays alive and the access obeys borrowing rules.
  - **Plain-English analogy / example:**
    ```rust
    fn length(values: &[i32]) -> usize {
        values.len() // inspect without owning or cloning
    }
    let values = vec![1, 2, 3];
    assert_eq!(length(&values), 3);
    ```
  - **See also:** `topics/rust/02-ownership/borrowing.md`, `topics/rust/02-ownership/slices.md`
**Question to answer later:** How does `.iter().copied()` borrow the vector while yielding copied `i32` values?
**Next:** Complete ownership drill d06: fill `PREDICT:`, run `cargo test --test d06_str_params`, fix minimally, then fill `WHY:`.

### 2026-08-26 - `&str` parameters and deref coercion
**Working on:** Ownership drill d06 - `code/02-ownership/drills-ownership/tests/d06_str_params.rs`
**What clicked:** A function parameter of `&str` accepts a string literal directly and also accepts `&String` through deref coercion. `String` implements `Deref<Target = str>`, allowing Rust to adapt `&String` to `&str` at a function-call boundary without cloning the string.
**What didn't:** The prediction again described `String` as `Copy`, although it is not. The first explanation also said the owned `String` changes type; more precisely, the `&String` argument is coerced to `&str` while the original `String` remains unchanged.
**Questions asked this session:** -
**Question to answer later:** In which contexts does deref coercion happen automatically, and when must a slice be written explicitly?
**Next:** Complete ownership drill d07: fill `PREDICT:`, run `cargo test --test d07_dangling`, fix minimally, then fill `WHY:`.

### 2026-08-26 - Returning ownership instead of a dangling reference
**Working on:** Ownership drill d07 - `code/02-ownership/drills-ownership/tests/d07_dangling.rs`
**What clicked:** A function cannot return a reference to a local `String` because that local owner is dropped when the function ends. Returning the owned `String` moves ownership to the caller, so the heap allocation remains valid until the caller's returned value is dropped.
**What didn't:** The return-type syntax and the reason for choosing an owned value over `&String` were initially unclear. The final `WHY:` located the text on the heap but did not yet identify the caller as the new owner of the returned `String`.
**Questions asked this session:**
- **Q:** How can I explain d07?
  - **Technical answer:** The original function returned `&String`, a borrowed reference, but the referenced owner `s` was local to the function. When the function ended, `s` would be dropped, so Rust rejected the reference because it would dangle—point to data that was no longer valid.
  - **Plain-English analogy / example:**
    ```text
    function creates a house
    function returns only its address
    function demolishes the house on exit
    caller receives an address to nothing
    ```
  - **See also:** `topics/rust/02-ownership/borrowing.md`, `topics/rust/02-ownership/lifetimes.md`
- **Q:** What does the return syntax mean, and why do we need it?
  - **Technical answer:** In `fn greeting() -> &String`, `->` introduces the return type and `&String` promises a borrowed string owned elsewhere. References are useful when returning a view into caller-owned input, but this function creates new data and has no longer-lived input owner from which to borrow.
  - **Plain-English analogy / example:**
    ```rust
    fn identity(text: &str) -> &str {
        text // returned view borrows from caller-owned input
    }
    let owner = String::from("hello");
    assert_eq!(identity(&owner), "hello");
    ```
  - **See also:** `topics/rust/02-ownership/ownership.md`, `topics/rust/02-ownership/borrowing.md`
- **Q:** What should I use to fix this?
  - **Technical answer:** Transfer ownership of the newly created value instead of returning a reference to the local binding. Moving a `String` out of the function prevents it from being dropped there; the caller becomes responsible for the value and its eventual cleanup.
  - **Plain-English analogy / example:**
    ```rust
    fn make_value() -> String {
        let value = String::from("owned");
        value // ownership moves to the caller
    }
    let value = make_value();
    ```
  - **See also:** `topics/rust/02-ownership/ownership.md`
**Question to answer later:** How can a function safely return a reference when that reference is tied to one of its input parameters?
**Next:** Complete ownership drill d08: fill `PREDICT:`, run `cargo test --test d08_lifetime_elision`, fix minimally, then fill `WHY:`.
