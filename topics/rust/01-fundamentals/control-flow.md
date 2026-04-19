---
title: 1.5 Control Flow
tags: [rust, fundamentals, control-flow]
---

# 1.5 Control Flow

## `if` — no parentheses, conditions must be `bool`

```rust
if temperature > 30 {
    println!("hot");
} else if temperature < 10 {
    println!("cold");
} else {
    println!("mild");
}
```

- The condition **must** be `bool` — no `if 1 { ... }` truthiness.
- `if`/`else` is an expression, so: `let n = if cond { 5 } else { 6 };`

## `loop` — infinite loop with labeled break

```rust
let result = loop {
    let x = try_get();
    if let Some(v) = x { break v; }   // break returns a value
};
```

Labels disambiguate nested loops:
```rust
'outer: for i in 0..10 {
    for j in 0..10 {
        if i * j > 50 { break 'outer; }
    }
}
```

## `while`

```rust
while x > 0 { x -= 1; }
```

`while let` consumes `Some`s until `None`:
```rust
while let Some(top) = stack.pop() { println!("{top}"); }
```

## `for` — use it almost always

```rust
for i in 0..10 { /* 0,1,...,9 */ }
for i in 0..=10 { /* 0,1,...,10 */ }
for item in &vec { /* borrow */ }
for item in &mut vec { /* borrow mut */ }
for item in vec { /* move, consumes vec */ }
```

`for` under the hood calls `.into_iter()` on the expression — see [[../08-closures-iterators/iterators|Iterators]].

## Ranges

| Syntax | Meaning |
|---|---|
| `start..end` | half-open — common |
| `start..=end` | inclusive |
| `..end`, `start..`, `..` | partial — useful for slicing |

## No fallthrough in `match`

Unlike C/Go/JS, `match` arms don't fall through. Each arm stands alone.

## Early return patterns

```rust
// Typical Rust style — prefer `?` and `let else` over deep nesting
let user = match lookup(id) {
    Some(u) => u,
    None    => return Err(Error::NotFound),
};

// Same thing with `let else` (1.65+)
let Some(user) = lookup(id) else {
    return Err(Error::NotFound);
};
```

## Exit criteria
- [ ] You can explain why `if` is an expression and write `let n = if …`.
- [ ] You can use `break value` from a `loop` to return a computed result.
- [ ] You can use `let else` to flatten early-return code.
- [ ] You default to `for item in &collection` and reach for `while` only for condition-driven loops.
