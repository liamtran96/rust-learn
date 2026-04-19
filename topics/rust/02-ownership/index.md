---
title: 2. Ownership, Borrowing & Lifetimes
tags: [rust, ownership, borrowing, lifetimes]
---

# 2. Ownership, Borrowing & Lifetimes

> This chapter **is** Rust. Everything else is consequence.

## Why it exists
Rust has **no GC** and **no runtime**. It still guarantees memory safety *and* data-race safety. It does this entirely at compile time, via the ownership/borrowing/lifetime system.

## Contents
- [[ownership|2.1 Ownership rules — move, copy, drop]]
- [[borrowing|2.2 References & borrowing]]
- [[slices|2.3 Slices — borrowed views into sequences]]
- [[lifetimes|2.4 Lifetimes — named scopes]]

## The 3 rules of ownership
1. Every value has an **owner** (a variable binding).
2. A value has **exactly one owner at a time**.
3. When the owner goes out of scope, the value is **dropped**.

## The 2 rules of borrowing
At any point in time:
1. **Either** one `&mut T` mutable reference, **or** any number of `&T` shared references — **never both**.
2. References must always be **valid** (point to live data).

## Mental model

Think of Rust values as real-world objects:
- **Move** is handing the object over. You no longer have it.
- **Borrow (`&T`)** is letting someone read it. Many people can read at once.
- **Borrow mut (`&mut T`)** is handing it over temporarily for changes. Exclusive.
- **Drop** is throwing it away — automatic when the owner leaves scope.

This is also why most beginner "fights" with the borrow checker reveal genuine aliasing bugs that a C++ program would have silently.

## Exit criteria
- [ ] You can predict when `String` moves vs when `i32` copies, and explain why.
- [ ] You know the difference between `&str` and `String`, and when to use each.
- [ ] You can write a function that takes `&mut` and mutates through it.
- [ ] You understand why `let s = String::from("hi"); let t = s; println!("{s}");` doesn't compile.
- [ ] You can write a function signature with a lifetime parameter (`fn f<'a>(x: &'a str) -> &'a str`) and explain what it promises.
