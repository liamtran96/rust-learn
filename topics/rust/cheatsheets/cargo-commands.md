---
title: Cargo Commands Cheatsheet
tags: [rust, cheatsheet, cargo]
---

# Cargo Commands Cheatsheet

## Project setup

| Command | What |
|---|---|
| `cargo new <name>` | New binary crate |
| `cargo new --lib <name>` | New library crate |
| `cargo init` | Turn cwd into a crate |

## Build / run / check

| Command | What |
|---|---|
| `cargo check` | Type-check without codegen (fast) |
| `cargo build` | Debug build into `target/debug/` |
| `cargo build --release` | Optimized build into `target/release/` |
| `cargo run` | Build + run debug binary |
| `cargo run --release` | Build + run release binary |
| `cargo run --bin <name>` | Run specific binary |
| `cargo run --example <name>` | Run file from `examples/` |

## Dependencies

| Command | What |
|---|---|
| `cargo add <crate>` | Add to dependencies |
| `cargo add <crate> --features foo,bar` | With features |
| `cargo add <crate> --dev` | Dev-dependency |
| `cargo remove <crate>` | Remove |
| `cargo update` | Bump within semver |
| `cargo tree` | Dependency graph |
| `cargo outdated` | Show what's out of date (needs `cargo-outdated`) |

## Testing

| Command | What |
|---|---|
| `cargo test` | Run all tests |
| `cargo test <name>` | Match test name |
| `cargo test -- --nocapture` | Show `println!` output |
| `cargo test -- --test-threads=1` | Serial |
| `cargo test --release` | With optimizations |

## Quality

| Command | What |
|---|---|
| `cargo fmt` | Format |
| `cargo fmt --check` | Fail if formatting is wrong (CI) |
| `cargo clippy` | Lint |
| `cargo clippy -- -D warnings` | Fail on any lint (CI) |
| `cargo clippy --fix` | Apply safe fixes |

## Docs

| Command | What |
|---|---|
| `cargo doc --open` | Build + open |
| `cargo doc --no-deps` | Skip dep docs |

## Publishing

| Command | What |
|---|---|
| `cargo login` | Save crates.io API token |
| `cargo publish --dry-run` | Validate |
| `cargo publish` | Upload |
| `cargo yank --vers X.Y.Z` | Soft-delete |

## Handy extras (install via `cargo install`)

| Tool | What |
|---|---|
| `cargo-watch` | Re-run on file change: `cargo watch -x check -x test` |
| `cargo-nextest` | Much faster test runner |
| `cargo-edit` | Adds `add`/`rm` subcommands (now builtin but the crate adds extras) |
| `cargo-expand` | Show macro expansions |
| `cargo-audit` | Security advisories |
| `cargo-deny` | License/source checks |
| `cargo-flamegraph` | Profiling |
| `bacon` | Pleasant background checker |
