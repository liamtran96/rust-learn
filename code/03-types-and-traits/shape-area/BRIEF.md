# Shape area - Brief

> You write the implementation yourself. This brief only explains the task and unfamiliar syntax.
> When it compiles, runs, and Liam says "done," use `$journal` to log it.

## What you are building

Model three possible shapes as one `Shape` enum: a circle with a radius, a rectangle
with width and height, and a triangle with three side lengths. Add an `area` method that
uses `match` to select the calculation for the current variant.

This is the same design used in applications where one value may be one of several
known cases, and each case carries exactly the data it needs.

## Expected input and output

The input is a `Shape` value and the output is the `f64` returned by its `area` method.
This is not a stdin/stdout exercise; `main` may print example results, but their display
format is intentionally unspecified.

Concrete examples:

```text
Circle with radius 2.0       -> area approximately 12.566
Rectangle 3.0 by 4.0        -> area 12.0
```

The specification does not define how invalid triangle side lengths should be handled,
so begin with valid triangles only.

## Required Rust syntax

```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { w: f64, h: f64 },
    Triangle { a: f64, b: f64, c: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        todo!()
    }
}
```

- `enum Shape` declares one type whose value is exactly one listed variant.
- `Circle`, `Rectangle`, and `Triangle` are variant names, reached as
  `Shape::Circle`, `Shape::Rectangle`, and `Shape::Triangle`.
- `{ radius: f64 }` is a struct-style payload: this variant stores a named `radius`
  field whose value is a 64-bit floating-point number.
- `impl Shape` opens a block for functions and methods associated with `Shape`.
- `fn area` declares a method named `area`.
- `&self` shared-borrows the current `Shape`, so calculating its area does not consume it.
- `-> f64` promises that every completed path returns a floating-point number.
- `todo!()` is a temporary placeholder that compiles but panics if execution reaches it.
- `match` inspects the enum variant, binds that variant's fields, and must cover every
  possible `Shape` variant.

## Your coding steps

1. Type the `Shape` enum, create one shape in `main`, and make `cargo run` print a simple
   scaffold message before calling `area`.
2. Add the `impl Shape` block and a compiling `area` stub, then write the exhaustive
   `match` and replace each placeholder calculation one variant at a time.
3. Demonstrate all three variants and add focused tests for their area calculations.

## Concepts in play

- Enums whose variants carry different named data
- Methods that inspect a value through `&self`
- Exhaustive pattern matching that produces a value

## Watch out for

Avoid a catch-all `_` arm: naming every variant lets the compiler flag every place that
needs updating if another shape is added later.

## References (read only if stuck)

- Chapter: `topics/rust/03-types-and-traits/index.md`
- Specific notes: `topics/rust/03-types-and-traits/enums.md`, `topics/rust/03-types-and-traits/pattern-matching.md`

## Checklist

- [x] I can explain each part of the required syntax
- [x] `cargo run` compiles and prints a stub
- [x] Implement the spec
- [x] `cargo clippy -- -D warnings` is clean
- [x] `cargo fmt` applied
- [x] Tests pass when the exercise requires them
- [x] Tell Codex "done" so `$journal` logs the session

## Run

```text
cd code/03-types-and-traits/shape-area
cargo run
```
