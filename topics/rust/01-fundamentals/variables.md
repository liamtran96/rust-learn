---
title: 1.2 Variables, Mutability, Shadowing
tags: [rust, fundamentals, variables]
---

# 1.2 Variables, Mutability, Shadowing

## Immutable by default

```rust
let x = 5;
x = 6;       // ❌ compile error: cannot assign twice to immutable variable
```

```rust
let mut x = 5;
x = 6;       // ✅
```

**Why this default matters:** a `mut` variable is a flag to every reader that this value *changes*. It's one of the quiet features that makes Rust code easier to review.

## Shadowing ≠ mutation

```rust
let spaces = "   ";
let spaces = spaces.len();  // shadowed with a different type
```

Shadowing creates a **new variable** in the same scope. The old binding is gone (to that name). This is different from `mut`:
- `mut` — same variable, new value, **same type**.
- Shadowing — new variable, can be a **different type**.

## Constants

```rust
const MAX_RETRIES: u32 = 3;
```

- Always `UPPER_SNAKE_CASE`.
- Must have an explicit type.
- Can be declared at any scope, including global.
- Evaluated at compile time → can only use `const` expressions.

## `static` vs `const`

| Aspect | `const` | `static` |
|---|---|---|
| Storage | Inlined at each use | Fixed memory location |
| Address | No stable address | Has a stable address |
| Mutation | Never | `static mut` exists but requires `unsafe` |
| Use for | Compile-time values, numbers | Globals, especially FFI |

Prefer `const` unless you need a stable address or interior mutability via `OnceLock`/`Mutex`.

## Type annotations

Type inference is good but not magic. Annotate when:
1. You have an ambiguous literal: `let x: u64 = 42;`
2. You're declaring a function parameter or struct field (**required**).
3. The collection type can't be inferred: `let v: Vec<i32> = vec![];`

## Gotchas

- **Integer literals default to `i32`**, float literals to `f64`.
- **Shadowing shadows only in the current scope** — leaving the block restores the outer binding.
- **`let` inside `if` branches doesn't leak out** — use `let x = if cond { a } else { b };` instead.

## Related
- [[data-types|Data types]]
- [[functions|Functions & expressions]]
- [[../02-ownership/ownership|Ownership]] — why immutability is even more valuable here
