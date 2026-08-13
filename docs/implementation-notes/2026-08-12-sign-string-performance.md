# Sign string performance comparison

## Goal
Provide a readable experiment comparing `sign` returning a borrowed string literal with returning a newly allocated owned `String`.

## Affected files
- `code/01-fundamentals/sign-expression/examples/performance.rs`
- `code/01-fundamentals/sign-expression/benches/sign_performance.rs`
- `code/01-fundamentals/sign-expression/borrowed-vs-owned.html`
- `code/01-fundamentals/sign-expression/Cargo.toml`

## Implementation flow
The example warms up both functions, measures ten million calls over repeated rounds, takes the median duration, and reports total time, nanoseconds per call, and the relative ratio. A Criterion benchmark independently performs warm-up, sampling, and statistical analysis through `cargo bench`. A standalone interactive visual steps through borrowing, heap allocation, ownership transfer, and cleanup for one or one million calls, using a reusable memory-shelf analogy.

## Important decisions
The manual experiment uses only the standard library. `black_box` discourages optimization from deleting the work, alternating measurement order reduces bias, and an example target keeps the original exercise binary unchanged. Criterion is a development-only dependency and both benchmark cases cycle through all three branches.

## Verification
Run formatting, check, test, and Clippy from the exercise crate, then run `cargo run --release --example performance` and `cargo bench --bench sign_performance`.

## Maintenance guidance
Treat results as local measurements rather than cross-machine constants. Keep both functions behaviorally equivalent; Criterion automatically uses an optimized benchmark profile.

## Request
Conversation request on 2026-08-12 to see and run both performance cases.
