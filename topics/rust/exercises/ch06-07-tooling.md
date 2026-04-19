---
title: Ch 6–7 — Modules & Testing Exercises
tags: [rust, exercises, modules, testing]
---

# Ch 6–7 — Modules & Testing Exercises

## Modules

1. Take any single-file project you've written and split it into ≥3 modules. Keep the public API unchanged.
2. Make an internal helper `pub(crate)` and verify it's accessible inside the crate but not when someone uses your library.
3. Set up a workspace with two crates: `core` (library) and `cli` (binary). The cli depends on core. Share dependencies via `[workspace.dependencies]`.

## Testing

4. Write unit tests for your FizzBuzz solution (Ch 1). Include one `#[should_panic]` test and one `#[ignore]`d slow test.
5. Add an integration test in `tests/` that runs your binary with `assert_cmd` and checks stdout.
6. Turn three of your `pub fn`s into documented items with runnable doc tests. Verify `cargo test --doc` picks them up.
7. Add a table-driven test: a slice of `(input, expected)` tuples, one assertion loop.
8. Install `cargo-nextest` and run your tests with it. Note the speedup.
