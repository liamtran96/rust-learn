# Rust Retrieval Homework - 2026-08-28

**Focus:** ownership, borrowing, slices, and lifetimes
**Based on:** completed material through Phase 2, Week 3 (ownership drill d12, `strip_margin`, and `split_at_mut`)
**Timebox:** 30 minutes

## Instructions

Answer from memory first. For code questions, predict before running anything. Explain
the Rust rule behind each answer. Write answers directly below each prompt.

## Questions

### 1. Move, copy, borrow, or clone?

Explain the difference between moving, copying, borrowing, and cloning a value. For each
line below, state which operation occurs, who owns the value afterward, and whether the
source binding remains usable.

```rust
let count = 8_i32;
let other_count = count;

let title = String::from("Ownership");
let reader = &title;

let archived = title.clone();
let published = title;
```

Also explain why using `clone()` merely to make a borrow-checker error disappear can hide
the real ownership decision.

**Your answer:**

### 2. What a lifetime annotation actually says

In your own words, explain what `'a` means in this type and what relationship Rust checks:

```rust
struct Cursor<'a> {
    source: &'a str,
    pos: usize,
}
```

Does `'a` create a lifetime, keep `source` alive, or change anything at runtime? Explain
why Rust must reject using a `Cursor` after the text referenced by `source` has been
dropped.

**Your answer:**

### 3. Trace two kinds of iteration

Without running the code, analyze Version A and Version B separately. For each version:

- state the type of `item` inside the loop;
- state whether the loop consumes or borrows `values`;
- decide whether the final `println!` compiles;
- explain why the fact that `i32` is `Copy` does or does not decide what happens to the
  vector binding.

```rust
// Version A
let values = vec![3, 6, 9];
for item in values {
    println!("{item}");
}
println!("{}", values.len());
```

```rust
// Version B
let values = vec![3, 6, 9];
for item in &values {
    println!("{item}");
}
println!("{}", values.len());
```

**Your answer:**

### 4. Diagnose the overlapping borrows

Predict whether this compiles. If it does not, identify the two operations whose borrows
conflict and explain why `Vec::push` matters even though it only appends one element.

```rust
let mut scores = vec![10, 20, 30];
let first = &scores[0];

scores.push(40);
println!("first={first}");
```

Then describe a minimal reordering that preserves all four values and makes the borrows
non-overlapping. Name the point where the shared borrow ends under non-lexical lifetime
analysis.

**Your answer:**

### 5. Write a disjoint-slice transformation

Write the body of this function. It must split `values` at `mid`, add `10` to every value
in the left slice, and add `100` to every value in the right slice.

```rust
fn adjust_halves(values: &mut [i32], mid: usize) {
    // Your implementation
}
```

Requirements:

- use the standard safe operation that returns two disjoint mutable slices;
- mutate through the references instead of merely calculating and discarding results;
- do not clone or allocate another collection;
- explain what the two slices contain when `mid` is `0`, when it equals `values.len()`,
  and what happens when `mid` is greater than `values.len()`.

**Your answer:**

### 6. Transfer: a borrowed command parser

A command-line application receives an owned `String` such as `"deploy production"`.
It needs a function that returns only the first whitespace-separated word without
allocating a new string.

Describe an appropriate function signature using `&str`, including the return type. Then
explain:

- who owns the original text;
- what the returned value contains and whether it owns or copies the word;
- why the returned value cannot be stored and used after the original `String` is dropped;
- what ownership and allocation tradeoff would change if the function returned `String`
  instead.

You do not need to implement the function.

**Your answer:**

## Confidence check

For each question, rate confidence from 1 (guessing) to 5 (certain) before checking notes.
