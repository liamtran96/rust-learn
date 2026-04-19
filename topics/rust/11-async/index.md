---
title: 11. Async/Await
tags: [rust, async, tokio]
---

# 11. Async/Await

Rust's approach to high-concurrency IO: **stackless coroutines** compiled to state machines, executed on a runtime of your choice.

## Contents
- [[async-await|11.1 async/await fundamentals]]
- [[futures|11.2 Futures & tasks]]
- [[tokio|11.3 Tokio — the runtime you'll actually use]]

## Key ideas
- `async fn` returns an **opaque `Future`** — nothing runs yet.
- The `.await` operator drives a future forward, yielding control when it's blocked.
- Rust **doesn't ship a runtime** — bring your own (`tokio`, `async-std`, `smol`).
- **Tokio** dominates the ecosystem; learn it first.

## When to pick async vs threads

| Use async when | Use threads when |
|---|---|
| Lots of IO, not much CPU | CPU-bound work |
| Tens of thousands of concurrent ops | Dozens of parallel workers |
| Network servers, clients | Filesystem scans, compute |
| You're already on tokio/axum | You want simple code |

## Exit criteria
- [ ] You can write an `async fn` that awaits a Tokio sleep or HTTP call.
- [ ] You understand why `async` blocks are **lazy** and why forgetting `.await` is a common bug.
- [ ] You can use `tokio::select!` to race futures.
- [ ] You can use `tokio::spawn` to run a task concurrently and `await` its `JoinHandle`.
