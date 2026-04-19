---
title: 8.1 Closures
tags: [rust, closures]
---

# 8.1 Closures

## Syntax

```rust
let add = |a, b| a + b;
let add: fn(i32, i32) -> i32 = |a, b| a + b;    // annotated
let add = |a: i32, b: i32| -> i32 { a + b };    // fully spelled out
```

## Capturing

Closures can reference variables from their environment. The compiler picks the *least restrictive* capture mode:

```rust
let name = String::from("Alice");

let greet_borrow    = || println!("hi {name}");        // &name
let greet_mut_borrow = {
    let mut n = name.clone();
    move || { n.push('!'); println!("{n}"); }          // &mut n (captured in closure)
};
let greet_move      = move || println!("hi {name}");   // moves name into closure
```

## Fn / FnMut / FnOnce — what kind of callable is it?

| Trait | Can be called | Captures |
|---|---|---|
| `Fn` | Any number of times, immutably | `&T` |
| `FnMut` | Any number of times, requires `&mut self` | `&mut T` |
| `FnOnce` | Exactly once (consumes self) | Moves `T` |

Every closure implements as many of these as its captures allow:
- A closure that only reads captures: `Fn + FnMut + FnOnce`.
- One that mutates captures: `FnMut + FnOnce`.
- One that moves out of captures: `FnOnce` only.

## Taking closures as arguments

```rust
fn call_twice<F: Fn()>(f: F) { f(); f(); }

fn call_mutating<F: FnMut()>(mut f: F) { f(); f(); }

fn call_once<F: FnOnce()>(f: F) { f(); }
```

Or with `impl Trait`:
```rust
fn apply(f: impl Fn(i32) -> i32) -> i32 { f(10) + f(20) }
```

## Returning closures

Closures have unnameable types. Return them as `impl Fn(...)` or box them:

```rust
fn make_adder(x: i32) -> impl Fn(i32) -> i32 { move |y| x + y }

fn boxed_adder(x: i32) -> Box<dyn Fn(i32) -> i32> { Box::new(move |y| x + y) }
```

Use `Box<dyn Fn…>` when you need a **heterogeneous** collection or when `impl Trait` isn't allowed (e.g., storing in a struct field).

## `move` keyword

`move` forces the closure to **take ownership** of captures. Necessary when the closure outlives its environment — most commonly when spawning threads or returning closures:

```rust
let name = String::from("Alice");
std::thread::spawn(move || println!("{name}"));
```

Without `move`, the closure would try to borrow `name`, but `name` is owned by the outer function and the thread can outlive it.

## Related
- [[iterators|Iterators — closures' main customer]]
- [[../10-concurrency/threads|Threads — where `move` shows up]]
