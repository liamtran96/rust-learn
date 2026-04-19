---
title: Ch 10 — Concurrency Exercises
tags: [rust, exercises, concurrency]
---

# Ch 10 — Concurrency Exercises

1. **Parallel sum**: given `&[u64]`, split into N chunks and sum in parallel via `thread::scope`. Compare performance with single-thread.
2. **Worker pool**: 1 producer, 4 workers, 1 aggregator. Producer sends `Vec<u8>` messages over `mpsc`; each worker hashes (use `sha2` crate) and sends the result onward.
3. **Counter**: increment a shared `Arc<Mutex<u64>>` from 8 threads, 10 000 times each. Verify the final count is 80 000.
4. **Locking scope**: solve #3 with `Arc<Mutex<HashMap<String, u64>>>` — per-key increment. Measure how lock scope affects throughput.
5. **Poison recovery**: deliberately panic in one of the threads while holding the mutex. Demonstrate that other threads can still use the mutex via `err.into_inner()` recovery.
6. **Port scanner**: given a host and a port range, spawn threads to check `TcpStream::connect` with a timeout. Collect open ports.

## Read the docs for these APIs as you go
- `std::thread::scope`
- `std::sync::mpsc::sync_channel`
- `crossbeam_channel` (MPMC)
- `rayon::par_iter` — for "just parallelize this iterator"
