---
title: Ch 11 — Async Exercises
tags: [rust, exercises, async, tokio]
---

# Ch 11 — Async Exercises

1. **Sleep concurrently**: create 10 tasks that each sleep 1 second, awaited with `join_all` from `futures::future`. Verify total wall time is ~1 s (not 10 s).
2. **Fetch with timeout**: make an HTTP GET with `reqwest`, wrapped in `tokio::time::timeout(Duration::from_secs(5), ...)`.
3. **Fan-in**: spawn 5 producer tasks all sending to one `mpsc::channel`, plus one consumer task that prints messages as they arrive.
4. **`select!` timeout + signal**: a server loop that accepts TcpStreams, cancelled on SIGINT via `tokio::select!`.
5. **Broadcast**: 1 producer, 3 subscribers via `tokio::sync::broadcast::channel`. Verify each subscriber sees every message.
6. **Shared state**: 10 tasks each bumping `Arc<Mutex<u64>>` N times. Compare `std::sync::Mutex` vs `tokio::sync::Mutex` (use `std::sync` unless you hold across an await).
7. **Rate-limited fetches**: given 100 URLs, fetch them with at most 10 in flight, using `tokio::sync::Semaphore`.
8. **Graceful shutdown**: build a tiny HTTP server (`axum`) that stops accepting new connections on `ctrl_c` but waits for in-flight responses.

## Common gotcha to watch for
Forgetting `.await`. Your compiler warns about unused `Future`s — always fix, never `#[allow]`.
