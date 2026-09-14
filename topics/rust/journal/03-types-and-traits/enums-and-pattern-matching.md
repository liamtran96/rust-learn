---
title: Enums and Pattern Matching
tags: [rust, journal, enums, pattern-matching]
---

# Enums and Pattern Matching

[[../../journal|Journal index]]

## Entries

### 2026-09-14 - Shape enum and area method
**Working on:** Shape area - `code/03-types-and-traits/shape-area/`
**What clicked:** A single `Shape` enum can represent several alternatives whose variants carry different named fields. An `area(&self)` method can immutably borrow the current shape, and an exhaustive `match` can destructure its active variant and return the appropriate calculation.
**What didn't:** `area` was initially treated like a free function taking a string rather than a method on a `Shape` value. The triangle constructor briefly contained calculation statements instead of its `a`, `b`, and `c` fields, and the purpose of `&self` was blended together with the separate role of match patterns. The circle test uses exact floating-point equality; it passes for the copied result but a tolerance comparison would be less brittle.
**Questions asked this session:**
- **Q:** "Example::First what is first?"
  - **Technical answer:** `Example` was a placeholder enum type and `First` was one of its placeholder variants. In this exercise the concrete paths are `Shape::Circle`, `Shape::Rectangle`, and `Shape::Triangle`; `::` selects a named item associated with the enum.
  - **Plain-English analogy / example:**
    ```rust
    enum Shape { Circle { radius: f64 } }
    let shape = Shape::Circle { radius: 2.0 };
    ```
  - **See also:** `topics/rust/03-types-and-traits/enums.md`
- **Q:** What do `&self` and `match self` do?
  - **Technical answer:** In `impl Shape`, `&self` is shorthand for `self: &Shape`, so the method reads a particular `Shape` without consuming it. `match self` inspects which variant the borrowed value contains, while patterns such as `Shape::Rectangle { w, h }` destructure that variant and bind references to its fields.
  - **Plain-English analogy / example:**
    ```rust
    match self {
        Shape::Rectangle { w, h } => w * h,
        // the other variants must also be covered
    }
    ```
  - **See also:** `topics/rust/03-types-and-traits/pattern-matching.md`
**Question to answer later:** Can Liam reconstruct the enum, exhaustive method, and tolerance-based floating-point tests without step-by-step prompting?
**Next:** Read `topics/rust/03-types-and-traits/structs.md`, then explain how a struct differs from an enum before starting the next Ch 3 exercise.
