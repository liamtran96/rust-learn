---
title: 7. Testing & Documentation
tags: [rust, testing, docs]
---

# 7. Testing & Documentation

Rust's built-in test runner is first-class. There's no "should I use JUnit or TestNG" debate.

## Contents
- [[unit-tests|7.1 Unit tests]]
- [[integration-tests|7.2 Integration tests]]
- [[doc-tests|7.3 Doc tests]]

## Commands
- `cargo test` — run all tests.
- `cargo test foo` — run tests whose name contains `foo`.
- `cargo test -- --nocapture` — show `println!` output.
- `cargo test -- --test-threads=1` — serialize (for tests touching shared resources).

## Testing philosophy

- **Unit tests** live *next to the code*, in a `#[cfg(test)] mod tests` block. They can test private items.
- **Integration tests** live in `tests/` at the workspace root. They only see the public API — which is how *users* see it.
- **Doc tests** are examples in doc comments. They're compiled and run — documentation that *can't lie*.

## Exit criteria
- [ ] You can write a `#[test]` function, including `#[should_panic]` and `#[ignore]`.
- [ ] You can set up an integration test in `tests/`.
- [ ] You can write a `///` doc comment with a runnable example.
- [ ] You know `cargo test -- --list` and `-- --nocapture`.
