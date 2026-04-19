---
title: Trait Bounds Cheatsheet
tags: [rust, cheatsheet, traits]
---

# Trait Bounds Cheatsheet

## The ones you'll write the most

| Bound | Means "the type supports" |
|---|---|
| `T: Copy` | bit-for-bit duplication |
| `T: Clone` | explicit `.clone()` |
| `T: Debug` | `{:?}` formatting |
| `T: Display` | `{}` formatting |
| `T: Default` | `T::default()` |
| `T: PartialEq` / `T: Eq` | `==`, `!=` |
| `T: Ord` / `T: PartialOrd` | `<`, `>`, sorting |
| `T: Hash` | use as a HashMap key |
| `T: Iterator<Item = U>` | can be iterated, yielding `U` |
| `T: FromIterator<U>` | can be built by `collect()` |
| `T: From<U>` / `T: Into<U>` | conversions |
| `T: TryFrom<U>` / `T: TryInto<U>` | fallible conversions |
| `T: AsRef<U>` | cheap `&T → &U` |
| `T: Deref<Target = U>` | smart pointer deref |
| `T: Send` | safe to cross threads (ownership) |
| `T: Sync` | `&T` safe to cross threads |
| `T: 'static` | no non-static borrows |
| `F: Fn(A) -> B` | callable (read captures) |
| `F: FnMut(A) -> B` | callable (mutate captures) |
| `F: FnOnce(A) -> B` | callable once |

## Combining
```rust
fn f<T>(x: T) where T: Clone + Debug + 'static { /* ... */ }
fn g<T: Clone + Debug>(x: T) { /* ... */ }          // same
fn h(x: impl Clone + Debug) { /* ... */ }           // same if T used once
```

## Where clauses — for readability

```rust
fn f<T, U, F>(xs: &[T], f: F) -> Vec<U>
where
    T: Clone,
    U: Debug,
    F: Fn(&T) -> U,
{ /* ... */ }
```

## `?Sized`

By default, generic bounds assume `Sized`. Opt out with `T: ?Sized` when accepting `str`, `[T]`, or `dyn Trait`:

```rust
fn print_any<T: Debug + ?Sized>(x: &T) { println!("{:?}", x); }
```

## `impl Trait` vs generic vs `dyn Trait`

| | static dispatch | dyn dispatch |
|---|---|---|
| Argument | `fn f(x: impl Trait)` | `fn f(x: &dyn Trait)` |
| Return | `fn f() -> impl Trait` | `fn f() -> Box<dyn Trait>` |
| Stored field | `struct S<T: Trait> { t: T }` | `struct S { t: Box<dyn Trait> }` |
