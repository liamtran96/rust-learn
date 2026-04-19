---
title: 4.1 String & &str
tags: [rust, strings]
---

# 4.1 `String` and `&str`

## The two string types

| | `String` | `&str` |
|---|---|---|
| Ownership | Owned | Borrowed |
| Mutability | Yes (if `mut`) | No |
| Storage | Heap | Anywhere — literal, heap, stack, file |
| When to use | When you need to build/own text | For parameters and slices |

```rust
let literal: &'static str = "hello";   // lives in read-only binary data
let owned: String = String::from(literal);
let borrowed: &str = &owned;
```

## Both are guaranteed UTF-8

Indexing by byte is allowed but **must land on a UTF-8 boundary**, else panic. So you never do `s[0]` — use iterators:

```rust
for c in "café".chars() { print!("{c} "); }
// c a f é
```

## Building strings

```rust
let mut s = String::new();
s.push_str("hello ");
s.push('w');
s += "orld";                       // uses Add<&str>
let n = 42;
let full = format!("{s} {n}");     // allocates a new String
```

## Parsing & conversion

```rust
let n: i32 = "42".parse()?;        // via FromStr
let s: String = 42.to_string();    // via Display
let s: &str = &String::from("hi"); // deref
```

## Slicing is by byte

```rust
let s = "héllo";       // bytes: h(1) é(2) l(1) l(1) o(1)
&s[0..1]               // "h"
&s[1..3]               // "é"
&s[1..2]               // ❌ panic — not a char boundary
```

Prefer `.chars().nth(i)`, `.char_indices()`, or `split_at` with a known boundary.

## Common pitfall: `.len()` is bytes, not chars

```rust
assert_eq!("café".len(), 5);        // 4 chars, 5 bytes
assert_eq!("café".chars().count(), 4);
```

## Useful method map

| Need | Method |
|---|---|
| Length in bytes | `.len()` |
| Is empty | `.is_empty()` |
| To uppercase/lowercase | `.to_uppercase()`, `.to_lowercase()` |
| Trim whitespace | `.trim()`, `.trim_start()`, `.trim_end()` |
| Split | `.split(',')`, `.split_whitespace()`, `.lines()` |
| Contains | `.contains("needle")` |
| Replace | `.replace("a", "b")`, `.replacen(...)` |
| Prefix/suffix | `.starts_with`, `.ends_with`, `.strip_prefix` |
| Iterate chars | `.chars()`, `.char_indices()` |
| Iterate bytes | `.bytes()`, `.as_bytes()` |
| Parse into number | `.parse::<i32>()` |

## Raw strings & special cases

```rust
let path = r"C:\Users\Alice";       // no escape processing
let regex = r#"he said "hi""#;      // choose delimiter count to avoid collision
let b = b"bytes";                   // &[u8; N]
```

## Related
- [[../02-ownership/slices|Slices]]
- [[vec|Vec]]
