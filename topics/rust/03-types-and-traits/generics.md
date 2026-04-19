---
title: 3.4 Generics
tags: [rust, generics]
---

# 3.4 Generics

## Functions

```rust
fn largest<T: PartialOrd>(xs: &[T]) -> &T {
    let mut best = &xs[0];
    for x in &xs[1..] {
        if x > best { best = x; }
    }
    best
}
```

`T: PartialOrd` is a **trait bound** — "T must support `>`". Without it the compiler wouldn't let you use `>`. See [[traits|Traits]].

## Structs & enums

```rust
struct Pair<T, U> { first: T, second: U }

enum Either<L, R> { Left(L), Right(R) }
```

## Impl blocks

```rust
impl<T> Pair<T, T> {                              // only when both types match
    fn swap(self) -> Self { Pair { first: self.second, second: self.first } }
}

impl<T: std::fmt::Debug> Pair<T, T> {             // bounded impl
    fn print(&self) { println!("{:?} {:?}", self.first, self.second); }
}
```

## `where` clauses for readability

```rust
fn some_fn<T, U>(a: T, b: U) -> i32
where
    T: std::fmt::Debug + Clone,
    U: Iterator<Item = T>,
{
    42
}
```

## Monomorphization

Rust generates a specialized compiled version of a generic function/type **for each concrete type you use it with**. The result: generics cost nothing at runtime — as fast as hand-written code. The tradeoff: bigger binaries if you instantiate widely.

## Turbofish

When inference can't figure out the type parameter, use `::<>`:

```rust
let parsed = "42".parse::<i32>().unwrap();
let v = (0..10).collect::<Vec<_>>();
```

## Defaults for type parameters

```rust
struct Counter<T = u32> { n: T }
let c: Counter = Counter { n: 0 };              // uses u32
```

## `impl Trait` in positions

```rust
fn make_iter() -> impl Iterator<Item = i32> { 0..10 }      // return position
fn accept(x: impl std::fmt::Debug) { println!("{x:?}"); }  // arg position
```

- **Argument position**: sugar for `fn accept<T: Debug>(x: T)`.
- **Return position**: "some concrete type that implements Iterator — you don't need to name it." Cleaner than spelling out an unnameable iterator adapter chain.

## Const generics

```rust
fn take_array<const N: usize>(a: [i32; N]) -> usize { N }
```

Useful for fixed-size buffer APIs.

## Related
- [[traits|Traits — the bounds you apply to generics]]
- [[trait-objects|Trait objects — dynamic dispatch]]
