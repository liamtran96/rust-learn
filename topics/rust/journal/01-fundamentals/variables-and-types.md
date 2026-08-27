---
title: Fundamentals Journal - Variables And Types
tags: [rust, journal, fundamentals]
---

# Variables And Types

> Fundamentals topic journal. Return to [[../../journal|Journal index]].

## Entries

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

