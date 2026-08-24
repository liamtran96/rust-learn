---
title: Ch 2 — My mistakes & misunderstandings
tags: [rust, ownership, mistakes, review]
---

# Ch 2 — My mistakes & misunderstandings

> Personal log of what I got wrong, why I got it wrong, and the rule that fixes it.
> Review these entries at the start of each Ch 2 study session until each rule feels obvious.

## Open mistakes (review these)

### 2026-08-17 — Reference target and `Vec` mutation (predict-and-fix B)
- **What I wrote:** Said the snippet compiled, that `r = &v[0]` referred to `4`, and that the concern was “maybe the original `v` can be reused somewhere.”
- **Why it's wrong:** Index `0` refers to the existing first element, `1`; the later `push(4)` appends at index `3`. More importantly, `push` may reallocate the vector's buffer, so Rust cannot allow that mutable borrow while `r` still needs an address inside the old buffer.
- **The rule:** A shared borrow `&T` and a mutable borrow `&mut T` cannot overlap. Use the shared reference for the last time before mutating its owner; non-lexical lifetimes let the borrow end at that final use.
- **Status:** 🟥 fresh

### 2026-08-24 — Returned slice used after its owner was dropped (predict-and-fix D)
- **What I wrote:** Predicted “yes” when asked whether `word` could be printed after the inner block containing `text: String` had ended.
- **Why it's wrong:** `word` is a borrowed `&str` pointing into `text`; leaving the inner block drops `text` and invalidates that data. Allowing the later print would create a dangling reference.
- **The rule:** A borrowed slice cannot outlive the value it references. Use the slice before its owner is dropped, or move the owner into a scope that lasts through the slice's final use.
- **Status:** 🟥 fresh
