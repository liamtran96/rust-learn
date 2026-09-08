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

### 2026-09-04 - Ownership retrieval homework completion
**Working on:** `topics/rust/homework/2026-08-28-retrieval-02-ownership.md`
**What clicked:** Completed all six retrieval questions and connected moves, Copy, borrowing, cloning, lifetime relationships, iteration ownership, Vec reallocation, non-lexical lifetimes, disjoint mutable slices, and borrowed parser output. Canonical reference answers now sit beneath each completed question for later comparison.
**What didn't:** Several rules needed narrow retries: Copy on i32 was applied to Vec<i32>, push was first treated as unable to grow a full vector, an invalid split was expected to return empty slices, String was chosen where a borrowed &str input was required, and NLL needed to be rebuilt from its full name and a last-use timeline. The original parser prompt also blurred ownership by saying the function "receives an owned String," so it was clarified to show the caller owning command and passing &command.
**Questions asked this session:**
- **Q:** What is the root reason for a lifetime annotation, and does `'a` create or extend a lifetime?
  - **Technical answer:** A lifetime annotation names a validity relationship that the compiler checks; it does not create time, keep a value alive, or add runtime work. It lets Rust reject a value containing a reference when that value might be used after the referenced source is dropped.
  - **Plain-English analogy / example:**
    ```text
    source text -> owned book
    &'a str     -> bookmark into that book
    'a          -> rule: bookmark cannot outlast book
    check       -> compile time only
    ```
  - **See also:** `topics/rust/02-ownership/lifetimes.md`
- **Q:** Why does i32 being Copy not make Vec<i32> Copy, and why does iterating over &values preserve the vector?
  - **Technical answer:** i32 values can be copied, but Vec<i32> separately owns a heap allocation and is not Copy. An owned-vector iterator consumes the Vec and yields i32, while iterating over &values borrows the Vec and yields &i32, leaving the container owned by values.
  - **Plain-English analogy / example:**
    ```rust
    let values = vec![1, 2, 3];
    for item in &values { println!("{item}"); }
    println!("{}", values.len()); // values was borrowed
    ```
  - **See also:** `topics/rust/02-ownership/ownership.md`, `topics/rust/02-ownership/borrowing.md`
- **Q:** Why does Vec::push conflict with a reference to an element, and what does NLL stand for?
  - **Technical answer:** push mutably borrows the Vec and may move its elements into a larger allocation, so it cannot overlap a still-needed shared reference into the old buffer. NLL means non-lexical lifetimes: a borrow can end at its final use rather than automatically lasting to the closing brace.
  - **Plain-English analogy / example:**
    ```rust
    scores.push(40);              // mutable borrow ends
    let first = &scores[0];       // shared borrow begins
    println!("{first}");          // final use; shared borrow ends
    ```
  - **See also:** `topics/rust/02-ownership/borrowing.md`
- **Q:** What does split_at_mut panicking mean, and does an empty slice start at index 0 or 1?
  - **Technical answer:** A panic is a runtime failure, not a compile error; split_at_mut panics when mid is greater than the slice length. Rust indices start at zero, but an empty slice contains no valid element index; zero is still a valid boundary at which to split.
  - **Plain-English analogy / example:**
    ```text
    values length:       3
    element indices:     0, 1, 2
    split boundaries:    0, 1, 2, 3
    split boundary 4:    panic
    ```
  - **See also:** `topics/rust/02-ownership/slices.md`
- **Q:** Why could adjust_halves not be printed directly with `{}`?
  - **Technical answer:** adjust_halves mutates through &mut [i32] and returns unit (), which does not implement Display for `{}`. Call the function first, then print the mutated array with debug formatting; the array binding must be mutable to pass &mut values.
  - **Plain-English analogy / example:**
    ```rust
    let mut values = [10, 20, 30, 40];
    adjust_halves(&mut values, 2);
    println!("{values:?}");
    ```
  - **See also:** `topics/rust/01-fundamentals/functions.md`, `topics/rust/02-ownership/borrowing.md`
- **Q:** How should a first-word helper borrow input and return a word without allocation, and what changes if it returns String?
  - **Technical answer:** The signature `fn first_word(text: &str) -> &str` borrows the caller's text and returns a non-owning slice into the same storage. The result cannot outlive the source; returning String instead allocates and copies an independently owned word that can remain valid after the source is dropped.
  - **Plain-English analogy / example:**
    ```rust
    let command = String::from("deploy production");
    let word = first_word(&command); // borrowed "deploy"
    println!("{word}");
    ```
  - **See also:** `topics/rust/02-ownership/slices.md`, `topics/rust/02-ownership/lifetimes.md`
- **Q:** How should homework attempts and final answers be tracked and committed?
  - **Technical answer:** Preserve each attempt and its review status, but do not create a commit for every retry. After the entire numbered question is correct, add its canonical reference answer and make one focused completion commit while leaving unrelated changes unstaged.
  - **Plain-English analogy / example:**
    ```text
    retries -> saved, uncommitted
    whole question correct -> add reference answer
    completed question -> one focused commit
    unrelated code -> remains unstaged
    ```
  - **See also:** `.agents/skills/homework/SKILL.md`
**Question to answer later:** After spacing, can Liam explain Vec reallocation, NLL, split boundaries, and borrowed parser output without step-by-step prompts?
**Next:** Read `topics/rust/02-ownership/lifetimes.md`, then begin the full Scanner exercise from `topics/rust/exercises/ch02-ownership.md`.

### 2026-09-07 - Full borrowed Scanner: syntax, state, and tests
**Working on:** Ch 2 exercise 3, borrowed `Scanner` - `code/02-ownership/scanner/`
**What clicked:** Built a struct and instance, added methods, returned `Option<char>` from a `match`, and wrote three passing tests. Liam correctly explained that advancing needs mutation of the cursor; the final code uses a shared receiver for peeking and an exclusive receiver for advancing. Codex performed the requested final tail-expression cleanup and formatting.
**What didn't:** Syntax needed small explicit examples throughout. Repeated difficulties included printing versus returning a value, using a local from another function, method definitions versus calls, one character versus a string, and relating test input to cursor state. UTF-8 byte positions were explained, but independent recall remains unverified: Liam answered "make it more flexible" for `len_utf8` and "r" when asked for the numeric position after `é`.
**Questions asked this session:**
- **Q:** "$next", "give me a hint", and repeated "what's next?"
  - **Technical answer:** The recorded next action was the full borrowed Scanner, whose crate was already scaffolded. The session progressed through state declaration, a peeking method, an advancing method, and three tests, with each attempt checked before the next step.
  - **Plain-English analogy / example:** Build a bookmark first, teach it to look at the next letter, then move it, then check each behavior.
  - **See also:** `WORKFLOW.md`, `topics/rust/exercises/ch02-ownership.md`
- **Q:** "i am not familar with the syntax" (struct and instance)
  - **Technical answer:** A struct is a custom type with named fields; its declaration uses `field: Type`, while an instance supplies `field: value`. `Scanner<'a>` declares a lifetime parameter, and `source: &'a str` connects the stored reference to that validity relationship; annotations do not extend the source's lifetime.
  - **Plain-English analogy / example:** The struct definition is a blank bookmark form with spaces for a book and position; an instance fills those spaces with specific text and zero.
  - **See also:** `topics/rust/03-types-and-traits/structs.md`, `topics/rust/02-ownership/lifetimes.md`
- **Q:** "ok done what's next" (method syntax)
  - **Technical answer:** An `impl<'a> Scanner<'a>` block defines behavior for the scanner type using the lifetime name declared for that block. `fn peek(&self) -> Option<char>` defines a method with a shared receiver and a result containing either one character or no character; the caller writes `scanner.peek()`.
  - **Plain-English analogy / example:** The method definition gives a bookmark an instruction; the dot call asks one particular bookmark to perform it.
  - **See also:** `topics/rust/03-types-and-traits/structs.md`, `topics/rust/02-ownership/borrowing.md`
- **Q:** "hmm i am not familiar with syntax" / "still dont understand" (Option and match)
  - **Technical answer:** `peek` already returns `Option<char>`, so calling `.chars()` on its result is a type mismatch. A `match` selects `Some(ch)` or `None`; `ch` names the contained character only within that branch, and `=>` introduces the branch's code.
  - **Plain-English analogy / example:** `Some(ch)` opens an occupied envelope and names its contents; `None` is the empty case. The envelope and its contents have different types.
  - **See also:** `topics/rust/03-types-and-traits/pattern-matching.md`, `topics/rust/05-error-handling/result-option.md`
- **Q:** How do the advance branches return their results? (repeated guided attempts)
  - **Technical answer:** The method promises `Option<char>`, so each branch must produce `Some(ch)` or `None`. A tail expression is the last expression without a semicolon; removing a semicolon from `println!` still leaves unit `()` because printing does not produce the printed value.
  - **Plain-English analogy / example:** Announcing a parcel's contents is different from handing the parcel to the caller; `println!` announces, while `Some(ch)` supplies the result.
  - **See also:** `topics/rust/01-fundamentals/functions.md`, `topics/rust/01-fundamentals/control-flow.md`
- **Q:** "i dont know" (creating a scanner in a test)
  - **Technical answer:** Local variables belong to the function body where they are declared; the test cannot use `main`'s local `scanner`. A test is a function marked `#[test]` and must set up its own scanner before using `assert_eq!` to compare actual and expected values.
  - **Plain-English analogy / example:** Each test has its own workbench and must put its own scanner on it; tools on main's workbench are not automatically available.
  - **See also:** `topics/rust/01-fundamentals/variables.md`, `topics/rust/07-testing/unit-tests.md`
- **Q:** Why did `Some('ru')` fail, and what does a second peek return? (assertion review)
  - **Technical answer:** A `char` literal contains one Unicode scalar value; `'ru'` contains two, while `"ru"` is a string slice and does not match `Option<char>`. Peeking does not move the cursor or accumulate characters, so repeated peeks at position zero in `"rust"` return `Some('r')`.
  - **Plain-English analogy / example:** Looking at the same bookmark twice shows the same letter, not a growing selection of letters.
  - **See also:** `topics/rust/02-ownership/slices.md`, `topics/rust/01-fundamentals/data-types.md`
- **Q:** "still dont understand" (the end-of-text test)
  - **Technical answer:** One advance consumes one character, so a scanner over `"rust"` still has `"ust"` left after returning `'r'`. The test changed its source to `"r"` so one advance really reached the end; subsequent peek and advance return `None`, leaving the cursor at 1.
  - **Plain-English analogy / example:** `|rust -> r|ust` still has text ahead, whereas `|r -> r|` has reached the end; the bar is the cursor.
  - **See also:** `topics/rust/02-ownership/slices.md`, `topics/rust/07-testing/unit-tests.md`
- **Q:** "do it for me" (final cleanup)
  - **Technical answer:** Clippy flagged an unnecessary binding whose value was immediately returned from `peek`. Codex replaced `let first = remaining.chars().next(); first` with the direct tail expression and ran formatting; behavior stayed the same.
  - **Plain-English analogy / example:** Pass the result straight to the caller instead of attaching a temporary name immediately before handing it over.
  - **See also:** `topics/rust/01-fundamentals/functions.md`, `topics/rust/cheatsheets/cargo-commands.md`
- **Q:** Why `&mut self`, and why `ch.len_utf8()`? (closeout recall)
  - **Technical answer:** `advance` updates the cursor and therefore requires exclusive mutable access, while `peek` only reads. The cursor measures UTF-8 bytes, so advancing over `é` moves from 0 to 2; the next character can be `r`, but that character is not the numeric cursor position.
  - **Plain-English analogy / example:** Letters occupy different widths on a byte ruler: `é` covers two marks, so the bookmark must jump two marks to reach the next letter.
  - **See also:** `topics/rust/02-ownership/borrowing.md`, `topics/rust/02-ownership/slices.md`
- **Q:** "ok what's next", "are we done?", and "yes please" (session closeout)
  - **Technical answer:** Borrowed Scanner exercise 3 is complete with its three required tests and verification. Owning a `String` is separate exercise 4; it remains unstarted, and the session should be journaled before moving on rather than silently extending the task.
  - **Plain-English analogy / example:** Finish and record one lesson before opening the next; owning the book is a different design from storing a bookmark into someone else's book.
  - **See also:** `WORKFLOW.md`, `topics/rust/exercises/ch02-ownership.md`, `topics/rust/02-ownership/ownership.md`
**Verification:** `cargo fmt --check`, `cargo check`, `cargo test` (3 passed), and `cargo clippy -- -D warnings` passed after cleanup. The runtime demonstration printed `Some('r')`, `Some('r')`, `Some('u')`. Unicode and initially empty input were not tested; direct slicing assumes an in-range cursor at a UTF-8 character boundary.
**Official sources checked during the session:** [Rust Book structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html), [methods](https://doc.rust-lang.org/book/ch05-03-method-syntax.html), [lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html), [tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html), [str::chars](https://doc.rust-lang.org/std/primitive.str.html#method.chars), [Iterator::next](https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next), [char::len_utf8](https://doc.rust-lang.org/std/primitive.char.html#method.len_utf8).
**Question to answer later:** Can Liam independently reconstruct a method call and test setup, distinguish printing from returning, and predict the byte cursor after a multibyte character?
**Next:** Short retrieval warm-up, then Ch 2 exercise 4: refactor Scanner to own `String` in the existing crate and compare ownership choices. No chapter completion or new chapter homework yet.

### 2026-09-08 - Owned Scanner: field borrowing and allocation
**Working on:** Ch 2 exercise 4, owned `Scanner` - `code/02-ownership/scanner/`
**What clicked:** Liam correctly recalled that `&self` permits reading and `&mut self` permits mutation of this Scanner. He refactored the source to String, removed the struct/impl lifetime parameter, converted the four initializers, and explicitly borrowed the field inside peek. After discussing ownership choices, he correctly answered "just one" text buffer for constructing a String and moving it into Scanner.
**What didn't:** Moving the owned field through a shared receiver needed a concrete explanation; merely accessing a field does not automatically borrow it in a by-value assignment. Initially equated owning String with allocating additional heap space. Some repeated checks saw the previous saved file; those are not treated as separate conceptual mistakes.
**Questions asked this session:**
- **Q:** "$next", "check it again", and "what should i do now?"
  - **Technical answer:** The authoritative next action was exercise 4 in the existing scanner crate. The borrowed Scanner remained officially complete; the owned variant required changing its stored input, adjusting construction, and comparing ownership choices.
  - **Plain-English analogy / example:** Keep the same bookmark behavior, but let the scanner carry its own book instead of pointing into a book held elsewhere.
  - **See also:** [[../../exercises/ch02-ownership|Ch 2 exercises]], `WORKFLOW.md`
- **Q:** "&self is reference to Scanner but we can not modify it on the other hand &mut self we can modify it" (warm-up answer)
  - **Technical answer:** Correct for these fields: &self is a shared borrow, while &mut self is an exclusive borrow permitting mutation. Neither receiver takes ownership of the Scanner, so peek observes and advance can update its cursor.
  - **Plain-English analogy / example:** A read-only visitor can inspect the bookmark position; an exclusive editor can change it.
  - **See also:** [[../../02-ownership/borrowing|Borrowing]]
- **Q:** "done help me check" and subsequent "done" checks
  - **Technical answer:** Changing the field to String initially left four string-literal initializers with the wrong type (E0308) and a field assignment attempting to move through a shared reference (E0507). Liam supplied owned inputs and then borrowed the field; the final saved code passed formatting, compilation, all three tests, and strict Clippy.
  - **Plain-English analogy / example:** The input must match the new storage contract, and a method that only reads must leave the stored text with its owner.
  - **See also:** [[../../02-ownership/ownership|Ownership]], `topics/rust/cheatsheets/cargo-commands.md`
- **Q:** "why is that? i still dont understand" (why `let text = self.source` fails)
  - **Technical answer:** With the earlier &str field, assignment copied a shared reference because shared references implement Copy, which permits implicit duplication. With String, the same assignment tries to move the owned value out through &self, which Rust rejects; `&self.source` instead creates a shared reference to the field.
  - **Plain-English analogy / example:** Borrowing a backpack lets you inspect the book inside, but does not let you take ownership of that book.
  - **See also:** [[../../02-ownership/ownership|Ownership]], [[../../02-ownership/borrowing|Borrowing]]
- **Q:** "i think borrowing &str is better because owing a String create a new space in heap" (ownership comparison)
  - **Technical answer:** Borrowing avoids allocating or copying source text and is suitable when the source stays valid while the scanner uses it. Owning an existing String does not itself allocate another text buffer: moving it transfers responsibility for the same buffer. String::from("rust") creates that initial buffer; Liam correctly identified one buffer after the move.
  - **Plain-English analogy / example:** Transferring a book to another owner does not print a second book; making the original book and handing it over are separate actions.
  - **See also:** [[../../02-ownership/ownership|Ownership]], [[../../02-ownership/lifetimes|Lifetimes]]
- **Q:** "what do u mean by who someone else?"
  - **Technical answer:** Here the other owner is a concrete variable, such as `input: String` in the calling function. The borrowed Scanner's source points into input's text, so that text must remain valid while the scanner uses it; moving input into the owned Scanner instead makes scanner.source the owner and leaves input unusable.
  - **Plain-English analogy / example:** In the borrowed version, input holds the book and Scanner holds a bookmark pointing into it. In the owned version, Scanner holds both book and bookmark.
  - **See also:** [[../../02-ownership/lifetimes|Lifetimes]], [[../../02-ownership/ownership|Ownership]]
**Verification:** `cargo fmt --check`, `cargo check`, `cargo test` (3 passed), and `cargo clippy -- -D warnings` passed. Tests cover ASCII peeking, advancing, and end-of-text; multibyte input remains untested. Explanations checked against the [Rust Book on ownership and moves](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html), [method syntax](https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html), and [E0507](https://doc.rust-lang.org/error_codes/E0507.html).
**Question to answer later:** Can Liam independently explain why a borrowed Scanner cannot outlive its source, and recall why cursor positions count UTF-8 bytes?
**Next:** After brief ownership recall, scaffold the Week 3 hand-written string-splitting task with `$new-exercise 02 split-text`; dedup follows. Reading and the split/dedup shipping milestone remain open, so this is not a chapter closeout.
