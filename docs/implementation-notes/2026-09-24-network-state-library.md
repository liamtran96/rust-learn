# Network state-machine library

## Goal

Turn the completed network-state binary exercise into a reusable enum-driven
library while retaining a small executable demonstration.

## Affected files

- `code/03-types-and-traits/network-state/src/lib.rs`
- `code/03-types-and-traits/network-state/src/main.rs`
- `code/03-types-and-traits/network-state/SHIP-BRIEF.md`

## Implementation flow

`ConnectionState` and its consuming transition methods now live in `src/lib.rs`
as public API. The binary imports the type through
`use network_state::ConnectionState`, demonstrates success and failure paths,
and keeps focused transition tests.

The added retry test documents the existing wildcard policy:
`Failed + on_connect_attempt() -> Connecting { attempts: 1 }`.

## Decisions

- Keep consuming `self` methods because each event replaces the old state.
- Preserve the existing transition behavior rather than redesigning the API.
- Keep the executable as a library client so visibility and crate paths are
  exercised directly.

## Verification

Run from `code/03-types-and-traits/network-state/`:

```text
cargo fmt --check
cargo check
cargo test
cargo clippy -- -D warnings
cargo run
```

On 2026-09-24, formatting, compilation, four tests, strict Clippy, and the
runtime demonstration passed.

## Maintenance

When adding an event or variant, state the intended old-state/event/new-state
policy and add a focused test. Review wildcard match arms carefully because a
new variant will also follow them unless handled explicitly.
