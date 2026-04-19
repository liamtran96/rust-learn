---
title: 3.6 Trait Objects (dyn Trait)
tags: [rust, traits, dyn, polymorphism]
---

# 3.6 Trait Objects (`dyn Trait`)

Generics give you **static** dispatch — resolved at compile time, zero runtime cost, but the concrete type must be known. Sometimes you want **dynamic** dispatch — a heterogeneous collection of "anything that implements this trait".

## The syntax

```rust
trait Draw { fn draw(&self); }

struct Button; struct Slider;
impl Draw for Button { fn draw(&self) { /* ... */ } }
impl Draw for Slider { fn draw(&self) { /* ... */ } }

let widgets: Vec<Box<dyn Draw>> = vec![
    Box::new(Button),
    Box::new(Slider),
];

for w in &widgets {
    w.draw();                         // vtable lookup at runtime
}
```

A `dyn Trait` is a **fat pointer**: `(data ptr, vtable ptr)`. Calls are virtual, like C++'s virtual methods.

## Static vs dynamic — when to pick which

| Use static (`impl Trait`, `T: Trait`) when | Use dynamic (`dyn Trait`) when |
|---|---|
| You want max perf, no indirection | You need a heterogeneous collection |
| You know all types at compile time | Plugin/registry patterns |
| Single concrete return type | Runtime-polymorphic return |
| Bounds stay simple | Code size matters more than a few cycles |

## Object safety

Not every trait can be used as `dyn Trait`. Rough rule — a trait is object-safe if:
- It has no generic methods (only associated-type or concrete types).
- It has no methods returning `Self` by value.
- It has no `Self: Sized` requirements (except on default-bodied methods that opt out).

`Clone` is **not** object-safe (it returns `Self`). `Iterator` *is* object-safe.

## Common containers

```rust
Box<dyn Error>        // owned, heap-allocated
&dyn Trait            // borrowed trait reference
Arc<dyn Trait>        // shared, thread-safe
```

## `Box<dyn Error>` for quick-and-dirty error handling

```rust
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let s = std::fs::read_to_string("in.txt")?;
    let n: i32 = s.trim().parse()?;
    println!("{n}");
    Ok(())
}
```

Handy for `main` and prototypes. For libraries, define a concrete error enum instead (see [[../05-error-handling/custom-errors|custom errors]]).

## Related
- [[traits|Traits]]
- [[generics|Generics — the static side]]
- [[../09-smart-pointers/box|Box — pairing with dyn]]
