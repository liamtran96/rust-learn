---
title: 10.2 Channels
tags: [rust, channels, concurrency]
---

# 10.2 Channels

*"Don't communicate by sharing memory; share memory by communicating."* Rust's `std::sync::mpsc` gives you multi-producer, single-consumer channels. For more flavors, reach for [`crossbeam`](https://docs.rs/crossbeam) or [`tokio::sync`](https://docs.rs/tokio).

## Basic usage

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send("hello from thread").unwrap();
});

let msg = rx.recv().unwrap();
println!("{msg}");
```

## Multi-producer

```rust
let (tx, rx) = mpsc::channel();

for i in 0..4 {
    let tx = tx.clone();         // each thread gets its own sender handle
    thread::spawn(move || tx.send(i).unwrap());
}
drop(tx);                         // important: drop the original so recv knows we're done

for msg in rx {                   // iter stops when all senders drop
    println!("{msg}");
}
```

## Bounded channels

`mpsc::sync_channel(n)` gives back-pressure — `send` blocks when the buffer is full.

```rust
let (tx, rx) = mpsc::sync_channel(10);
```

## Crossbeam MPMC

`std::sync::mpsc` is single-consumer only. For multi-consumer use `crossbeam_channel`:

```rust
let (tx, rx) = crossbeam_channel::unbounded();
let rx2 = rx.clone();            // multiple receivers — MPMC
```

## Pattern: pipeline

```
[producer] --tx--> [worker pool] --tx--> [aggregator]
```

Build N channels, spawn threads between them, let the runtime orchestrate.

## Related
- [[threads|Threads]]
- [[shared-state|Shared state (when channels aren't the right tool)]]
