---
title: Iterator Methods Cheatsheet
tags: [rust, cheatsheet, iterators]
---

# Iterator Methods Cheatsheet

## Creating iterators

| Call | Yields |
|---|---|
| `v.iter()` | `&T` |
| `v.iter_mut()` | `&mut T` |
| `v.into_iter()` | `T` (consumes) |
| `std::iter::once(x)` | single value |
| `std::iter::empty()` | no values |
| `std::iter::repeat(x)` | infinite `x` |
| `0..10`, `0..=10` | range |

## Adapters (lazy — return a new iterator)

| Adapter | Does |
|---|---|
| `map(\|x\| ...)` | Transform each item |
| `filter(\|x\| ...)` | Drop items failing predicate |
| `take(n)` / `skip(n)` | Prefix / drop prefix |
| `take_while` / `skip_while` | Conditional versions |
| `enumerate()` | `(index, value)` pairs |
| `zip(other)` | pair with another iter |
| `chain(other)` | concatenate iters |
| `flat_map(f)` / `flatten()` | expand nested iters |
| `rev()` | reverse (requires `DoubleEndedIterator`) |
| `peekable()` | adds `.peek()` method |
| `cloned()` / `copied()` | `&T` → `T` via Clone / Copy |
| `step_by(n)` | every nth element |
| `windows(n)` / `chunks(n)` | slices (methods on `&[T]`) |
| `scan(init, f)` | fold-like but yields at each step |

## Consumers (run the pipeline)

| Consumer | Yields |
|---|---|
| `collect::<C>()` | any `C: FromIterator` |
| `for_each(f)` | `()` |
| `sum()`, `product()` | `T` |
| `count()` | `usize` |
| `min()`, `max()` | `Option<T>` |
| `min_by_key`, `max_by_key` | ditto |
| `any(p)`, `all(p)` | `bool`, short-circuits |
| `find(p)`, `find_map(f)` | `Option<...>`, short-circuits |
| `position(p)` | `Option<usize>` |
| `fold(init, f)` | accumulated value |
| `reduce(f)` | `Option<T>` |
| `try_fold(init, f)` | short-circuiting fold for `Result`/`Option` |
| `last()`, `nth(i)` | `Option<T>` |
| `unzip()` | `(Vec<A>, Vec<B>)` from iter of `(A,B)` |

## Classic recipes

```rust
// Word count
let mut counts = HashMap::new();
for w in text.split_whitespace() { *counts.entry(w).or_insert(0) += 1; }

// Flatten Vec<Result<T,E>> into Result<Vec<T>,E>
let parsed: Result<Vec<_>, _> = strings.iter().map(|s| s.parse::<i32>()).collect();

// Max by field
let oldest = users.iter().max_by_key(|u| u.age);

// Deduplicate while preserving order
let mut seen = HashSet::new();
let deduped: Vec<_> = items.into_iter().filter(|i| seen.insert(i.clone())).collect();

// Group-by-ish with fold
let groups = items.iter().fold(HashMap::<_, Vec<_>>::new(), |mut acc, x| {
    acc.entry(x.key()).or_default().push(x);
    acc
});
```
