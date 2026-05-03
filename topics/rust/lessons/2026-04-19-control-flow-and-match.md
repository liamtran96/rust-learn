---
title: Control flow + match (FizzBuzz)
date: 2026-04-19
chapter: 01-fundamentals
tags: [rust, lesson, control-flow, match, ranges, macros]
related: [[01-fundamentals/control-flow]]
---

# Control flow + match (FizzBuzz)

## TL;DR
`for x in 1..=100` is the only for-loop in Rust; `match` on a tuple of remainders is the cleanest way to branch on multiple conditions; `println!` is a macro (note the `!`) and uses positional `{}` placeholders.

## What it is
Rust has `if`, `loop`, `while`, and `for`, but no C-style `for(i=0; i<n; i++)`. The `for` form iterates over anything implementing `Iterator`, including the range types `1..100` (exclusive) and `1..=100` (inclusive). `match` is an expression — every arm produces a value of the same type, every possible input must be covered, and the compiler enforces both. For divisibility-style branching, matching on a *tuple* of computed values lets each arm read like a truth table instead of a stack of `if`/`else if`.

## Canonical form

```rust
fn main() {
    for i in 1..=100 {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{i}"),
        }
    }
}
```

## Why Rust does it this way
Iterator-based `for` and exhaustive `match` are both expressions of the "make the wrong code fail to compile" philosophy: you can't accidentally fall off the end of a loop counter, you can't accidentally skip a case, and the compiler catches both at build time. The tuple-match pattern leverages exhaustiveness — if you forget the `(_, _)` arm, the compiler refuses to build, which is a much better outcome than a silent fall-through.

## Pitfalls
- **Branch order with `if`/`else if`.** If you write the FizzBuzz check after the single-factor checks, 15 prints `Fizz`. `match` on a tuple sidesteps this entirely — the `(0, 0)` arm matches first.
- **`..` vs `..=`.** `1..100` stops at 99. Use `..=` whenever the upper bound is inclusive.
- **`for x in v` consumes `v`.** Doesn't bite here (range types are `Copy`), but will bite hard in Ch 2 when `v` is a `Vec<T>`. See [[pitfalls]].
- **Forgetting the `!` on `println`.** It's a macro, not a function. `println(...)` is an error, not a typo.

## Self-check (answer without docs)
1. Why does `match (i % 3, i % 5)` not need a `_` fallthrough arm? *(Hint: it has `(_, _)`. What does that pattern match?)*
2. What is the type of `1..=100`, and why can `for` iterate over it?
3. Rewrite the loop body using `if`/`else if` and explain in one line why the order of branches matters.
