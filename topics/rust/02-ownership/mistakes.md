---
title: Ch 2 — My mistakes & misunderstandings
tags: [rust, ownership, mistakes, review]
---

# Ch 2 — My mistakes & misunderstandings

> Personal log of what I got wrong, why I got it wrong, and the rule that fixes it.
> Review these entries at the start of each Ch 2 study session until each rule feels obvious.

## Open mistakes (review these)

### 2026-08-27 - Word splitting destroyed line structure (`strip_margin`)
- **What I wrote:** `let words = s.split_whitespace();` followed by `words.collect::<Vec<&str>>().join(" ")`
- **Why it's wrong:** `split_whitespace` treats every run of whitespace, including newlines, as a separator. Joining with a space therefore flattens the input into words and loses the line boundaries that `strip_margin` must process independently.
- **The rule:** Choose an iterator whose units match the transformation: use `lines` for line-by-line work, and use `split_whitespace` only when whitespace-separated words are the intended units.
- **Status:** 🆕 fresh

### 2026-08-27 - Arithmetic result discarded instead of assigned (ownership drill d10)
- **What I wrote:** `let _ = *value * 2;` and then `*value * 2;`
- **Why it's wrong:** Both expressions calculate a doubled integer without writing it through the mutable reference, so the original slice remains unchanged. The empty-input test still passes because the loop body never runs, which does not verify mutation behavior.
- **The rule:** Dereference an `&mut T` and assign through it to change the referent; compound assignment such as `*value *= 2` both calculates and stores the result.
- **Status:** 🟥 fresh

### 2026-08-26 - `todo!()` and a discarded tail value (ownership drill d09)
- **What I wrote:** Predicted "no" to whether the file with `todo!()` compiles, then wrote `first.unwrap_or("");` before leaving `todo!()` in place.
- **Why it's wrong:** `todo!()` has the never type and can stand where a `&str` is expected, so the file compiles but panics when that line executes. The semicolon turns the desired `&str` into a discarded statement result; it must be the tail expression to become the function's return value.
- **The rule:** `todo!()` is a compiling runtime panic placeholder. A block returns its final expression only when that expression has no trailing semicolon.
- **Status:** 🟥 fresh

### 2026-08-26 - Lifetime annotation treated as extending values (ownership drill d08)
- **What I wrote:** "`<'a>` ... keep those parameter valid during run time"
- **Why it's wrong:** A lifetime annotation does not keep a value alive or alter runtime behavior. It names a relationship the compiler checks: both possible source references must be valid for `'a`, and the returned reference may be used for no longer than that shared period.
- **The rule:** Lifetimes describe and constrain relationships between references; they do not extend the lifetimes of the referenced values.
- **Status:** 🟥 fresh

### 2026-08-26 - Heap location confused with returned ownership (ownership drill d07)
- **What I wrote:** "WHY:in the heap"
- **Why it's wrong:** A `String` does store its text buffer on the heap, but memory location alone does not explain why the return is safe. The decisive change is that ownership of the `String` moves out of the function to the caller, so the heap buffer is not freed when the local function scope ends.
- **The rule:** A function may return an owned local value by moving it to the caller. It cannot return a reference to that local because the local owner would be dropped when the function returns.
- **Status:** 🟥 fresh

### 2026-08-26 - Owned `String` confused with deref coercion (ownership drill d06)
- **What I wrote:** "String is copy type" and "String will be automatically change to &str type."
- **Why it's wrong:** `String` is not `Copy`, and the owned `String` does not change its type. At the call to a function expecting `&str`, Rust coerces the argument from `&String` to `&str` because `String` dereferences to `str`.
- **The rule:** String literals already have type `&str`; an owned `String` remains a `String`, while a shared `&String` can undergo deref coercion to `&str` for function and method arguments.
- **Status:** 🟥 fresh

### 2026-08-26 - Copy confused with borrowing and iteration ownership (ownership drill d05)
- **What I wrote:** "because v is not a copy type and i use & to reference to the v we use it temporary"; the prediction also said "the x take the ownership from v."
- **Why it's wrong:** Whether the element type is `Copy` does not decide whether the vector is consumed. The owned-vector iterator takes `v` by value, while `x` receives one element at a time; iterating over `&v` instead borrows the vector and changes `x` from `i32` to `&i32`.
- **The rule:** `for x in collection` follows the collection's `IntoIterator` implementation. An owned `Vec<T>` is consumed and yields `T`; a shared borrow `&Vec<T>` is not consumed and yields `&T`.
- **Status:** 🟥 fresh

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
