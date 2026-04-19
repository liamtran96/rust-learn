---
title: 4. Collections & Strings
tags: [rust, collections, strings]
---

# 4. Collections & Strings

The data structures you'll use in 90% of Rust code.

## Contents
- [[strings|4.1 String & &str]]
- [[vec|4.2 Vec<T>]]
- [[hashmap|4.3 HashMap<K, V>]]

## Cheat-table

| You want | Type |
|---|---|
| Growable UTF-8 text | `String` |
| Borrowed text view | `&str` |
| Growable array | `Vec<T>` |
| Fixed-size array | `[T; N]` |
| Borrowed sequence view | `&[T]` |
| Key→value map | `HashMap<K, V>` (unordered) |
| Sorted key→value map | `BTreeMap<K, V>` |
| Unique unordered set | `HashSet<T>` |
| Unique sorted set | `BTreeSet<T>` |
| Double-ended queue | `VecDeque<T>` |
| Linked list (rarely useful) | `LinkedList<T>` |

## Exit criteria
- [ ] You never reach for `String` when `&str` works.
- [ ] You can iterate a `Vec` three ways: `iter`, `iter_mut`, `into_iter`.
- [ ] You can count word frequencies with a `HashMap`.
- [ ] You understand why `s.chars()` is often what you want instead of `s.bytes()`.
