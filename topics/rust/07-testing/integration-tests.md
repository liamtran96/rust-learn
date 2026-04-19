---
title: 7.2 Integration Tests
tags: [rust, testing]
---

# 7.2 Integration Tests

Integration tests live in a **top-level `tests/` directory** (sibling to `src/`). Each file is compiled as its own crate and uses your library through the public API only.

```
my_crate/
├── src/lib.rs
└── tests/
    ├── api.rs              # one crate
    ├── cli.rs              # another crate
    └── common/
        └── mod.rs          # shared helpers
```

## Example

```rust
// tests/api.rs
use my_crate::Engine;

#[test]
fn engine_starts() {
    let e = Engine::new();
    assert!(e.is_ready());
}
```

## Sharing setup across tests

Helpers go into submodules (not `tests/helpers.rs`, which would be its own test crate):

```rust
// tests/common/mod.rs
pub fn setup() { /* ... */ }
```

```rust
// tests/api.rs
mod common;

#[test]
fn with_setup() {
    common::setup();
    // ...
}
```

## Testing binaries with `assert_cmd`

```rust
// tests/cli.rs
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn greets() {
    Command::cargo_bin("my_app").unwrap()
        .arg("Alice")
        .assert()
        .success()
        .stdout(predicate::str::contains("hello Alice"));
}
```

## Related
- [[unit-tests|Unit tests]]
- [[doc-tests|Doc tests]]
