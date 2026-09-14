---
title: Chapter 3 Mistakes
tags: [rust, mistakes, types, enums, traits]
---

# Chapter 3 Mistakes

## Open mistakes

### 2026-09-14 - Calling an enum method without an enum value (`shape-area`)
- **What I wrote:** Tried to call `area("Circle")` instead of constructing a `Shape` and calling its method.
- **Why it's wrong:** `area` has a `self` receiver, so it operates on a concrete enum value whose active variant carries the required dimensions. A string such as `"Circle"` has neither the `Shape` type nor a radius.
- **The rule:** Construct the correct variant first, then use method syntax such as `shape.area()`; `&self` borrows that value for the call.
- **Status:** fresh

### 2026-09-14 - Putting calculations inside an enum constructor (`shape-area` tests)
- **What I wrote:** Placed `let s = ...` and `let area_squared = ...` inside `Shape::Triangle { ... }`.
- **Why it's wrong:** Struct-style variant braces initialize named fields; the parser therefore expects field names such as `a`, `b`, and `c`, not statements. The area formula belongs in the method that consumes those stored values.
- **The rule:** A constructor supplies the declared payload fields; behavior belongs in functions or methods.
- **Status:** fresh

### 2026-09-14 - Exact equality for a calculated floating-point result (`shape-area` tests)
- **What I wrote:** `assert_eq!(shape.area(), 12.566370614359172);`
- **Why it's wrong:** This test currently passes, but exact equality is brittle when floating-point calculations or their operation order change because many real numbers cannot be represented exactly in binary.
- **The rule:** For calculated `f64` values, usually assert that `(actual - expected).abs()` is smaller than a chosen tolerance.
- **Status:** fresh

## Resolved (kept for reference)

*(none yet)*
