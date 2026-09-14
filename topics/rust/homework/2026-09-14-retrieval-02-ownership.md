# Rust Retrieval Homework - 2026-09-14

**Focus:** Chapter 2 ownership, borrowing, slices, and lifetimes
**Based on:** official progress at Phase 3 / Week 4; questions are limited to completed Chapter 2 material
**Timebox:** 25-35 minutes

## Instructions

Answer from memory first. For code questions, predict before running anything. Explain
the Rust rule behind each answer. Write answers directly below each prompt.

## Questions

### 1. Move, drop, borrow, and allocation

Explain what happens at each marked line. State which binding owns the text, whether a
new text buffer is allocated, when the original binding becomes unusable, and when the
buffer is finally dropped.

```rust
let title = String::from("field guide"); // A
let reader = &title;                     // B
println!("{reader}");                    // C
let archived = title;                    // D
println!("{archived}");                  // E
```

Then explain how replacing line D with `let archived = title.clone();` changes ownership
and allocation.

**Your answer:**

### 2. Borrowing rule and non-lexical lifetimes

In your own words, state the rule governing multiple shared references and an exclusive
mutable reference. Then explain what a non-lexical lifetime is and why it can allow a
mutation before the surrounding block ends.

Use one short example of a shared borrow whose final use occurs before a later mutable
borrow.

**Your answer:**

### 3. Diagnose a vector borrow conflict

Predict whether each version compiles. Identify the shared and mutable borrows, explain
why capacity does not change the compiler's decision, and name the exact point where the
shared borrow ends.

```rust
// Version A
let mut queue = vec![5, 10, 15];
let head = &queue[0];
queue.push(20);
println!("{head}");
```

```rust
// Version B
let mut queue = vec![5, 10, 15];
let head = &queue[0];
println!("{head}");
queue.push(20);
```

**Your answer:**

### 4. Trace UTF-8 slice boundaries

Without running this code, predict separately what happens at A, B, and C: does the line
produce a slice, fail to compile, or panic at runtime? If it produces a slice, give its
text. Explain your answers using UTF-8 byte boundaries rather than character counts.

```rust
let text = String::from("café");
let a = &text[..3]; // A
let b = &text[..4]; // B
let c = &text[..5]; // C
```

**Your answer:**

### 5. Write a borrowed word helper

Write this function without allocating or cloning:

```rust
fn first_word(text: &str) -> &str {
    // Your implementation
}
```

Return the first whitespace-separated word, or an empty string when the input has no
word. Then explain who owns the returned characters and why the returned slice cannot be
used after the input's owner is dropped.

**Your answer:**

### 6. Transfer: choose a scanner ownership model

A configuration parser needs to store source text and a cursor. Compare these designs:

```rust
struct BorrowedScanner<'a> {
    source: &'a str,
    pos: usize,
}

struct OwnedScanner {
    source: String,
    pos: usize,
}
```

Choose one design for each situation and justify it:

1. The caller already owns a large configuration string and the scanner is used only
   during one function call.
2. The scanner must be returned and stored independently of the caller's local string.

Explain what `'a` guarantees, whether moving an existing `String` into `OwnedScanner`
allocates a second text buffer, and what tradeoff each design makes.

**Your answer:**

## Confidence check

For each question, rate confidence from 1 (guessing) to 5 (certain) before checking notes.
