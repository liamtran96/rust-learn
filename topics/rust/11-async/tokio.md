---
title: 11.3 Tokio
tags: [rust, async, tokio]
---

# 11.3 Tokio

The de-facto async runtime. Almost every async crate you'll use (axum, reqwest, sqlx, tonic, tower) expects a tokio runtime.

## Minimum setup

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

For production, replace `"full"` with only the features you need — `rt-multi-thread`, `macros`, `io-util`, `net`, `time`, `sync`, etc.

## Building blocks

```rust
use tokio::time::{sleep, Duration, timeout};
use tokio::sync::{mpsc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
```

- `tokio::time` — sleep, timeout, interval
- `tokio::sync` — async channels (`mpsc`, `broadcast`, `watch`, `oneshot`), async locks (`Mutex`, `RwLock`, `Semaphore`)
- `tokio::io` — async `Read`/`Write` traits + helpers
- `tokio::net` — TCP/UDP/Unix
- `tokio::fs` — async filesystem (thin wrapper, uses a blocking pool)
- `tokio::process` — spawn subprocesses

## A tiny server

```rust
use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (mut sock, _) = listener.accept().await?;
        tokio::spawn(async move {
            let _ = sock.write_all(b"hello\n").await;
        });
    }
}
```

## `tokio::sync::Mutex` vs `std::sync::Mutex`

- `std::sync::Mutex` — blocks the thread. Fast for short critical sections. Preferred **if** you never hold it across an `.await`.
- `tokio::sync::Mutex` — yields when contended. Required if the critical section contains an `.await`.

Most of the time `std::sync::Mutex` is the right choice — keep critical sections short.

## Channels

- `mpsc::channel(cap)` — multi-producer, single-consumer, bounded.
- `broadcast::channel(cap)` — multi-producer, multi-consumer, each receiver gets every message.
- `watch::channel(init)` — latest-value broadcast, receivers see only the most recent.
- `oneshot::channel()` — one-shot single value (RPC-style).

## Graceful shutdown pattern

```rust
use tokio::signal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let shutdown = async { signal::ctrl_c().await.ok(); };
    tokio::select! {
        _ = server_loop() => {},
        _ = shutdown => println!("shutting down"),
    }
    Ok(())
}
```

## Blocking work

Don't call blocking code (heavy CPU, sync IO) directly in async tasks — it stalls the whole worker thread. Offload:

```rust
let result = tokio::task::spawn_blocking(|| heavy_cpu_work()).await?;
```

## Ecosystem pointers

- **Web**: `axum` (tower-based), `actix-web`, `warp`, `rocket`
- **HTTP client**: `reqwest`
- **Database**: `sqlx`, `sea-orm`, `diesel-async`
- **gRPC**: `tonic`
- **Tracing**: `tracing` + `tracing-subscriber`
- **Observability**: `metrics`, `opentelemetry-otlp`

## Related
- [[async-await|async/await]]
- [[futures|Futures]]
