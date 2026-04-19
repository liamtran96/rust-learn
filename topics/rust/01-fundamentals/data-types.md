---
title: 1.3 Data Types
tags: [rust, fundamentals, types]
---

# 1.3 Data Types

Rust is **statically typed** — every value's type is known at compile time. The compiler infers most types, but the *set* of types you need to recognize is small.

## Scalar types

| Category | Types |
|---|---|
| Signed integers | `i8`, `i16`, `i32`, `i64`, `i128`, `isize` |
| Unsigned integers | `u8`, `u16`, `u32`, `u64`, `u128`, `usize` |
| Floats | `f32`, `f64` |
| Boolean | `bool` → `true` / `false` |
| Character | `char` — a **Unicode scalar value**, 4 bytes, not a byte |

### Rules of thumb
- Default to `i32` for integers, `f64` for floats unless you have reason.
- Use `usize` for indexing and sizes (length of collections, array indices).
- `u8` is the byte — but `String` is UTF-8, not bytes; see [[../04-collections/strings|Strings]].

### Overflow behavior
- **Debug builds**: panic on overflow.
- **Release builds**: two's-complement wrap.
- Use explicit methods when you care: `wrapping_add`, `checked_add`, `saturating_add`, `overflowing_add`.

## Compound types

### Tuples — fixed-size, heterogeneous

```rust
let t: (i32, f64, char) = (42, 3.14, 'z');
let (a, b, c) = t;         // destructuring
let first = t.0;           // indexed access
```

The empty tuple `()` is the **unit type** — what functions return when they "return nothing".

### Arrays — fixed-size, homogeneous, stack-allocated

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
let zeros = [0; 10];       // [0, 0, 0, ... 10 times]
let x = a[0];              // bounds-checked
```

For growable sequences, use [[../04-collections/vec|`Vec<T>`]].

### Slices — a borrowed view into a sequence

```rust
let a = [1, 2, 3, 4, 5];
let s: &[i32] = &a[1..4];  // [2, 3, 4]
```

See [[../02-ownership/slices|Slices]].

## Type conversions

Rust forbids **implicit** conversions. Use:

```rust
let x: i32 = 10;
let y: i64 = x as i64;          // primitive cast — can truncate silently!
let y: i64 = i64::from(x);      // infallible widening — safe
let z: u8  = x.try_into()?;     // fallible narrowing — returns Result
```

Prefer `From`/`Into`/`TryFrom`/`TryInto` over `as` for non-numeric conversions.

## Inference limits

Inference is **local** — the compiler infers within a function but not across function boundaries. Function signatures always need explicit types.

```rust
fn double(x: i32) -> i32 { x * 2 }  // both required
```

## Related
- [[variables|Variables & mutability]]
- [[../03-types-and-traits/index|Custom types — struct, enum]]
- [[../04-collections/index|Growable collections]]
