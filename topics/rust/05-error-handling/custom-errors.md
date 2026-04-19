---
title: 5.3 Custom Error Types
tags: [rust, errors, thiserror, anyhow]
---

# 5.3 Custom Error Types

Libraries should define **concrete error enums**. Applications can leverage catch-all errors.

## Rule of thumb

- **Libraries** → `thiserror` to build a typed enum per module.
- **Applications / binaries** → `anyhow` to blanket-bubble any error with context.

(`thiserror` and `anyhow` are by the same author and designed as a pair.)

## `thiserror` — library style

```rust
// Cargo.toml
// [dependencies]
// thiserror = "1"

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("config file not found at {0}")]
    Missing(std::path::PathBuf),

    #[error("invalid field `{field}`: {reason}")]
    InvalidField { field: String, reason: String },

    #[error("io error")]
    Io(#[from] std::io::Error),           // auto From impl for ?

    #[error("parse error")]
    Parse(#[from] serde_json::Error),
}
```

The derive generates:
- `impl Display` — from the `#[error(...)]` strings.
- `impl Error` — including `source()` from `#[from]` and `#[source]` fields.
- `impl From<io::Error> for ConfigError` — from `#[from]`.

## `anyhow` — app style

```rust
// Cargo.toml
// [dependencies]
// anyhow = "1"

use anyhow::{Context, Result};

fn load(path: &str) -> Result<Config> {
    let bytes = std::fs::read(path)
        .with_context(|| format!("reading config at {path}"))?;
    let cfg: Config = serde_json::from_slice(&bytes)
        .context("parsing config")?;
    Ok(cfg)
}
```

`anyhow::Result<T>` = `Result<T, anyhow::Error>`. `anyhow::Error` wraps any `Error + Send + Sync + 'static` and adds context chains — the printed output shows the full cause chain:

```
Error: reading config at app.json

Caused by:
    No such file or directory (os error 2)
```

## Do I still need my own enum?

- Library author: yes — define an enum per module. Users want to `match` your errors.
- Binary author: rarely — `anyhow::Error` is almost always fine.
- Hybrid: define enums for the interesting operational errors, bubble everything else via `anyhow`.

## Without the crates

You can hand-roll a type, but `thiserror` saves ~20 lines per error:

```rust
#[derive(Debug)]
enum MyError { NotFound, Io(std::io::Error) }

impl std::fmt::Display for MyError { /* ... */ }
impl std::error::Error for MyError { /* ... */ }
impl From<std::io::Error> for MyError { /* ... */ }
```

## Related
- [[question-mark|?]]
- [[result-option|Result & Option]]
