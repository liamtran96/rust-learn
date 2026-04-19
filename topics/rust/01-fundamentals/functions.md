---
title: 1.4 Functions, Expressions, Statements
tags: [rust, fundamentals, functions]
---

# 1.4 Functions, Expressions, Statements

## Anatomy

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // no semicolon → this is the return expression
}
```

- Parameter types are **required**.
- Return type is **required** unless the function returns `()`.
- The final expression (no semicolon) is the return value.

## Expressions vs statements

- **Statements** — perform an action, do not return a value. End with `;`.
- **Expressions** — evaluate to a value. No trailing `;`.

```rust
let y = {           // block IS an expression
    let x = 3;
    x + 1           // ← no semicolon; this is the block's value
};                  // y == 4
```

Adding a semicolon turns any expression into a statement, which yields `()`. This is the #1 source of confusing compiler errors for beginners:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b;   // ❌ returns () instead of i32 — mismatched types
}
```

## `return` keyword

Use `return` for *early* exits; prefer implicit returns at the end.

```rust
fn clamp(x: i32, min: i32, max: i32) -> i32 {
    if x < min { return min; }
    if x > max { return max; }
    x
}
```

## `if` is an expression

```rust
let n = if condition { 5 } else { 6 };
```

Both branches must have the **same type**.

## `match` is an expression

```rust
let description = match code {
    200..=299 => "success",
    400..=499 => "client error",
    500..=599 => "server error",
    _         => "unknown",
};
```

See [[../03-types-and-traits/pattern-matching|Pattern matching]].

## Function pointers & closures (preview)

```rust
fn apply(f: fn(i32) -> i32, x: i32) -> i32 { f(x) }
```

For generic callables, use traits — see [[../08-closures-iterators/closures|Closures]].

## Associated functions & methods

Defined on a type via `impl`:

```rust
struct Point { x: f64, y: f64 }

impl Point {
    fn new(x: f64, y: f64) -> Self { Self { x, y } }     // associated fn (no self)
    fn magnitude(&self) -> f64 { (self.x.powi(2) + self.y.powi(2)).sqrt() }  // method
}
```

Calling:
```rust
let p = Point::new(3.0, 4.0);   // :: for associated
let m = p.magnitude();          // .  for methods
```

See [[../03-types-and-traits/structs|Structs]].

## Related
- [[control-flow|Control flow]]
- [[../03-types-and-traits/traits|Traits — methods that work across types]]
