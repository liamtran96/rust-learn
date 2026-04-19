---
title: 3.3 Pattern Matching
tags: [rust, pattern-matching, match]
---

# 3.3 Pattern Matching

Patterns appear in `match`, `if let`, `while let`, `let`, `for`, function parameters, and `let else`. Learn them well — they're ubiquitous.

## `match` — exhaustive, value-producing

```rust
let msg = match code {
    200..=299 => "success",
    404       => "not found",
    n if n >= 500 => "server error",  // match guard
    _         => "other",             // mandatory catch-all if non-exhaustive
};
```

## Destructuring

```rust
struct Point { x: i32, y: i32 }

let p = Point { x: 1, y: 2 };
let Point { x, y } = p;          // binds x=1, y=2
let Point { x: a, y: b } = p;    // renaming
let Point { x, .. } = p;         // ignore the rest
```

Enums, tuples, references, arrays all destructure similarly.

## `if let` — single-pattern match

```rust
if let Some(v) = lookup(k) {
    use_it(v);
} else {
    // fallback
}
```

Use when you care about **one** variant and don't want `match`'s boilerplate.

## `while let` — loop until pattern fails

```rust
while let Some(top) = stack.pop() {
    process(top);
}
```

## `let else` — flattens early return

```rust
let Some(user) = db.get(id) else {
    return Err(Error::NotFound);
};
use_user(user);
```

Requires the `else` branch to **diverge** (`return`, `continue`, `break`, `panic!`, `loop`).

## Patterns cheat-table

| Pattern | Matches |
|---|---|
| `1`, `'x'`, `"hi"` | Literal |
| `1..=5`, `'a'..='z'` | Inclusive range |
| `Some(x)`, `Ok(_)` | Enum variant with binding/ignore |
| `Point { x, y }` | Struct |
| `(a, b, _)` | Tuple |
| `[first, .., last]` | Slice/array (with rest) |
| `x @ 1..=5` | Range bind (`x` is the value) |
| `Some(x) \| None` | Multiple alternatives |
| `n if n > 0` | Guard — arbitrary boolean |

## Refutable vs irrefutable

- **Irrefutable** patterns always match — required by `let`, function params, `for`.
- **Refutable** patterns can fail — required by `if let`, `while let`, `match` arms.

```rust
let Some(x) = y;           // ❌ might not match — use `let else` or `if let`
```

## Bindings and `@`

```rust
match n {
    x @ 1..=9 => println!("small number {x}"),
    x         => println!("other {x}"),
}
```

## Exhaustiveness is your friend

Resist the temptation to always add `_ => {}`. Catching *every* variant makes future refactors safer — when a variant is added, the compiler tells you exactly which `match` needs to consider it.

## Related
- [[enums|Enums]]
- [[../05-error-handling/result-option|Result/Option]]
