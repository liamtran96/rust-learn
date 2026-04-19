---
title: 5.1 Result & Option
tags: [rust, result, option, errors]
---

# 5.1 `Result<T, E>` & `Option<T>`

## The types

```rust
enum Option<T> {
    None,
    Some(T),
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## Matching — always works

```rust
match divide(10, 0) {
    Ok(q)  => println!("{q}"),
    Err(e) => eprintln!("error: {e}"),
}
```

## Combinators — preferred when possible

Both types share a big family of helper methods. Master these — `match` is cleaner when logic is complex, combinators are cleaner when it's linear.

### Option-flavored

| Method | Meaning |
|---|---|
| `unwrap()` | Some → inner; None → panic |
| `expect("msg")` | Like unwrap with a message |
| `unwrap_or(default)` | None → default |
| `unwrap_or_else(\|\| …)` | None → lazy default |
| `unwrap_or_default()` | None → `T::default()` |
| `map(\|v\| …)` | `Some(v)` → `Some(f(v))` |
| `and_then(\|v\| …)` | Chain another `Option`-returning op |
| `or(other)` | None → other |
| `or_else(\|\| …)` | None → lazy alternative |
| `ok_or(err)` | Option → Result |
| `filter(\|v\| …)` | Drop Some that fails predicate |
| `is_some()`, `is_none()` | Boolean checks |
| `as_ref()` | `&Option<T>` → `Option<&T>` |

### Result-flavored

| Method | Meaning |
|---|---|
| `ok()`, `err()` | Convert to `Option` |
| `map(\|v\| …)` | Transform `Ok` |
| `map_err(\|e\| …)` | Transform `Err` |
| `and_then(\|v\| …)` | Chain a `Result`-returning op (aka flatMap) |
| `or_else(\|e\| …)` | Recover |
| `unwrap()`, `expect(...)` | Panic on `Err` |
| `unwrap_or(v)` / `unwrap_or_else(\|e\| …)` | Default on `Err` |
| `?` | Propagate on `Err` (see [[question-mark|next note]]) |

## `if let` and `let else` sugar

```rust
if let Some(x) = maybe { use_x(x); }

let Ok(v) = parse_input() else { return; };
```

## Converting between them

```rust
let opt: Option<i32> = Some(3);
let res: Result<i32, &str> = opt.ok_or("missing");

let res: Result<i32, _> = "42".parse();
let opt: Option<i32> = res.ok();
```

## Anti-patterns to avoid

- **`.unwrap()` in production paths** — prefer `?` or explicit handling.
- **Panicking on user input** — that's a recoverable error.
- **Nesting `match`** 3 levels deep — use combinators or the `?` operator.
- **Ignoring `Result`** — the `#[must_use]` lint catches you.

## Related
- [[question-mark|The ? operator]]
- [[custom-errors|Custom error types]]
