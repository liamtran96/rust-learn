---
title: Chapter 3 Mistakes
tags: [rust, mistakes, types, enums, traits]
---

# Chapter 3 Mistakes

## Open mistakes

### 2026-09-18 - Confusing empty input with an empty element (`generic-largest`)
- **What I wrote:** "because the String value can be empty so we should optional and return None if it's empty"
- **Why it's wrong:** `largest_borrowed` may validly return a reference to an empty `String` if that element wins the comparison. `None` represents a slice containing no elements at all, because only then is there no possible winner.
- **The rule:** Use `Option` when a result may be absent; here absence depends on the slice length, not on the contents of a `String` element.
- **Status:** dYY - fresh

### 2026-09-18 - Test name shadowed the function under test (`generic-largest`)
- **What I wrote:** Named test functions `largest_optional` and `largest_borrowed`, then tried to call the production functions by those names inside the test module.
- **Why it's wrong:** A test function declared in the module shadows an item with the same name brought in by `use super::*`, so the call resolved to the zero-argument test function instead.
- **The rule:** Give tests behavior-oriented names such as `optional_returns_none_for_empty`; avoid reusing the exact name of the function under test in the same namespace.
- **Status:** dYY - fresh

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

### 2026-09-14 - Confusing an enum variant with a field (struct-versus-enum retrieval)
- **What I wrote:** "each shape value is exactly one field"
- **Why it's wrong:** A field is one stored piece of data, such as `radius`; it is not the alternative case represented by the whole value. `Circle` and `Rectangle` are variants, and their payloads contain fields.
- **The rule:** An enum value has exactly one active variant; that variant stores zero or more fields.
- **Status:** fresh

### 2026-09-14 - Using a namespace path instead of a method call (`network-state`)
- **What I wrote:** `connection::on_connect_attempt()` and, earlier, `ConnectionState::on_connect_attempt` without calling it on the existing value.
- **Why it's wrong:** `::` selects an item through a type or module path, while the lowercase `connection` binding is a value whose consuming method must receive that value as `self`. Naming a method without the call syntax produces a function item rather than the returned `ConnectionState`.
- **The rule:** Use `Type::Variant` to select an enum variant and `value.method()` to call a method on a particular value; the expression before `.` supplies `self`.
- **Status:** 🟥 fresh

### 2026-09-14 - Comparing a test result with itself (`network-state`)
- **What I wrote:** `assert_eq!(message, message)`
- **Why it's wrong:** Both sides refer to the same value, so the assertion passes regardless of which message the transition stored. It cannot detect a regression that changes or discards the caller-provided message.
- **The rule:** Compare the actual result with an independently specified expected value, such as `assert_eq!(message, "timed out")`.
- **Status:** 🟥 fresh

## Resolved (kept for reference)

*(none yet)*
