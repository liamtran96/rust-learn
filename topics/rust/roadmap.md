---
title: Rust Learning Roadmap
tags: [rust, roadmap]
---

# Rust Learning Roadmap

A pragmatic path from zero to shipping Rust. Estimated ~8 weeks at ~1 hr/day, or ~4 weeks full-time.

## Week 1 — Toolchain & syntax
- Install `rustup`, learn `cargo` basics ([[01-fundamentals/toolchain]])
- Variables, shadowing, mutability, constants
- Primitives, tuples, arrays
- Functions, expressions vs statements
- Control flow: `if`/`else`, `loop`, `while`, `for`
- **Milestone:** ✅ FizzBuzz · ⬜ temperature converter · ⬜ guessing game.

## Week 2 — Ownership (THE chapter)
- [[02-ownership/ownership|Ownership rules]] — move, copy, drop
- [[02-ownership/borrowing|References & borrowing]] — `&T` vs `&mut T`
- [[02-ownership/slices|Slices]] — `&str`, `&[T]`
- [[02-ownership/lifetimes|Lifetimes]] — the scary part that clicks once you see why
- **Milestone:** Re-implement `str::split`, `Vec::dedup` by hand.

## Week 3 — Types, enums, pattern matching
- [[03-types-and-traits/structs|Structs]] (including tuple structs, unit structs)
- [[03-types-and-traits/enums|Enums]] — the best feature in the language
- [[03-types-and-traits/pattern-matching|Pattern matching]] & `if let` / `while let`
- [[03-types-and-traits/generics|Generics]]
- [[03-types-and-traits/traits|Traits]] & `impl` blocks
- **Milestone:** A state-machine library (`enum`-driven).

## Week 4 — Collections & error handling
- [[04-collections/strings|String]] vs `&str` — when to use which
- [[04-collections/vec|Vec<T>]] — the workhorse
- [[04-collections/hashmap|HashMap<K, V>]]
- [[05-error-handling/result-option|Result<T, E> / Option<T>]]
- [[05-error-handling/question-mark|The `?` operator]]
- [[05-error-handling/custom-errors|Custom error types]] — `thiserror`, `anyhow`
- **Milestone:** A word-frequency CLI with graceful error handling.

## Week 5 — Organization & testing
- [[06-modules/modules-and-paths|Modules, `use`, visibility]]
- [[06-modules/crates-and-workspaces|Crates & workspaces]]
- [[07-testing/unit-tests|Unit tests]], [[07-testing/integration-tests|integration tests]], [[07-testing/doc-tests|doc tests]]
- `rustdoc` basics
- **Milestone:** Publish a (private or public) crate with docs and CI.

## Week 6 — Closures & iterators
- [[08-closures-iterators/closures|Closures]] — `Fn`, `FnMut`, `FnOnce`
- [[08-closures-iterators/iterators|Iterators]] — the lazy pipeline pattern
- Common adapters: `map`, `filter`, `collect`, `fold`, `zip`, `chain`, `flat_map`
- **Milestone:** Re-solve earlier exercises using only iterator chains — no explicit loops.

## Week 7 — Smart pointers & interior mutability
- [[09-smart-pointers/box|Box<T>]] — heap allocation, recursion
- [[09-smart-pointers/rc-arc|Rc<T> / Arc<T>]] — shared ownership
- [[09-smart-pointers/refcell|RefCell<T> / Cell<T>]] — interior mutability
- `Deref`, `DerefMut`, `Drop`
- **Milestone:** Implement a tree that supports parent pointers (forces you to grapple with `Rc`/`Weak`).

## Week 8 — Concurrency & async
- [[10-concurrency/threads|std::thread]]
- [[10-concurrency/channels|Channels]] — `mpsc`
- [[10-concurrency/shared-state|Mutex<T> / RwLock<T> with Arc]]
- [[10-concurrency/send-sync|Send, Sync]]
- [[11-async/async-await|async/await fundamentals]]
- [[11-async/tokio|Tokio runtime basics]]
- **Milestone:** A concurrent port scanner (threads) and an async chat server (tokio).

## Going deeper (Weeks 9+)
- [[12-advanced/unsafe|Unsafe Rust]] — raw pointers, FFI
- [[12-advanced/macros|Macros]] — `macro_rules!` first, proc macros later
- Domain-specific stacks: **web** (axum/actix), **CLI** (clap), **embedded** (embassy/no_std), **systems** (nix, libc)

## Ongoing habits
- Daily: read the compiler's suggestions end-to-end, don't skim.
- Weekly: one [Rustlings](https://github.com/rust-lang/rustlings) or [Exercism](https://exercism.org/tracks/rust) set.
- Monthly: read one chapter of [The Rustonomicon](https://doc.rust-lang.org/nomicon/) or [Rust for Rustaceans](https://rust-for-rustaceans.com/).
