---
title: 4.2 Vec<T>
tags: [rust, vec, collections]
---

# 4.2 `Vec<T>`

The workhorse: a growable, heap-allocated, contiguous array. Think `std::vector` / `ArrayList` / `list`.

## Creating

```rust
let v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3];
let v = vec![0; 10];                         // ten zeros
let v: Vec<i32> = (0..10).collect();
let v = Vec::with_capacity(1_000);           // pre-allocate — avoid reallocations
```

## Adding & removing

```rust
let mut v = vec![1, 2, 3];
v.push(4);
let last = v.pop();                          // Option<i32>
v.insert(0, 0);
v.remove(0);
v.swap_remove(0);                            // O(1) — doesn't preserve order
v.extend([5, 6, 7]);
v.clear();
```

## Accessing

```rust
let v = vec![10, 20, 30];
let a = v[0];                    // panics on out-of-bounds
let b = v.get(5);                // Option<&i32> — no panic
let last = v.last();             // Option<&i32>
```

Prefer `.get()` when the index might be invalid.

## Iterating — three ways

```rust
let v = vec![1, 2, 3];
for x in &v        { /* &i32  — borrow */ }
for x in &mut v    { /* &mut i32 */ }
for x in v         { /* i32   — consumes v */ }
```

Same as calling `.iter()`, `.iter_mut()`, `.into_iter()`.

## Transforming with iterators

```rust
let doubled: Vec<i32> = (1..=5).map(|x| x * 2).collect();
let evens:   Vec<i32> = v.iter().copied().filter(|n| n % 2 == 0).collect();
let sum:     i32      = v.iter().sum();
```

See [[../08-closures-iterators/iterators|Iterators]].

## Slicing

```rust
let v = vec![1, 2, 3, 4, 5];
let s: &[i32] = &v[1..4];        // [2, 3, 4]
```

## Sorting

```rust
let mut v = vec![3, 1, 2];
v.sort();                        // requires Ord
v.sort_by(|a, b| b.cmp(a));      // custom, descending
v.sort_by_key(|&n| -n);          // by key
```

## Capacity vs length

- `len()` — number of elements.
- `capacity()` — allocated slots.

Pushes amortize to O(1); occasional reallocation doubles capacity. If you know the size ahead, use `with_capacity` or `reserve(n)`.

## Common pitfalls

- **Holding a reference while mutating**: `let first = &v[0]; v.push(4);` — ❌. The push may reallocate, invalidating `first`.
- **`swap_remove` vs `remove`**: use `swap_remove` for O(1) when order doesn't matter.
- **`Vec<u8>` is not a string**: use `Vec<u8>` for bytes, `String` for text. Convert with `String::from_utf8`.

## Related
- [[strings|String]]
- [[hashmap|HashMap]]
- [[../08-closures-iterators/iterators|Iterators]]
