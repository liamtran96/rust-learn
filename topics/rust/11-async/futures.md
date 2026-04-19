---
title: 11.2 Futures & Tasks
tags: [rust, async, futures]
---

# 11.2 Futures & Tasks

## The `Future` trait

```rust
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

enum Poll<T> { Ready(T), Pending }
```

- A future's `poll` is called by the runtime. It either returns `Ready(value)` or `Pending`.
- When `Pending`, the future registers the current task's **waker** with whatever it's waiting on (socket, timer, channel). When ready, the waker notifies the runtime to poll again.

You rarely implement `Future` by hand — the `async` keyword generates the state machine for you.

## Tasks

A **task** is a top-level future scheduled on the runtime. `tokio::spawn(fut)` creates one and returns a `JoinHandle<T>`:

```rust
let handle = tokio::spawn(async {
    do_work().await;
    42
});
let result = handle.await?;     // 42
```

A task runs concurrently with the spawner. If the spawner drops the handle, the task still runs (unlike threads, which detach the same way).

## `select!` — race futures

```rust
use tokio::time::{sleep, Duration};

tokio::select! {
    _ = sleep(Duration::from_secs(1)) => println!("timeout"),
    msg = rx.recv() => println!("got {msg:?}"),
}
```

Polls all branches, completes when the first one does. Useful for timeouts, graceful shutdown, multiplexing channels.

## Cancellation

Dropping a future cancels it — no destructor runs on the awaited resources beyond normal Rust drops. This is elegant but surprising: async cancellation is sometimes called *"cancel-safe by structure"*.

Gotcha: if a future was mid-operation when dropped (e.g., writing to a socket), the operation may be partially complete. For cancel-safe primitives, look at tokio's docs for each API.

## `Pin<&mut T>` — the weird bit

Async state machines can be **self-referential** — an `.await` may produce a future that holds a pointer into an earlier local. Moving such a value would dangle the pointer, so they must be **pinned**.

You'll hit `Pin` occasionally. For now: boxing (`Box::pin(fut)`) or `tokio::pin!(fut)` are the common escape hatches when a signature demands it.

## Related
- [[async-await|async/await basics]]
- [[tokio|Tokio]]
