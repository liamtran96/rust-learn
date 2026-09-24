# Ship: enum-driven state-machine library

The numbered Chapter 3 exercise is complete. This follow-up turns it into the
separate Week 4 shipping milestone without starting a new crate.

## Goal

Make the connection state machine reusable from outside `main.rs`. Keep invalid
state combinations unrepresentable, define the transition policy deliberately,
and verify that policy with focused tests.

## Warm-up (answer before editing)

1. Why can a `ConnectionState` value hold only one variant at a time?
2. What is moved when a transition method takes `self` rather than `&self`?
3. For one existing event, name the old state, event, and resulting state.

## Build requirements

- Put the reusable `ConnectionState` type and its transition methods in
  `src/lib.rs`.
- Expose only the type and methods that a caller needs.
- Keep `src/main.rs` as a small client of the library rather than duplicating
  the state-machine implementation.
- Write down your policy for events received in each state, then encode it. You
  decide whether an unexpected event preserves the state, changes it, or is
  represented another way using only concepts learned so far.
- Add focused tests for:
  - a successful connection path;
  - a failed connection path;
  - repeated connection attempts;
  - at least one transition whose behavior was not already tested.
- Do not use `.clone()` merely to avoid an ownership decision, and avoid
  `unwrap()`.

## New syntax

```rust
// src/lib.rs
pub enum ExampleState {
    Ready,
}

impl ExampleState {
    pub fn event(self) -> Self {
        Self::Ready
    }
}
```

- `pub enum` lets code outside the library name the enum and construct or match
  its variants.
- `pub fn` lets callers invoke that method. Methods without `pub` remain private
  to the library module.
- In `main.rs`, the package name `network-state` is written as the Rust crate
  name `network_state`, so a library item is reached through
  `network_state::...`.
- `src/lib.rs` is Cargo's conventional library entry point; `src/main.rs` is the
  binary entry point.

The example above only demonstrates visibility and crate layout. Choose and
write the actual connection transitions yourself.

## First coding step

Create `src/lib.rs`, move the enum and `impl` block into it, and add only the
`pub` markers needed for the existing `main.rs` demonstration to compile.

If the compiler reports a privacy or unresolved-name error, share its complete
output before changing the design.

## Done when

From `code/03-types-and-traits/network-state/`, all of these pass:

```text
cargo fmt --check
cargo check
cargo test
cargo clippy -- -D warnings
cargo run
```

You can also explain the transition policy, why the methods consume `self`, and
why the public API is smaller than the implementation. Then say `done` so the
shipping milestone can be recorded with `$journal`.

## Unblock notes

- Enums: `topics/rust/03-types-and-traits/enums.md`
- Pattern matching: `topics/rust/03-types-and-traits/pattern-matching.md`
- Ownership: `topics/rust/02-ownership/ownership.md`
