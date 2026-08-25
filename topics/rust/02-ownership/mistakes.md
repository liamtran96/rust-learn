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

### 2026-08-25 - Move direction reversed (ownership drill d01)
- **What I wrote:** "move the ownership from t to s"
- **Why it's wrong:** In `let t = s`, the value starts in `s`; the assignment transfers that owned `String` into `t`, not the other way around. After the move, `t` is valid and `s` is invalid.
- **The rule:** For a non-`Copy` value, `let destination = source` moves ownership from `source` to `destination`. Using `&source` creates a borrow instead and leaves the source as owner.
- **Status:** 🟥 fresh

### 2026-08-25 - Clone confused with Copy and borrowing (ownership drill d02)
- **What I wrote:** "Copy type mean it has the ownership" and used `let b = a.clone()` to make the `String` test pass.
- **Why it's wrong:** Ownership and `Copy` are different concepts: every value has an owner, but only some types are implicitly duplicated by assignment. Cloning a `String` explicitly duplicates its heap data and was unnecessary when the test only needed shared read access.
- **The rule:** `Copy` makes ordinary assignment leave the source usable; a move transfers a non-`Copy` value; `&value` borrows without ownership transfer; `.clone()` is reserved for a genuinely required independent owned copy.
- **Status:** 🟥 fresh

### 2026-08-25 - Borrow lifetime confused with lexical scope (ownership drill d04)
- **What I wrote:** "when it return new value" and then "when it out of scope right"
- **Why it's wrong:** A borrow does not depend on returning a value, and it need not remain active for the reference variable's entire lexical scope. Rust can end the borrow after the reference's final use, even though the name remains in scope.
- **The rule:** Going out of scope always ends a borrow, but non-lexical lifetime analysis may end it earlier at the reference's last use, allowing a later non-overlapping borrow.
- **Status:** 🟥 fresh
