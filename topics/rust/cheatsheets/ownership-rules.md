---
title: Ownership Rules Cheatsheet
tags: [rust, cheatsheet, ownership]
---

# Ownership Rules Cheatsheet

## The three rules
1. Every value has **one owner**.
2. Ownership **moves** on assignment / passing (unless the type is `Copy`).
3. Values are **dropped** when the owner goes out of scope.

## Borrow rules
- At any moment: **one `&mut T`** XOR **many `&T`** — never both.
- References must always be valid.

## Fast decision table

| Situation | Pass by |
|---|---|
| Don't need to mutate, don't need to keep it | `&T` |
| Need to mutate | `&mut T` |
| Want to store/own it afterwards | `T` (move) |
| Want both sides to own it | `T` with `.clone()` or `Rc`/`Arc` |

## `Copy` vs `Clone`

- `Copy`: implicit duplicate. Cheap, bit-for-bit.
  - Implemented for: primitives, `char`, `bool`, shared refs `&T`, tuples/arrays of `Copy` types, `Option<T>` when `T: Copy`.
- `Clone`: explicit `.clone()`. May allocate.
- `Copy` implies `Clone`.

## Common moves you don't expect
- Assigning a non-`Copy` value to another variable.
- Passing a non-`Copy` value to a function by value.
- `for x in vec` — consumes `vec`. Use `&vec` or `&mut vec`.

## Reading the errors
- "cannot borrow as mutable" → variable isn't declared `mut`, **or** a shared borrow is alive.
- "borrow of moved value" → value was moved away; either clone or borrow instead.
- "cannot move out of borrowed content" → trying to take ownership from a `&T`. Clone, or destructure with `match`/patterns.
- "`X` does not live long enough" → a reference's target drops before the reference does.

## Fixes you'll try in order
1. Does the caller need to keep it? If yes → borrow with `&`/`&mut`.
2. Can I restructure to not need both at once? (Split into separate scopes.)
3. Do I truly need shared mutation? → `Rc<RefCell<T>>` (single-thread) or `Arc<Mutex<T>>` (multi-thread).
4. Truly need fearless sharing? → Clone.
