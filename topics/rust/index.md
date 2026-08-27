---
title: Rust Knowledge Base
tags: [rust, moc, index]
---

# 🦀 Rust Knowledge Base

> A structured curriculum for learning Rust effectively — from `cargo new` to `unsafe`.

## How to use this vault

1. Start with [[roadmap|the Roadmap]] to see the full learning arc.
2. Use [[study-plan|the Study Plan]] for a concrete week-by-week calendar at 1 hr/day.
3. Work through the numbered folders in order — each builds on the last.
4. After every chapter, do the linked [[exercises/index|exercises]] before moving on. **Active recall > re-reading.**
5. When stuck, consult the [[cheatsheets/index|cheatsheets]] and [[pitfalls|common pitfalls]].
6. Open [[journal|the journal index]], then use the matching chapter journal for confusions, aha-moments, and open questions.

## 🗺️ Curriculum

### Phase 1 — Foundations (Week 1–2)
- [[01-fundamentals/index|1. Fundamentals]] — toolchain, variables, types, functions, control flow
- [[02-ownership/index|2. Ownership, Borrowing & Lifetimes]] — Rust's core superpower

### Phase 2 — Building Blocks (Week 3–4)
- [[03-types-and-traits/index|3. Structs, Enums, Generics & Traits]]
- [[04-collections/index|4. Strings, Vec & HashMap]]
- [[05-error-handling/index|5. Error Handling — Result, Option, `?`]]

### Phase 3 — Organization & Quality (Week 5)
- [[06-modules/index|6. Modules, Crates, Workspaces]]
- [[07-testing/index|7. Testing & Documentation]]

### Phase 4 — Idiomatic Rust (Week 6–7)
- [[08-closures-iterators/index|8. Closures & Iterators]]
- [[09-smart-pointers/index|9. Smart Pointers — Box, Rc, Arc, RefCell]]

### Phase 5 — Systems & Scale (Week 8)
- [[10-concurrency/index|10. Concurrency — threads, channels, Send/Sync]]
- [[11-async/index|11. Async/Await & Tokio]]
- [[12-advanced/index|12. Advanced — unsafe, macros, FFI]]

### Phase 6 — Tauri Capstone (Weeks 9–12)
- [[13-tauri/index|13. Tauri — ship a real desktop app]]

### Phase 7 — Advanced Track C: Unsafe & Memory Model (Weeks 13–16)
- [[14-unsafe/index|14. Unsafe Rust & the Memory Model]] — *optional specialty track*

## 📚 Quick-reference

- [[cheatsheets/index|Cheatsheets]] — ownership rules, lifetime syntax, iterator methods, trait bounds
- [[pitfalls|Common Pitfalls]] — mistakes that bite every beginner
- [[exercises/index|Exercises & Projects]] — graded by difficulty
- [[resources/index|Learning Resources]] — books, videos, tools, communities

## 🎯 Goals & milestones

Track your progress — tick boxes as you master each level:

- [ ] **Beginner:** Write a CLI tool that reads a file, parses it, and writes output with proper error handling
- [ ] **Intermediate:** Build a REST API with `axum`/`actix-web` + `sqlx`, including tests
- [ ] **Capstone:** Ship a Tauri 2 desktop app — installer, signed bundle, persisted state ([[13-tauri/capstone|tracks]])
- [ ] **Advanced:** Write a concurrent async service with graceful shutdown, tracing, and back-pressure
- [ ] **Systems:** Implement a non-trivial data structure (`MyVec`, arena, intrusive list) with `unsafe`, Miri-clean ([[14-unsafe/index|Track C]])

## 🧠 Learning principles

- **Fight the borrow checker deliberately.** Every fight teaches you something about memory.
- **Read compiler errors fully.** Rust's errors are the best teacher in the ecosystem.
- **Prefer owned types first; add references once it compiles.**
- **`clippy` is your pair programmer.** Run `cargo clippy` on every project.
- **Read real code** — pick one small crate per week and read its source (start with [`anyhow`](https://docs.rs/anyhow) or [`once_cell`](https://docs.rs/once_cell)).
