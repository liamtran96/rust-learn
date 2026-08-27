# Strip Margin Exercise Scaffold

## Goal
Prepare the Chapter 2 `strip_margin` exercise workspace without implementing Liam's solution.

## Affected files
- `code/02-ownership/strip-margin/`: default Cargo binary crate and exercise brief.
- `bacon.toml`: check and run aliases for the crate.

## Implementation flow and decisions
The crate was generated with the default `cargo new` binary template. `BRIEF.md` explains the function signature token by token, gives one behavioral example, and breaks the work into beginner-friendly milestones without providing implementation code. Liam subsequently replaced the generated stub with his completed implementation.

## Verification
- Validated the Bacon configuration by listing its jobs; no build or run command was executed.
- After implementation, verified `cargo fmt --check`, `cargo check`, `cargo test`, and `cargo clippy -- -D warnings`; the crate currently defines no unit tests.

## Maintenance
Keep the brief aligned with `topics/rust/exercises/ch02-ownership.md`. Update both Bacon jobs if the crate is renamed or moved.
