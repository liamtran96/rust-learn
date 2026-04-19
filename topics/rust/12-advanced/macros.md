---
title: 12.2 Macros
tags: [rust, macros]
---

# 12.2 Macros

Two flavors:
- **Declarative** (`macro_rules!`) — pattern-matching substitution.
- **Procedural** — separate compiled crates that operate on token streams; used for custom `#[derive]`, attribute macros, and function-like macros.

## Declarative macros

```rust
macro_rules! hashmap {
    ( $( $k:expr => $v:expr ),* $(,)? ) => {{
        let mut m = ::std::collections::HashMap::new();
        $( m.insert($k, $v); )*
        m
    }};
}

let m = hashmap! { "a" => 1, "b" => 2, };
```

Pattern kinds you'll meet: `expr`, `ident`, `ty`, `pat`, `block`, `stmt`, `tt`, `literal`.

## When to reach for one

- Repetitive boilerplate the borrow-less world can't DRY with a function (e.g., builders over many types).
- DSLs — `html!`, `sql!`, `println!`-like formatting.
- Compile-time enum-to-string tables.

## Procedural macros (proc macros)

Used via `#[derive(Foo)]`, `#[some_attribute]`, or `my_macro!()`. They live in their own crate (`proc-macro = true` in Cargo.toml) and consume/produce `TokenStream`.

Typical stack: `syn` for parsing Rust syntax, `quote` for code generation.

Popular proc macros you already use:
- `#[derive(Serialize, Deserialize)]` — serde
- `#[tokio::main]` — tokio
- `#[derive(Error)]` — thiserror
- `#[instrument]` — tracing

## Rule of thumb

Start with **functions & generics**. Reach for macros only when the duplication is structural (e.g., DSL, matching on tokens) rather than data.

## Related
- [[unsafe|Unsafe]]
- [[advanced-traits|Advanced traits]]
