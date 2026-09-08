---
title: Ch 2 — Ownership Exercises
tags: [rust, exercises, ownership]
---

# Ch 2 — Ownership Exercises

## Predict & fix
For each snippet: predict whether it compiles; if not, explain why and make the minimal fix.

> **These snippets are now runnable drills** at `code/02-ownership/drills-ownership/` (plus eight more). Do them there — the compiler grades you. Start with `cargo test --test d01_move` and see the crate's `BRIEF.md`. The snippets below stay as reference.

```rust
// A
let s = String::from("hi");
let t = s;
println!("{s} {t}");
```

```rust
// B
let mut v = vec![1, 2, 3];
let r = &v[0];
v.push(4);
println!("{r}");
```

```rust
// C
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

```rust
// D
fn first_word(s: &String) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
```

## Implement
1. Write `fn strip_margin(s: &str, prefix: char) -> String` that trims each line's leading whitespace up to (and including) the first `prefix`.
2. Write `fn split_at_mut<T>(v: &mut [T], mid: usize) -> (&mut [T], &mut [T])` — classic exercise in exclusive borrows. **Hint:** the safe version uses `slice::split_at_mut` — try to implement it from scratch, understand why the straightforward version fails, then read the stdlib source.

## Lifetime reasoning
3. Design a struct `Scanner<'a>` that holds a `&'a str` and a cursor position, with methods `peek(&self) -> Option<char>` and `advance(&mut self) -> Option<char>`. Write three tests.

4. Refactor `Scanner` to own its source (`String`). Which feels better and why?

5. Re-implement the consecutive-duplicate removal half of `Vec::dedup` by hand.
   Write `fn dedup_in_place<T: PartialEq>(values: &mut Vec<T>)` so repeated
   neighboring values collapse to one value while the original vector is
   modified in place. Keep non-adjacent duplicates; for example,
   `[1, 1, 2, 1, 2, 2]` becomes `[1, 2, 1, 2]`.

## Checkpoint
You're done when you can read any of these errors and immediately know the fix:
- `cannot borrow 'v' as mutable, as it is also borrowed as immutable`
- `borrow of moved value: 's'`
- `lifetime may not live long enough`
- `'x' does not live long enough`
