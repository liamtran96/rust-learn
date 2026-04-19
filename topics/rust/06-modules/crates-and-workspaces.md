---
title: 6.2 Crates & Workspaces
tags: [rust, crates, workspaces]
---

# 6.2 Crates & Workspaces

## Crate types

- **Binary crate**: has `src/main.rs`, produces an executable.
- **Library crate**: has `src/lib.rs`, produces `.rlib` / `.so`.
- A project can have **both** — often a lib for logic + a thin main for CLI.

Additional binaries go in `src/bin/*.rs` and are run via `cargo run --bin name`.

## Dependencies — `Cargo.toml`

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
uuid  = { version = "1", default-features = false, features = ["v4"] }

# Local path (for workspaces / development)
my_shared = { path = "../shared" }

# Git
reqwest = { git = "https://github.com/seanmonstar/reqwest", branch = "master" }

[dev-dependencies]
assert_cmd = "2"
proptest   = "1"

[build-dependencies]
# for build.rs scripts
```

Version requirements follow [SemVer caret](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html): `"1.2"` means `>=1.2.0, <2.0.0`.

## Features — conditional compilation

```toml
[features]
default = ["http"]
http = ["dep:reqwest"]
metrics = []
```

```rust
#[cfg(feature = "metrics")]
pub fn record_metric() { /* … */ }
```

Users enable with `cargo build --features metrics` or in their `Cargo.toml`:
```toml
my_crate = { version = "1", features = ["metrics"] }
```

## Workspaces — multiple related crates

Structure:
```
my_project/
├── Cargo.toml             # workspace manifest
├── Cargo.lock             # shared lockfile
├── cli/
│   ├── Cargo.toml
│   └── src/main.rs
├── core/
│   ├── Cargo.toml
│   └── src/lib.rs
└── storage/
    ├── Cargo.toml
    └── src/lib.rs
```

Top-level `Cargo.toml`:
```toml
[workspace]
resolver = "2"
members = ["cli", "core", "storage"]

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

Each member's `Cargo.toml`:
```toml
[package]
name = "cli"
version = "0.1.0"
edition = "2021"

[dependencies]
core    = { path = "../core" }
storage = { path = "../storage" }
serde   = { workspace = true }
```

Benefits: one shared lockfile, unified `cargo test`, dedicated-path dependencies, shared `[profile.*]` settings.

## Publishing to crates.io

```bash
cargo login        # API token from crates.io
cargo publish      # uploads the current package
```

Requirements: unique name, `license` + `description` + `repository` in Cargo.toml, all deps published on crates.io (no git/path deps in published versions).

## Related
- [[modules-and-paths|Modules & paths]]
- [[../07-testing/index|Testing]]
