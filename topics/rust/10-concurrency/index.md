---
title: 10. Concurrency
tags: [rust, concurrency]
---

# 10. Concurrency

"Fearless concurrency" — the type system catches data races at compile time.

## Contents
- [[threads|10.1 Threads]]
- [[channels|10.2 Channels — message passing]]
- [[shared-state|10.3 Shared state — Mutex, RwLock]]
- [[send-sync|10.4 Send & Sync marker traits]]

## Key ideas
- Two primary styles: **message passing** (channels) and **shared state** (locks). Rust supports both well.
- `Send` and `Sync` are **auto-derived** marker traits the compiler uses to decide what's safe to cross thread boundaries.
- Prefer channels for decoupled pipelines; prefer `Arc<Mutex<T>>` for hot shared state.
- The threading APIs are portable and blocking. For massive concurrency (many thousands of tasks) use [[../11-async/index|async/await]] instead.

## Exit criteria
- [ ] You can spawn a thread, `move` data into it, and `join` it.
- [ ] You can build a producer/consumer with `mpsc::channel`.
- [ ] You can wrap shared state in `Arc<Mutex<T>>` and reason about lock scope.
- [ ] You can explain what it means for a type to be `Send` vs `Sync` and name types that are only one.
