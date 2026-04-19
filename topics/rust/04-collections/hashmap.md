---
title: 4.3 HashMap<K, V>
tags: [rust, hashmap, collections]
---

# 4.3 `HashMap<K, V>`

## Creating

```rust
use std::collections::HashMap;

let mut m: HashMap<String, i32> = HashMap::new();
m.insert("alice".into(), 90);

let scores: HashMap<&str, i32> = [("alice", 90), ("bob", 85)].into();
```

## Reading

```rust
let a = m.get("alice");          // Option<&i32>
let a = m["alice"];              // panics if missing — avoid
let exists = m.contains_key("alice");
```

## Updating patterns

```rust
// Overwrite
m.insert("alice".into(), 95);

// Insert if absent
m.entry("charlie".into()).or_insert(0);

// Update in place
*m.entry("alice".into()).or_insert(0) += 1;   // classic counter
```

The `entry` API is one of the most useful parts of the stdlib — learn it cold.

## Iterating

```rust
for (k, v) in &m           { /* &K, &V */ }
for (k, v) in &mut m       { /* &K, &mut V */ }
for (k, v) in m            { /* K, V — consumes */ }
```

Iteration order is **unspecified** (randomized per program run, for DoS resistance).

## Keys and values

- Key type must implement `Eq + Hash`.
- Value type has no constraints (but is cloned/moved as usual).

Custom structs: `#[derive(Eq, Hash, PartialEq)]`.

## Performance notes

- Default hasher is SipHash — secure but slower than libraries need. For non-adversarial inputs swap to `ahash` or `rustc_hash::FxHashMap`.
- Use `HashMap::with_capacity(n)` when you know the size up front.
- For small maps (<32 entries) a `Vec<(K, V)>` with linear search is often faster.

## When to pick which map

| Need | Use |
|---|---|
| Fast unordered lookup | `HashMap` |
| Sorted keys, range queries | `BTreeMap` |
| Tiny maps, few entries | `Vec<(K, V)>` |
| Fixed, known-at-compile-time keys | Plain struct with named fields |

## Related
- [[vec|Vec]]
- [[../cheatsheets/iterator-methods|Iterator methods]]
