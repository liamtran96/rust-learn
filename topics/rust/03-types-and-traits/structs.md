---
title: 3.1 Structs
tags: [rust, structs]
---

# 3.1 Structs

## Three flavors

```rust
// 1. Named-field struct (most common)
struct User {
    id: u64,
    email: String,
    active: bool,
}

// 2. Tuple struct — like a named tuple
struct Pair(i32, i32);
struct Meters(f64);        // newtype pattern (see below)

// 3. Unit struct — zero-sized
struct Always;
```

## Construction & update syntax

```rust
let u1 = User { id: 1, email: "a@b.com".into(), active: true };

let u2 = User {
    email: "c@d.com".into(),
    ..u1                       // copy remaining fields from u1
};
```

If a field and a variable share a name, you can use **field init shorthand**:

```rust
fn build(id: u64, email: String) -> User {
    User { id, email, active: true }
}
```

## Methods & associated functions

```rust
impl User {
    fn new(email: String) -> Self {        // associated fn — no self
        Self { id: 0, email, active: true }
    }
    fn deactivate(&mut self) { self.active = false; }  // method — &mut self
    fn domain(&self) -> &str {             // method — &self
        self.email.split('@').nth(1).unwrap_or("")
    }
}
```

Receiver options:
- `self` — takes ownership (consuming method — e.g., builder `.finish()`).
- `&self` — read-only method.
- `&mut self` — mutating method.

## Deriving common traits

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct User { id: u64, email: String }
```

Pull behavior for free: `Debug` enables `{:?}`, `Clone` enables `.clone()`, etc.

## The newtype pattern

Wrap an existing type to get a distinct one:

```rust
struct UserId(u64);
struct OrderId(u64);

fn fetch_user(id: UserId) { /* ... */ }
```

Now the compiler won't let you pass an `OrderId` where a `UserId` is expected. **Zero runtime cost.**

## Visibility

By default, everything is **private to the module**. Use `pub` to expose:

```rust
pub struct User {
    pub id: u64,
    email: String,       // still private
}
```

Private fields + public constructors = **invariants enforced at the type level**.

## Related
- [[enums|Enums]]
- [[traits|Traits — behavior across types]]
- [[../06-modules/modules-and-paths|Modules & visibility]]
