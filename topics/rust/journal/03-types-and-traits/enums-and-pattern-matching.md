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

### 2026-09-14 - Struct versus enum retrieval
**Working on:** Preparation for Ch 3 network state machine - `code/03-types-and-traits/network-state/`
**What clicked:** Liam corrected the distinction that a `Shape` value is exactly one variant, while each variant stores only the fields needed for that case. An enum therefore makes circle, rectangle, and triangle mutually exclusive at the type level.
**What didn't:** The first recall used `field` where `variant` was intended. Liam did not initially identify an invalid value permitted by an all-in-one struct until shown optional radius, width, and height fields that could represent multiple shapes or no shape.
**Questions asked this session:**
- **Q:** Why is `Shape` better modeled as an enum instead of one struct containing every possible shape field?
  - **Technical answer:** A struct instance contains all fields declared by that struct, whereas an enum value contains exactly one active variant. Each enum variant may carry different fields, so `Shape` can require precisely the data belonging to its selected case.
  - **Plain-English analogy / example:**
    ```rust
    enum Shape {
        Circle { radius: f64 },
        Rectangle { w: f64, h: f64 },
    }
    ```
  - **See also:** `topics/rust/03-types-and-traits/structs.md`, `topics/rust/03-types-and-traits/enums.md`
- **Q:** What invalid combination could an all-in-one `Shape` struct accidentally allow?
  - **Technical answer:** Optional fields could all be present, claiming that one value is both a circle and rectangle, or all be absent, representing no shape. The enum declaration rules out both combinations because only one named variant can be active and that variant requires its own payload.
  - **Plain-English analogy / example:**
    ```text
    radius = Some(2.0)
    w      = Some(3.0)
    h      = Some(4.0)
    invalid: circle and rectangle at the same time
    ```
  - **See also:** `topics/rust/03-types-and-traits/enums.md`
**Question to answer later:** Can Liam independently name both an impossible all-fields-present state and an all-fields-absent state?
**Next:** Open `code/03-types-and-traits/network-state/BRIEF.md`, then type the enum and one initial state in `src/main.rs`.
