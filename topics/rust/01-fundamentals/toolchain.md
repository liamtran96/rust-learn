---
title: 1.1 Toolchain
tags: [rust, fundamentals, toolchain, cargo]
---

# 1.1 Toolchain

Rust ships a tight, opinionated toolchain. Learn these 5 commands and you're 80% of the way.

## Install

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This installs `rustup` (version manager), `cargo` (build tool & package manager), `rustc` (compiler), and the standard library.

## The 5 commands you'll use daily

| Command | What it does |
|---|---|
| `cargo new my_app` | Create a new binary project |
| `cargo new --lib my_lib` | Create a new library project |
| `cargo run` | Compile + run the binary crate |
| `cargo test` | Run all tests |
| `cargo check` | Type-check without producing a binary (fast) |

## The 3 that make you a pro

| Command | Why it matters |
|---|---|
| `cargo clippy` | Lints idiomatic issues. **Run on every save.** |
| `cargo fmt` | Canonical formatter. No bikesheds. |
| `cargo doc --open` | Generates and opens HTML docs for your crate + deps |

## Project anatomy

```
my_app/
├── Cargo.toml       # manifest: name, version, deps, features
├── Cargo.lock       # pinned dep versions — commit for binaries, not for libs (nuance — search modern guidance)
├── src/
│   ├── main.rs      # binary entry point
│   ├── lib.rs       # library entry point (if applicable)
│   └── bin/         # additional binaries
├── tests/           # integration tests (each file = separate crate)
├── examples/        # runnable examples (`cargo run --example foo`)
└── benches/         # benchmarks
```

## `Cargo.toml` essentials

```toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2021"  # editions = opt-in language changes; 2024 is newer

[dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }

[dev-dependencies]
assert_cmd = "2"

[profile.release]
lto = true         # link-time optimization
codegen-units = 1  # slower build, smaller/faster binary
```

## Toolchains & components via `rustup`

```bash
rustup update                  # get the latest stable
rustup install nightly         # get nightly (for unstable features)
rustup component add clippy rustfmt rust-analyzer
rustup target add wasm32-unknown-unknown  # cross-compile target
```

## Editor setup (non-negotiable)

Install **`rust-analyzer`** in your editor. It's the official LSP and is dramatically better than the old RLS. Inline hints, hover types, and quick-fixes make Rust *much* more learnable.

## Related
- [[../06-modules/crates-and-workspaces|Crates & workspaces]]
- [[../cheatsheets/cargo-commands|Cargo commands cheatsheet]]
