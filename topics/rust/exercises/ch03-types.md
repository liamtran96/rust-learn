---
title: Ch 3 — Types & Traits Exercises
tags: [rust, exercises, types, traits]
---

# Ch 3 — Types & Traits Exercises

## Modeling
1. Model a `Shape` enum with `Circle { radius: f64 }`, `Rectangle { w: f64, h: f64 }`, and `Triangle { a: f64, b: f64, c: f64 }`. Implement `fn area(&self) -> f64` via `match`.
2. Model a state machine for a network connection: `Disconnected`, `Connecting { attempts: u32 }`, `Connected { peer: String }`, `Failed(String)`. Write methods that return new states on events like `on_connect_success(peer)`.

## Generics
3. Write `fn largest<T: PartialOrd + Copy>(xs: &[T]) -> T`. Make a version that returns `Option<T>` to handle empty slices. Make a non-Copy version returning `Option<&T>`.
4. Write a `Pair<T, U>` struct with `fn swap(self) -> Pair<U, T>`.

## Traits
5. Define `trait Animal { fn name(&self) -> &str; fn sound(&self) -> String; fn describe(&self) -> String { format!("{} says {}", self.name(), self.sound()) } }`. Implement for `Dog`, `Cat`.
6. Define `trait Summary { fn summarize(&self) -> String; }`. Write `fn notify<T: Summary>(s: &T)` and `fn notify_dyn(items: &[Box<dyn Summary>])`.
7. Use the **newtype pattern** to wrap `u64` in a `UserId` and an `OrderId`, and write a function that refuses to take the wrong one.

## Derives
8. For a `struct Point { x: i32, y: i32 }`, derive `Debug, Clone, Copy, PartialEq, Eq, Hash`. Verify each by writing a test that uses the behavior.
