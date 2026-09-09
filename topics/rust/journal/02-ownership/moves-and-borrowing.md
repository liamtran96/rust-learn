---
title: Ownership Journal - Moves And Borrowing
tags: [rust, journal, ownership]
---

# Moves And Borrowing

> Ownership topic journal. Return to [[../../journal|Journal index]].

## Entries

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

### 2026-08-27 - Mutating through a slice reference
**Working on:** Ownership drill d10 - `code/02-ownership/drills-ownership/tests/d10_mut_through_ref.rs`
**What clicked:** `&mut Vec<i32>` can coerce to `&mut [i32]`, allowing a function to accept a general mutable slice while callers still pass vectors. `iter_mut` yields `&mut i32` values, and dereferencing each one permits in-place mutation with `*=`.
**What didn't:** The first attempts calculated `*value * 2` but discarded the result, first explicitly with `let _ =` and then as an unused expression. The empty-vector test passed because its loop had no iterations, so only the non-empty assertion demonstrated that mutation actually happened.
**Questions asked this session:**
- **Q:** How should d10 mutate every element, and why can `&mut Vec<i32>` satisfy a `&mut [i32]` parameter?
  - **Technical answer:** Iterating with `iter_mut` yields exclusive references to individual elements; dereferencing and compound-assigning changes the original storage. `Vec<T>` implements mutable dereferencing to `[T]`, so Rust can coerce a mutable vector reference into the mutable slice view required by the function.
  - **Plain-English analogy / example:**
    ```rust
    let mut values = [1, 2];
    for value in values.iter_mut() {
        *value *= 2;
    }
    assert_eq!(values, [2, 4]);
    ```
  - **See also:** `topics/rust/02-ownership/borrowing.md`, `topics/rust/02-ownership/slices.md`
**Question to answer later:** When should a function require `&mut Vec<T>` rather than the more general `&mut [T]`?
**Next:** Complete ownership drill d11: fill `PREDICT:`, run `cargo test --test d11_scoped_return`, fix minimally, then fill `WHY:`.

### 2026-09-08 - Borrowing a String at a function call
**Working on:** Week 3 splitting scaffold and warm-up, `code/02-ownership/split-text/`.
**What clicked:** Liam entered the supplied borrowing example and reported done; verification shows the original message prints after the call. This demonstrates the behavior, but independent explanation is still pending.
**What didn't:** Initially said an &str parameter takes ownership and prevents caller reuse; the abstract follow-up about who owns the text remained unclear. Concrete code was requested and should precede further abstract questions.
**Questions asked this session:**
- **Q:** "ok what should i do next?"
  - **Technical answer:** Start with the recorded borrowing-versus-moving recall before implementing splitting. Then use the brief's decoded signature to write a compiling function stub, a temporary body that establishes the interface before the algorithm.
  - **Plain-English analogy / example:** First check whether you are lending or giving away a book; then start building the tool that reads its pages.
  - **See also:** `WORKFLOW.md`, `code/02-ownership/split-text/BRIEF.md`
- **Q:** "i still dont understand the question"
  - **Technical answer:** Ownership means responsibility for a value, including dropping it when appropriate. A shared reference allows a function to read text owned elsewhere; passing &message does not move the original String into the function.
  - **Plain-English analogy / example:** Lending someone a book to read leaves it yours; giving them the book transfers ownership.
  - **See also:** `topics/rust/02-ownership/ownership.md`, `topics/rust/02-ownership/borrowing.md`
- **Q:** "you should give me the code for better understand"
  - **Technical answer:** The supplied show_text function takes &str and receives &message, so message can be printed after the call. The contrasting take_text function takes String by value; passing message moves it, making a later use invalid. Only the borrowing version was saved and run in this session.
  - **Plain-English analogy / example:**
    ```rust
    fn show_text(text: &str) { println!("{text}"); }
    fn main() {
        let message = String::from("red,blue");
        show_text(&message);
        println!("{message}");
    }
    ```
  - **See also:** `topics/rust/02-ownership/borrowing.md`, `topics/rust/02-ownership/slices.md`
**Verification:** Formatted the example; fmt/check/test (zero tests)/strict Clippy passed. Runtime printed `Inside function:  red,blue` and `After function: red,blue`. Rules verified against [Rust Book: References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html).
**Question to answer later:** Explain from the concrete example why the final println remains valid.
**Next:** Complete that recall, then write a compiling split_text stub in the same crate. Splitting, deduplication, and the Week 3 milestone remain unfinished.

### 2026-09-09 - Generic in-place vector deduplication
**Working on:** Ch 2 exercise 5, hand-written consecutive deduplication - `code/02-ownership/dedup-vec/`
**What clicked:** Liam implemented `dedup_in_place<T: PartialEq>` with an exclusive `&mut Vec<T>` borrow, compared each element with its previous neighbor, and preserved non-adjacent duplicates. He correctly explained at closeout that removal shifts a new element into the current index, so advancing immediately would skip a needed comparison.
**What didn't:** UTF-8 character ordinals were initially treated as possible string-slice positions. The cursor was first expected to advance after removal, the loop initially kept hard-coded indices, and the first test compared the function's unit return value rather than the vector it mutated. Test inputs also briefly used nested arrays where flat vectors were intended.
**Questions asked this session:**
- **Q:** "Why don't `char_indices()` return the character counts?"
  - **Technical answer:** A Rust `str` stores UTF-8 bytes, and string slice ranges require byte offsets that lie on character boundaries. A character ordinal identifies which `char` was visited but is not necessarily its byte address because one `char` uses one to four UTF-8 bytes.
  - **Plain-English analogy / example:**
    ```text
    "a💖b" character ordinals: 0, 1, 2
    "a💖b" byte positions:     0, 1, 5
    slicing needs byte addresses, not positions in a queue
    ```
  - **See also:** `topics/rust/02-ownership/slices.md`
- **Q:** "Please explain it to me" and "why and when we use this `PartialEq`?"
  - **Technical answer:** `T` makes the function reusable for different element types, while the `PartialEq` trait bound guarantees that values of `T` support `==` and `!=`. Deduplication needs that capability to decide whether neighboring elements are duplicates; a generic function that never compares its elements would not need this bound.
  - **Plain-English analogy / example:**
    ```rust
    fn same<T: PartialEq>(left: T, right: T) -> bool {
        left == right
    }
    ```
  - **See also:** `topics/rust/03-types-and-traits/generics.md`, `topics/rust/03-types-and-traits/traits.md`
- **Q:** "Give me a hint", "I don't know what to do next", and "I don't [feel] familiar with `while` syntax."
  - **Technical answer:** A `while` loop checks a Boolean condition before each iteration and repeats its block while that condition is true. This loop makes progress in two ways: removing a duplicate shortens the vector, while keeping a distinct element advances the mutable cursor.
  - **Plain-English analogy / example:**
    ```rust
    let mut index = 0;
    while index < 3 {
        index += 1;
    }
    ```
  - **See also:** `topics/rust/01-fundamentals/control-flow.md`
- **Q:** "Why `values[index - 1]`?"
  - **Technical answer:** Consecutive deduplication compares the current element at `index` with its immediate predecessor at `index - 1`. Starting at index 1 makes the subtraction safe, and unequal non-adjacent repetitions remain in the vector as required.
  - **Plain-English analogy / example:**
    ```text
    values:   [1, 2, 1]
    index 2:   previous=2, current=1
    unequal:   keep both; the earlier 1 is not adjacent
    ```
  - **See also:** `topics/rust/02-ownership/borrowing.md`
- **Q:** Should the cursor advance after removing a duplicate, and why does it ultimately stay unchanged?
  - **Technical answer:** `Vec::remove(index)` shifts every later element one position left. The newly shifted element at the same index has not yet been compared with its predecessor, so advancing would skip it; the cursor advances only when the current element is retained.
  - **Plain-English analogy / example:**
    ```text
    [1, 1, 1, 2], remove index 1 -> [1, 1, 2]
        ^ new current element still needs checking
    keep index 1 -> compare it with index 0 again
    ```
  - **See also:** `topics/rust/02-ownership/borrowing.md`
**Verification:** `cargo check`, `cargo test` (5 passed), `cargo clippy -- -D warnings`, `cargo run`, `cargo fmt`, and final `cargo fmt --check` passed. Tests cover duplicate runs, empty input, non-adjacent duplicates, duplicates at both edges, and a non-integer `&str` element type. Explanations were checked against [`str::char_indices`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.char_indices), [Rust Book string slicing](https://doc.rust-lang.org/book/ch08-02-strings.html#slicing-strings), [Rust Book mutable references](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references), and [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html).
**Question to answer later:** Can Liam independently reconstruct the cursor loop and its tests without hard-coded indices or step-by-step prompts?
**Next:** Read `topics/rust/02-ownership/lifetimes.md`, then explain how lifetime annotations constrain borrowed values without extending their lives.

