# Rust Retrieval Homework - 2026-08-18

**Focus:** expressions and unit, loop values and bindings, shadowing and numeric types
**Based on:** completed material through Phase 1 / Week 1
**Timebox:** 25-35 minutes

## Instructions

Answer from memory first. For code questions, predict before running anything. Explain
the Rust rule behind each answer. Write answers directly below each prompt.

## Questions

### 1. What leaves a block?

In your own words, explain the difference between an expression and a statement in Rust.
What effect can a trailing semicolon have on the value of a block, and what is `()`? Give
one tiny example of a block that produces an integer and one that produces `()`.

**Your answer:**

### 2. Same name, different operation

Explain how shadowing differs from assignment. Include the roles of `let`, `mut`, and
scope. Then explain why a type annotation does not convert an `i32` into an `i64` for an
arithmetic operation.

**Your answer:**

### 3. Diagnose the returned type

Without compiling, decide whether this function is valid. State what type each branch
produces, what type the complete `if` expression produces, and what rule causes any
compiler error. Describe the smallest conceptual correction; do not merely quote an
expected compiler message.

```rust
fn direction(change: i32) -> String {
    if change >= 0 {
        "up".to_string();
    } else {
        "down".to_string();
    }
}
```

**Your answer:**

### 4. A binding that does not exist yet

Without running this code, determine whether it compiles. Trace the order in which Rust
would need to evaluate the `let` statement, identify the precise scoping problem, and
describe how the variables should be structured so the loop can produce its final value.

```rust
fn digit_count(mut number: u32) -> u32 {
    let total = loop {
        total += 1;
        number /= 10;

        if number == 0 {
            break total;
        }
    };

    total
}
```

**Your answer:**

### 5. Write a value-producing function

Write only the function `shipping_band(weight: u32) -> &'static str`. It must produce
`"light"` for weights below 5, `"standard"` for weights from 5 through 20, and `"heavy"`
for larger weights. Use one connected conditional expression as the function's final
value. Do not use `return` and do not allocate a `String`. Briefly explain why your code's
final expression has the declared return type.

**Your answer:**

### 6. Transfer: safe command-line input

A new CLI program expects a quantity as its first user argument, such as:

```text
cargo run -- 12
```

Describe a safe flow that collects the arguments, avoids indexing before checking the
length, borrows the first user argument, and parses it as `u32` using `Result` and `match`.
For each step, explain what failure it prevents or handles. You may use short Rust fragments,
but a complete program is not required.

**Your answer:**

## Confidence check

For each question, rate confidence from 1 (guessing) to 5 (certain) before checking notes.
