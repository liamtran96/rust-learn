---
title: Exercises & Projects
tags: [rust, exercises, projects]
---

# Exercises & Projects

Active practice beats re-reading. Do **at least one** exercise per chapter before continuing.

## Structured programs

- [Rustlings](https://github.com/rust-lang/rustlings) — 80+ tiny code-fix exercises, maps 1:1 to the Rust Book. **Run through this first.**
- [Exercism — Rust track](https://exercism.org/tracks/rust) — mentored exercises.
- [Advent of Code](https://adventofcode.com/) — daily puzzles, Rust is a community favorite.
- [Codewars](https://www.codewars.com/?language=rust) — katas.

## Exercises by chapter

- [[ch01-fundamentals|Ch 1 — Fundamentals exercises]]
- [[ch02-ownership|Ch 2 — Ownership exercises]]
- [[ch03-types|Ch 3 — Types & traits exercises]]
- [[ch04-collections|Ch 4 — Collections exercises]]
- [[ch05-errors|Ch 5 — Error handling exercises]]
- [[ch06-07-tooling|Ch 6–7 — Modules & testing exercises]]
- [[ch08-iterators|Ch 8 — Iterators exercises]]
- [[ch09-smart-pointers|Ch 9 — Smart pointers exercises]]
- [[ch10-concurrency|Ch 10 — Concurrency exercises]]
- [[ch11-async|Ch 11 — Async exercises]]
- [[ch13-tauri|Ch 13 — Tauri exercises (capstone ladder)]]
- [[ch14-unsafe|Ch 14 — Unsafe & memory model exercises (Miri-driven)]]

## Small projects (1–2 days each)

1. **Unit converter CLI** — currencies, lengths, temperatures. Practice: arg parsing, match, `Display`.
2. **Word frequency counter** — read stdin or a file, print top-N words. Practice: `HashMap`, iterators.
3. **TODO CLI** — add / list / complete / remove, persist to JSON. Practice: `serde`, filesystem, `Result`.
4. **Tiny HTTP server** — respond to `GET /hello` with plaintext. Practice: `std::net::TcpListener`, threads.
5. **Chat server (tokio)** — accept many clients, broadcast messages. Practice: async, `select!`, channels.
6. **Minimal grep** — implement the parts of `grep` (regex, line numbers, recursion). Practice: I/O, `clap`, errors.
7. **Key-value store with a file backend** — like a tiny Redis. Practice: serialization, locks.

## Bigger projects (1–2 weeks)

- A static-site generator (markdown → HTML).
- A REST API with `axum` + `sqlx` + tests.
- A Bencode / JSON parser written by hand (no crate).
- A toy interpreter for a Lisp or arithmetic language.
- Port a small C library to Rust.

## Reading real code

Pick one crate per week and read its source. Suggested progression:
1. [`anyhow`](https://github.com/dtolnay/anyhow) — small, idiomatic error crate.
2. [`once_cell`](https://github.com/matklad/once_cell) — clever use of `UnsafeCell`.
3. [`itertools`](https://github.com/rust-itertools/itertools) — iterator mastery.
4. [`clap`](https://github.com/clap-rs/clap) — large-scale derive-macro API design.
5. [`tokio`](https://github.com/tokio-rs/tokio) — when you're ready to see how runtimes work.
