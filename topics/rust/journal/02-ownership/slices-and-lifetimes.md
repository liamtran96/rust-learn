---
title: Ownership Journal - Slices And Lifetimes
tags: [rust, journal, ownership]
---

# Slices And Lifetimes

> Ownership topic journal. Return to [[../../journal|Journal index]].

## Entries

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

### 2026-08-26 - Explicit lifetime relationships
**Working on:** Ownership drill d08 - `code/02-ownership/drills-ownership/tests/d08_lifetime_elision.rs`
**What clicked:** With one borrowed input, lifetime elision can connect the returned reference to that input automatically. Because `longest` may return either of two inputs, the shared `'a` annotation explicitly connects both input references to the output and restricts the result to a period valid for both.
**What didn't:** Initially described `'a` as identifying which parameter is returned and then as keeping the parameters valid at runtime. Lifetime annotations neither choose the returned value nor extend any value's lifetime; they describe constraints that the borrow checker verifies.
**Questions asked this session:**
- **Q:** Why does `longest` need the same named lifetime on both inputs and the output?
  - **Technical answer:** The function chooses between `x` and `y` at runtime, so its result might borrow from either input. The shared `'a` gives the caller a safe contract: both inputs must be valid for `'a`, and the returned reference cannot be used beyond that common period.
  - **Plain-English analogy / example:**
    ```rust
    let outer = String::from("outside");
    {
        let inner = String::from("inside");
        let result = longest(&outer, &inner);
        println!("{result}"); // both possible sources are alive
    }
    ```
  - **See also:** `topics/rust/02-ownership/lifetimes.md`
**Question to answer later:** If a two-parameter function always returns only `x`, how can its output lifetime be tied to `x` without tying it to `y`?
**Next:** Complete ownership drill d09: fill `PREDICT:`, run `cargo test --test d09_slice_window`, implement it, then fill `WHY:`.

### 2026-08-26 - Returning a borrowed word slice
**Working on:** Ownership drill d09 - `code/02-ownership/drills-ownership/tests/d09_slice_window.rs`
**What clicked:** `split_whitespace` iterates over `&str` subslices borrowed from the input, `next` produces `Option<&str>`, and `unwrap_or("")` supplies a borrowed empty fallback. Leaving that final expression without a semicolon returns the slice without allocating or copying text.
**What didn't:** Initially predicted that `todo!()` prevented compilation, but it compiles and panics only when executed. The first implementation discarded the desired `&str` with a semicolon and then reached the remaining `todo!()`; the explanation also described the slice as being removed rather than borrowed from the still-existing input.
**Questions asked this session:**
- **Q:** How do I implement the borrowed first-word function when I do not know how to combine the APIs?
  - **Technical answer:** Break the transformation into typed stages: `split_whitespace` creates an iterator of borrowed slices, `next` returns the first as `Option<&str>`, and `unwrap_or` converts that option into a `&str` using `""` for `None`. The final `&str` must be the function's tail expression so it is returned rather than discarded.
  - **Plain-English analogy / example:**
    ```rust
    let mut pieces = "one two".split_whitespace();
    let first: Option<&str> = pieces.next();
    let word: &str = first.unwrap_or("");
    assert_eq!(word, "one");
    ```
  - **See also:** `topics/rust/02-ownership/slices.md`, `topics/rust/01-fundamentals/functions.md`
**Question to answer later:** How would a byte-index implementation safely find the same slice boundary without allocating?
**Next:** Complete ownership drill d10: fill `PREDICT:`, run `cargo test --test d10_mut_through_ref`, implement it, then fill `WHY:`.

### 2026-08-27 - Borrowing struct preview (d12 in progress)
**Working on:** Ownership drill d12 - `code/02-ownership/drills-ownership/tests/d12_scanner_peek.rs`
**What clicked:** The visual model separates the owned text from the borrowing `Scanner`: the text is the book, while `Scanner` is a bookmark containing a borrowed view and its own cursor position. `&self` means read the bookmark, while `&mut self` permits moving its position.
**What didn't:** Rust struct and `impl` syntax is new material from the later types chapter, so combining it with a lifetime and two receiver forms made the drill difficult to parse before any implementation attempt. The drill remains incomplete: `PREDICT:` and `WHY:` are empty and both method bodies still contain `todo!()`.
**Questions asked this session:**
- **Q:** What is a struct in Rust, and how does it compare with a struct in Go?
  - **Technical answer:** A Rust struct is a custom type that groups named fields into one value; Go structs serve the same basic data-grouping purpose. Rust places associated functions and methods in an `impl Type` block and uses `self`, `&self`, or `&mut self` to state ownership and borrowing, whereas Go declares methods separately with value or pointer receiver parameters.
  - **Plain-English analogy / example:**
    ```rust
    struct Bookmark { page: usize }
    impl Bookmark {
        fn page(&self) -> usize { self.page }
    }
    ```
  - **See also:** `topics/rust/03-types-and-traits/structs.md`, `topics/rust/02-ownership/visuals/d12-scanner-borrowing.svg`
- **Q:** Why does d12 need this unfamiliar syntax: `Scanner<'a>`, `impl<'a>`, `&self`, and `&mut self`?
  - **Technical answer:** `Scanner<'a>` says the value contains a reference valid for a compiler-checked lifetime relationship named `'a`, and `impl<'a>` makes that name available while defining behavior for the type. `&self` temporarily borrows the scanner read-only; `&mut self` temporarily borrows it exclusively so a field such as the cursor position may change.
  - **Plain-English analogy / example:**
    ```text
    source: &'a str -> which book the bookmark refers to
    pos: usize      -> where the bookmark currently sits
    &self           -> look at the bookmark
    &mut self       -> move the bookmark
    ```
  - **See also:** `topics/rust/02-ownership/borrowing.md`, `topics/rust/02-ownership/lifetimes.md`, `topics/rust/02-ownership/visuals/d12-scanner-borrowing.svg`
- **Q:** Can Codex connect to my tldraw account through MCP to explain this visually?
  - **Technical answer:** tldraw publishes an MCP App that exposes canvas shape creation, editing, and deletion to supported MCP clients. No tldraw connector is installed in this Codex session, so the safe fallback was a standalone SVG rather than claiming account access that was not available.
  - **Plain-English analogy / example:**
    ```text
    MCP connector installed -> agent can operate the shared canvas
    no connector here        -> agent creates a portable SVG
    SVG                      -> open directly or bring into a canvas
    ```
  - **See also:** `topics/rust/02-ownership/visuals/d12-scanner-borrowing.svg`
**Question to answer later:** After filling the prediction, can Liam identify which field `peek` only reads and which field `advance` must change?
**Next:** Reopen `topics/rust/02-ownership/visuals/d12-scanner-borrowing.svg`, then fill d12's `PREDICT:` in Liam's own words before running `cargo test --test d12_scanner_peek`.

### 2026-08-27 - `strip_margin`: line iterators, slices, and `Option`
**Working on:** `strip_margin` - `code/02-ownership/strip-margin/`
**What clicked:** `str::lines` preserves the line-sized units needed by the problem; `Iterator::map` transforms every line, while `Option::map` transforms a found prefix index only when it exists. `find` returns a UTF-8 byte index, `char::len_utf8` moves beyond the prefix safely, the intermediate slices keep borrowing from the input, and `join` finally creates the owned `String` required by the signature.
**What didn't:** The first attempt used `split_whitespace` and joined words with spaces, which erased line boundaries. The unfamiliar closure, iterator, `Option`, slicing, and type-inference syntax made the working pipeline difficult to read even after it produced the expected output.
**Questions asked this session:**
- **Q:** Can you change the way you write the brief because I am not familiar with the syntax in Rust, while I still implement it by hand?
  - **Technical answer:** The exercise brief now separates syntax literacy from implementation: it explains `fn`, borrowed `&str` input, `char`, and owned `String` output without filling the function body. Future briefs follow the same rule, so syntax is decoded while the algorithm and every implementation line remain Liam's work.
  - **Plain-English analogy / example:**
    ```rust
    fn transform(input: &str, marker: char) -> String {
        todo!() // signature explained; implementation remains yours
    }
    ```
  - **See also:** `code/02-ownership/strip-margin/BRIEF.md`, `topics/rust/02-ownership/slices.md`
- **Q:** What should I do next for `strip_margin`; can I get some hints?
  - **Technical answer:** `split_whitespace` produces whitespace-separated words, so it cannot retain line boundaries. `lines` instead produces an iterator whose items are borrowed `&str` lines, matching the unit that this exercise needs to transform.
  - **Plain-English analogy / example:**
    ```rust
    let text = "first\nsecond";
    for line in text.lines() {
        println!("{line}");
    }
    ```
  - **See also:** `topics/rust/02-ownership/slices.md`, `topics/rust/pitfalls.md`
- **Q:** Can you check the implementation and explain it to me?
  - **Technical answer:** The outer `Iterator::map` transforms each input line, while `find` returns `Option<usize>` because the prefix may be absent. The inner `Option::map` creates a slice after a found prefix, `unwrap_or(line)` preserves a line without a prefix, and collecting plus joining converts the borrowed slices into the returned owned `String`.
  - **Plain-English analogy / example:**
    ```text
    input text -> borrowed lines
    each line  -> suffix after prefix, or original line
    slices     -> collect and join with newlines
    result     -> owned String
    ```
  - **See also:** `topics/rust/02-ownership/slices.md`
**Question to answer later:** Should `strip_margin` preserve a final newline, and should a prefix after non-whitespace text count as a margin marker?
**Next:** Scaffold and begin Ch 2 `split_at_mut`; d12 remains deferred to Week 3.

### 2026-08-27 - Generic mutable-slice splitting
**Working on:** `split_at_mut` - `code/02-ownership/split-at-mut/`
**What clicked:** A generic type parameter `T` lets one function split slices of any element type while preserving compile-time type checking. The standard `slice::split_at_mut` method returns two non-overlapping mutable slice references in a tuple; tuple destructuring names those results in the caller, and mutations through either slice update the original array.
**What didn't:** Generic function syntax, method-call syntax, dereferencing, tuple returns, and block scope were initially unfamiliar. Separate range indexing looked safe but the borrow checker could not prove the ranges disjoint; split positions were also briefly treated as if splitting padded, duplicated, or rearranged elements.
**Questions asked this session:**
- **Q:** What is a generic in Rust?
  - **Technical answer:** A generic type parameter such as `T` is a compile-time placeholder for a concrete type selected at each use. Every `T` in one instantiation means the same concrete type, so `&mut [T]` is a mutable slice whose elements are all that type; generics preserve type checking rather than accepting arbitrary mixed values.
  - **Plain-English analogy / example:**
    ```rust
    fn length<T>(items: &[T]) -> usize {
        items.len()
    }
    assert_eq!(length(&[10, 20]), 2);
    ```
  - **See also:** `topics/rust/03-types-and-traits/generics.md`
- **Q:** Can I get hints because I am not familiar with the syntax?
  - **Technical answer:** The signature declares `T` in `<T>`, takes `v` as an exclusive borrowed slice with `&mut [T]`, takes the split index as `usize`, and returns a tuple `(&mut [T], &mut [T])`. A temporary `todo!()` body allows the call site and tuple destructuring to be set up before implementing the split.
  - **Plain-English analogy / example:**
    ```rust
    fn example<T>(values: &mut [T], index: usize) -> (&mut [T], &mut [T]) {
        todo!()
    }
    ```
  - **See also:** `code/02-ownership/split-at-mut/BRIEF.md`, `topics/rust/02-ownership/slices.md`
- **Q:** What is `*v`?
  - **Technical answer:** The unary `*` operator dereferences a reference, meaning it accesses the value behind that reference. With `v: &mut [T]`, `v` is the mutable reference and `*v` denotes the underlying slice; slice indexing can perform this dereference automatically.
  - **Plain-English analogy / example:**
    ```rust
    let mut number = 10;
    let reference = &mut number;
    *reference = 20;
    assert_eq!(number, 20);
    ```
  - **See also:** `topics/rust/02-ownership/borrowing.md`
- **Q:** Why is `let right = &mut v[mid..];` rejected after `let left = &mut v[..mid];`?
  - **Technical answer:** Returning `left` keeps its mutable borrow active while the second borrow is created. Although the two ranges are mathematically disjoint, separate slice-indexing expressions are both treated as mutable borrows from `*v`, and the borrow checker does not prove their non-overlap from those expressions.
  - **Plain-English analogy / example:**
    ```text
    first request:  borrow part of one strip exclusively
    second request: borrow from that same strip again
    indexing alone: does not provide a proof that the parts differ
    split_at_mut:   provides that non-overlap guarantee
    ```
  - **See also:** `topics/rust/02-ownership/borrowing.md`, `topics/rust/02-ownership/slices.md`
- **Q:** What does `v.split_at_mut(mid)` return?
  - **Technical answer:** It returns one tuple containing two mutable slice references with type `(&mut [T], &mut [T])`. The first covers indices `0..mid`, the second covers `mid..len`, and both borrow the original data rather than copying it; the method panics at runtime when `mid > len`.
  - **Plain-English analogy / example:**
    ```rust
    let mut values = [10, 20, 30, 40];
    let (left, right) = values.split_at_mut(2);
    assert_eq!(left, [10, 20]);
    assert_eq!(right, [30, 40]);
    ```
  - **See also:** `topics/rust/02-ownership/slices.md`
**Question to answer later:** How can the standard library implement `split_at_mut` internally when the straightforward safe indexing version is rejected?
**Next:** Resume ownership drill d12: fill `PREDICT:` in Liam's own words, then run `cargo test --test d12_scanner_peek`.

### 2026-08-28 - A borrowing scanner with methods
**Working on:** Ownership drill d12 - `code/02-ownership/drills-ownership/tests/d12_scanner_peek.rs`
**What clicked:** A `Scanner` groups a borrowed source and an owned cursor; `peek(&self)` reads without moving the cursor, while `advance(&mut self)` may update it. A suffix slice plus `chars().next()` returns `Option<char>`, and a scanner is a reduced form of the cursor used by parsers for commands, configuration, and expressions.
**What didn't:** `struct`, `impl`, `Self`, receiver, and lifetime syntax were still outside the learned syntax boundary and needed to be decoded before implementation. The first `peek` copied demonstration constants literally, then bound `self.pos` to a variable without using it as the slice start. The final `WHY:` still described `'a` as creating a lifetime rather than naming a validity relationship.
**Questions asked this session:**
- **Q:** "I don't familiar with the syntax; please remember that and don't make me say that again and again."
  - **Technical answer:** New syntax must be introduced before it is required in an implementation. `struct Scanner<'a>` declares a type with a lifetime parameter, `impl<'a> Scanner<'a>` defines its methods, and `&self` versus `&mut self` states whether a method only reads or may mutate the current scanner.
  - **Plain-English analogy / example:**
    ```text
    source: &'a str -> borrowed book
    pos: usize      -> owned bookmark position
    &self           -> inspect the bookmark
    &mut self       -> move the bookmark
    ```
  - **See also:** `topics/rust/02-ownership/visuals/d12-scanner-borrowing.svg`, `topics/rust/03-types-and-traits/structs.md`
- **Q:** "I still don't understand what should I do."
  - **Technical answer:** `advance` first asks `peek` for the current `Option<char>`. It increments `pos` only when that option is `Some`, then returns the original option; this preserves `None` at end-of-input and avoids moving beyond the ASCII source.
  - **Plain-English analogy / example:**
    ```rust
    let next = queue.front();
    if next.is_some() {
        // move the cursor only when an item exists
    }
    ```
  - **See also:** `topics/rust/05-error-handling/result-option.md`
- **Q:** "Why do I need to learn this, and how do I apply it to build a real application?"
  - **Technical answer:** The scanner is a small stateful parser: it combines input with a cursor and separates observation from mutation. Real applications use this shape to parse commands, configuration, search filters, logs, and expressions; the lifetime prevents the parser from retaining a reference after its input is no longer valid.
  - **Plain-English analogy / example:**
    ```text
    peek digit  -> decide to parse a number
    advance     -> consume that digit
    peek '+'    -> decide to parse an operator
    end of text -> return None safely
    ```
  - **See also:** `topics/rust/02-ownership/lifetimes.md`, `topics/rust/02-ownership/visuals/d12-scanner-borrowing.svg`
- **Q:** "Make sure in the future you explain the code for me."
  - **Technical answer:** Future sessions will introduce unfamiliar code in three layers before implementation: its purpose, its connection to a real application, and the meaning of each new syntax element. The repository instructions now make this a persistent teaching rule rather than relying on Liam to repeat the preference.
  - **Plain-English analogy / example:**
    ```text
    why it exists       -> destination
    application example -> map
    syntax walkthrough  -> road signs
    implementation      -> Liam drives
    ```
  - **See also:** `AGENTS.md`, `WORKFLOW.md`
**Question to answer later:** Can Liam explain that `'a` constrains the scanner/source relationship without saying that it creates or extends a lifetime?
**Next:** Read `topics/rust/02-ownership/lifetimes.md`, then explain how `Scanner<'a>` prevents a borrowed scanner from outliving its source text.
