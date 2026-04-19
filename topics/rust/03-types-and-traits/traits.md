---
title: 3.5 Traits
tags: [rust, traits]
---

# 3.5 Traits

Traits are Rust's way to say "a type *behaves like* this." Think Haskell typeclasses or Java interfaces, more ergonomic than both.

## Defining a trait

```rust
trait Summary {
    fn summarize(&self) -> String;

    fn announce(&self) -> String {
        format!("Breaking news! {}", self.summarize())     // default method
    }
}
```

## Implementing

```rust
struct Tweet { user: String, body: String }

impl Summary for Tweet {
    fn summarize(&self) -> String { format!("@{}: {}", self.user, self.body) }
}
```

## Using traits as bounds

```rust
fn notify<T: Summary>(item: &T) {
    println!("{}", item.announce());
}

// Syntactic sugar — same thing:
fn notify2(item: &impl Summary) { println!("{}", item.announce()); }
```

## Multiple bounds

```rust
fn f<T: Summary + Clone>(x: T) { /* ... */ }

fn g<T>(x: T)
where T: Summary + Clone + std::fmt::Debug { /* ... */ }
```

## The orphan rule

You can implement **your trait** for **any type**, and **any trait** for **your type** — but not someone else's trait on someone else's type. Prevents incoherent impls across crates.

Workarounds for external-trait-on-external-type: **newtype** the type.

## Derive macros

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Config { retries: u32 }
```

Derivable stdlib traits include: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, `PartialOrd`, `Ord`, `Default`.

## Associated types

```rust
trait Iterator {
    type Item;                                           // associated type
    fn next(&mut self) -> Option<Self::Item>;
}
```

Use associated types when there's exactly **one** natural answer per implementation (the Item of a `Vec<T>`-iterator is `T`). Use generics when multiple parameterizations make sense.

## Supertraits

```rust
trait Printable: std::fmt::Debug {        // implementors must also be Debug
    fn print(&self) { println!("{:?}", self); }
}
```

## Blanket impls

```rust
impl<T: std::fmt::Display> ToString for T { /* ... */ }
```

"Every `Display` type also implements `ToString`." The stdlib is full of these.

## Traits you'll touch often
- `Debug`, `Display` — printing
- `Clone`, `Copy` — duplication
- `PartialEq`, `Eq`, `Hash`, `Ord` — equality and sorting
- `Default` — zero-arg constructors
- `From`/`Into`, `TryFrom`/`TryInto` — conversions
- `Iterator` — the big one
- `Deref`, `Drop` — smart pointers & RAII
- `Send`, `Sync` — concurrency markers
- `Fn`, `FnMut`, `FnOnce` — callables

## Related
- [[trait-objects|Trait objects — runtime polymorphism]]
- [[generics|Generics]]
- [[../08-closures-iterators/iterators|Iterator — the most important trait]]
