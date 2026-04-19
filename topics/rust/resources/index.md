---
title: Learning Resources
tags: [rust, resources]
---

# Learning Resources

## Books (free, online)

- **[The Rust Programming Language ("the book")](https://doc.rust-lang.org/book/)** — canonical tutorial. Start here.
- **[Rust by Example](https://doc.rust-lang.org/rust-by-example/)** — same topics, more code, less prose.
- **[Rustlings](https://github.com/rust-lang/rustlings)** — code-fix exercises; map 1:1 to the book.
- **[The Rustonomicon](https://doc.rust-lang.org/nomicon/)** — *unsafe* Rust. For after you're comfortable.
- **[Rust Reference](https://doc.rust-lang.org/reference/)** — language spec.
- **[The Cargo Book](https://doc.rust-lang.org/cargo/)**
- **[Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)** — concepts behind `async`.
- **[The Little Book of Rust Macros](https://veykril.github.io/tlborm/)** — macros in depth.

## Books (paid, excellent)

- **Rust for Rustaceans** — Jon Gjengset. Intermediate → advanced; read after the book.
- **Programming Rust** — Blandy, Orendorff, Tindall. Deep reference; good second book.
- **Zero To Production In Rust** — Luca Palmieri. Building a real web app with axum/sqlx/tower.
- **Rust Atomics and Locks** — Mara Bos. Low-level concurrency.

## Videos / talks / streams

- **Jon Gjengset** — [YouTube](https://www.youtube.com/@jonhoo). Deep dives, live coding.
- **Let's Get Rusty** — beginner-friendly channel.
- **No Boilerplate** — fast, opinionated Rust explainers.

## Interactive playgrounds

- [Rust Playground](https://play.rust-lang.org/) — paste & run without installing.
- [Godbolt](https://godbolt.org/) — see the generated assembly.

## Docs & ecosystem

- [docs.rs](https://docs.rs/) — every published crate's docs.
- [crates.io](https://crates.io/) — the registry.
- [lib.rs](https://lib.rs/) — better-curated crate listing.
- [Blessed.rs](https://blessed.rs/crates) — curated "what crate should I use" guide.
- [caniuse.rs](https://caniuse.rs/) — when a feature stabilized.

## Community

- [Rust Users Forum](https://users.rust-lang.org/) — friendly, patient, searchable.
- [r/rust](https://reddit.com/r/rust/) — news, discussion.
- [Rust Discord](https://discord.gg/rust-lang) — real-time help.
- [This Week in Rust](https://this-week-in-rust.org/) — weekly roundup of crates, posts, jobs.
- [Rust on Stack Overflow](https://stackoverflow.com/questions/tagged/rust)

## Tools you'll want

- `rust-analyzer` — LSP for your editor.
- `cargo-watch` — recompile on save.
- `cargo-nextest` — fast test runner.
- `cargo-audit` / `cargo-deny` — security & licensing.
- `cargo-expand` — see what macros expand to.
- `bacon` — nicer background-compile TUI.
- `just` — a better `make`.
