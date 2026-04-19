---
title: 8.2 Iterators
tags: [rust, iterators]
---

# 8.2 Iterators

## The trait

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // … dozens of provided methods
}
```

All you need to implement an iterator is `next()`. Everything else (`map`, `filter`, `collect`, …) is a default method built on top.

## Consuming vs lazy

Iterators are **lazy** — adapters like `map`/`filter` build up a *description*. Nothing happens until a **consumer** pulls values out.

| Lazy adapters | Consumers |
|---|---|
| `map`, `filter`, `take`, `skip`, `enumerate` | `collect`, `for_each`, `sum`, `product` |
| `zip`, `chain`, `flat_map`, `flatten`, `peekable` | `count`, `min`, `max`, `fold`, `reduce` |
| `rev`, `cloned`, `copied`, `cycle`, `step_by` | `any`, `all`, `find`, `position`, `nth`, `last` |

## Classic pipeline

```rust
let total: i32 = (1..=100)
    .filter(|n| n % 2 == 0)
    .map(|n| n * n)
    .sum();
```

Every element flows through the chain one at a time — no intermediate `Vec`s allocated.

## `collect` — the opposite of `iter`

```rust
let v: Vec<i32>       = (0..5).collect();
let s: String         = ['h','i'].iter().collect();
let m: HashMap<_, _>  = vec![(1,"a"), (2,"b")].into_iter().collect();

// Turns Vec<Result<T,E>> into Result<Vec<T>, E> — super handy
let res: Result<Vec<i32>, _> = lines.iter().map(|s| s.parse::<i32>()).collect();
```

`collect` is generic over a `FromIterator` target — turbofish when inference can't pick.

## Three ways to iterate a collection

```rust
for x in &v       { /* &T   — iter()      */ }
for x in &mut v   { /* &mut T — iter_mut()*/ }
for x in v        { /* T    — into_iter() — consumes */ }
```

## Writing your own iterator

```rust
struct Counter { n: u32, max: u32 }

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.n < self.max { self.n += 1; Some(self.n) } else { None }
    }
}

let sum: u32 = Counter { n: 0, max: 5 }.sum();   // 15
```

Free: `map`, `filter`, `sum`, … — all from the trait default impls.

## `fold` / `reduce`

```rust
let product = (1..=10).fold(1, |acc, n| acc * n);   // seed = 1
let total   = (1..=10).reduce(|acc, n| acc + n);    // Option<i32>
```

`fold` carries an accumulator of any type; `reduce` folds with the element type and returns `None` on empty.

## `zip`, `chain`, `flat_map`

```rust
let names = vec!["a", "b"];
let ages  = vec![1, 2];
for (n, a) in names.iter().zip(ages.iter()) { /* ... */ }

let merged: Vec<_> = (0..3).chain(10..13).collect();        // 0,1,2,10,11,12
let chars: Vec<char> = ["hi", "lo"].iter().flat_map(|s| s.chars()).collect();
```

## `enumerate`, `peekable`, `windows`/`chunks`

```rust
for (i, x) in v.iter().enumerate() { /* ... */ }

let mut it = (0..).peekable();
if let Some(&next) = it.peek() { /* ... */ }

for w in v.windows(3) { /* &[T] of size 3 */ }
for c in v.chunks(2)  { /* &[T] of size 2 (last may be smaller) */ }
```

## The `?` of iterators: `try_fold` / `try_for_each`

```rust
let total: Result<i32, _> = lines.iter().try_fold(0, |acc, s| {
    let n: i32 = s.parse()?;
    Ok(acc + n)
});
```

Short-circuits on the first error — idiomatic.

## Related
- [[closures|Closures]]
- [[../cheatsheets/iterator-methods|Iterator methods cheatsheet]]
