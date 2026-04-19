---
title: 2.3 Slices
tags: [rust, slices, references]
---

# 2.3 Slices

A **slice** is a borrowed view into a contiguous sequence. Two common forms:

| Type | Backing | Notes |
|---|---|---|
| `&[T]` | array, `Vec<T>`, another slice | Generic over element type |
| `&str` | `String` literal, `String`, another `&str` | UTF-8 **bytes**, not chars |

## Creating slices

```rust
let a = [10, 20, 30, 40, 50];
let s = &a[1..4];          // &[i32] of length 3

let v = vec![1, 2, 3, 4];
let s = &v[..];            // whole-Vec slice

let name = String::from("Alice");
let prefix: &str = &name[..3];
```

## Why slices matter

A slice is a **fat pointer**: `(pointer, length)`. It carries its own bounds information, which is why slicing is bounds-checked. It also means a function taking `&[T]` works with arrays, `Vec`s, and sub-ranges — one signature, many callers.

```rust
fn sum(xs: &[i32]) -> i32 { xs.iter().sum() }

sum(&[1, 2, 3]);
sum(&vec![1, 2, 3]);
sum(&vec![1,2,3,4][1..3]);
```

**Rule of thumb:** prefer `&[T]` over `&Vec<T>` as a parameter, and `&str` over `&String`. More flexible, no cost.

## String slicing gotcha

`&str` indexing is by **byte offset**, not character. For non-ASCII text this can panic:

```rust
let s = "café";
&s[0..3]    // "caf"
&s[0..4]    // ❌ panic: byte 4 is a UTF-8 boundary? actually: 'é' is 2 bytes
```

Use `.chars()` or `.char_indices()` for character iteration. See [[../04-collections/strings|Strings]].

## Returning a slice — the lifetime teaser

```rust
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
```

The returned `&str` **borrows from** `s`. The compiler enforces that the returned slice can't outlive `s`. This is where [[lifetimes|lifetimes]] become explicit.

## Related
- [[borrowing|Borrowing]]
- [[lifetimes|Lifetimes]]
- [[../04-collections/strings|Strings]]
