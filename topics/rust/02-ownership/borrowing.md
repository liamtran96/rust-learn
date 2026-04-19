---
title: 2.2 References & Borrowing
tags: [rust, borrowing, references]
---

# 2.2 References & Borrowing

## Taking a reference vs taking ownership

```rust
fn length(s: &String) -> usize { s.len() }     // borrow (shared)

let s = String::from("hi");
let n = length(&s);                            // & creates a reference
println!("{s} has {n} bytes");                 // ✅ s is still ours
```

## Shared references `&T` — read, share

```rust
let v = vec![1, 2, 3];
let r1 = &v;
let r2 = &v;                   // multiple shared refs are fine
println!("{:?} {:?}", r1, r2);
```

Many readers at once. None of them can mutate.

## Exclusive references `&mut T` — mutate, alone

```rust
let mut v = vec![1, 2, 3];
let r = &mut v;
r.push(4);                     // ✅
// let r2 = &mut v;            // ❌ cannot borrow mut while another mut exists
// let r2 = &v;                // ❌ cannot borrow shared while mut exists
```

The compiler enforces: **one `&mut` XOR many `&`**. Never both, never two `&mut`s.

## Non-lexical lifetimes (NLL)

The borrow is alive for as long as *actually used*, not until the end of scope:

```rust
let mut v = vec![1, 2, 3];
let r = &v;
println!("{:?}", r);           // r's borrow ends here
v.push(4);                     // ✅ despite r being "in scope"
```

This is why many code samples from pre-2018 Rust look overly restrictive.

## Re-borrowing

You can pass along a reference without moving it:

```rust
fn double(x: &mut i32) { *x *= 2; }

let mut n = 5;
let r = &mut n;
double(r);                     // re-borrows for the call
*r += 1;                       // ✅ r is still valid
```

## Dereferencing

Rust auto-derefs in most cases (method calls, field access). Explicit deref uses `*`:

```rust
let n = 10;
let r = &n;
assert_eq!(*r, 10);
```

For smart pointers this is the `Deref` trait — see [[../09-smart-pointers/deref|Deref]].

## Common error: "cannot borrow as mutable"

```rust
let v = vec![1, 2, 3];
v.push(4);          // ❌ v isn't declared mut
```

Fix: `let mut v = vec![...];`

## Common error: self-referential borrow

```rust
let mut s = String::from("hello");
let first = &s;                  // shared borrow
s.push_str(" world");            // ❌ mutable use while shared borrow live
println!("{first}");
```

Fix: shorten the borrow scope, or clone, or restructure.

## Related
- [[slices|Slices — cheap views into a collection]]
- [[lifetimes|Lifetimes — naming reference scopes]]
- [[../09-smart-pointers/refcell|RefCell — runtime-checked borrowing]]
