---
title: 3.2 Enums
tags: [rust, enums, sum-types]
---

# 3.2 Enums

> If you only take **one** thing from Rust into every other language you use, make it enums.

## A tagged union, done right

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

Each variant can hold different shapes of data (tuple-style, struct-style, or nothing). The compiler stores **one tag + the data**.

## Standard library enums you'll use constantly

```rust
enum Option<T> { None, Some(T) }
enum Result<T, E> { Ok(T), Err(E) }
```

These replace `null`, exceptions, and most sentinel values in the entire ecosystem. See [[../05-error-handling/result-option|Result/Option]].

## Using them — `match`

```rust
fn describe(ip: &IpAddr) -> String {
    match ip {
        IpAddr::V4(a, b, c, d) => format!("IPv4 {a}.{b}.{c}.{d}"),
        IpAddr::V6(s)          => format!("IPv6 {s}"),
    }
}
```

The compiler enforces **exhaustiveness** — if you add a new variant later, every `match` that doesn't handle it becomes a compile error. This is one of the most valuable forms of refactor-safety in any language.

## Methods on enums

```rust
impl IpAddr {
    fn is_loopback(&self) -> bool {
        matches!(self, IpAddr::V4(127, _, _, _) | IpAddr::V6(s) if s == "::1")
    }
}
```

## Modeling state machines

```rust
enum Connection {
    Disconnected,
    Connecting { attempt: u32 },
    Connected { since: Instant, peer: SocketAddr },
    Failed(std::io::Error),
}
```

Now impossible-state combinations (e.g., "connected but no peer") are literally unrepresentable. This is the single highest-leverage Rust pattern for application code.

## Size & niches

The compiler is clever about layout:
- `Option<&T>` is the same size as `&T` — `None` uses the null pointer niche.
- `Option<NonZeroU32>` is the same size as `u32` — uses the zero niche.

So "enum-wrapping" is usually free.

## Related
- [[pattern-matching|Pattern matching — the consumer of enums]]
- [[../05-error-handling/result-option|Result and Option]]
