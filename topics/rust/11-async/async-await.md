---
title: 11.1 async/await Fundamentals
tags: [rust, async]
---

# 11.1 async/await Fundamentals

## Syntax

```rust
async fn fetch(url: &str) -> reqwest::Result<String> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}
```

- `async fn` desugars to a normal function returning `impl Future<Output = ...>`.
- `.await` suspends the current future until the awaited one is ready.
- Inside async, use `?` just like in sync code.

## `async` blocks

Make any expression async without defining a function:

```rust
let fut = async { 1 + 2 };
let n = fut.await;
```

## Futures are lazy

**Nothing runs** until you await (or spawn) the future:

```rust
async fn do_work() { println!("hi"); }

let _f = do_work();       // "hi" NOT printed — future just sits there
```

This is Rust's #1 async gotcha. If your async code "does nothing", you probably forgot to `.await` or `spawn`.

## Running async from sync — you need a runtime

```rust
#[tokio::main]
async fn main() {
    println!("{}", fetch("https://example.com").await.unwrap());
}
```

`#[tokio::main]` is a macro that:
1. Builds a runtime.
2. Calls your `async fn main()` on it.

Without a runtime, nothing executes. You can also construct one explicitly:

```rust
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async_main());
}
```

## Running multiple futures concurrently

```rust
// Sequential — one after the other
let a = fetch("a").await?;
let b = fetch("b").await?;

// Concurrent — same task, interleaved
let (a, b) = tokio::join!(fetch("a"), fetch("b"));

// Concurrent, on their own tasks (may use multiple cores)
let ta = tokio::spawn(fetch("a".to_string()));
let tb = tokio::spawn(fetch("b".to_string()));
let (a, b) = (ta.await??, tb.await??);
```

## `Send` in async

Most executors (including tokio's multi-thread runtime) schedule tasks across worker threads, so your future and everything it holds must be `Send`. This is why you'll see bounds like `F: Future + Send + 'static` in spawn signatures.

## Async traits

As of Rust 1.75, you can use `async fn` directly in traits. Before that, the [`async-trait`](https://crates.io/crates/async-trait) crate was standard. Either works.

## Related
- [[futures|Futures & tasks]]
- [[tokio|Tokio]]
