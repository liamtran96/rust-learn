# Owned Scanner exercise

## Goal and source

Complete [Ch 2 exercise 4](../../topics/rust/exercises/ch02-ownership.md): refactor the borrowed Scanner to own String and compare ownership choices. Source request: Liam's learning session on 2026-09-08; no external issue.

## Changes and flow

- `code/02-ownership/scanner/src/main.rs`: replace the borrowed field with String and remove the type/impl lifetime parameter. Main and all three tests construct owned input. `peek` borrows the field rather than moving it out through `&self`; `advance` retains the existing byte-cursor logic.
- `code/02-ownership/scanner/BRIEF.md`: record the owned variant while preserving the original borrowed brief.
- Progress, study plan, ownership journal, and mistake log record completion and the reviewed allocation/borrowing distinction.

An existing String can be moved into Scanner without another text allocation. The String::from calls in the demonstration and tests create their initial text buffers. Method behavior is unchanged: peek observes, advance returns the next character and updates the cursor, and end-of-text returns None.

## Verification and limits

From the scanner crate, `cargo fmt --check`, `cargo check`, `cargo test` (3 passed), and `cargo clippy -- -D warnings` passed on 2026-09-08. Tests cover ASCII peeking, advancing, and end-of-text. No new tests were added for this ownership refactor; multibyte input remains untested.

## Maintenance

Preserve the cursor's UTF-8 byte-boundary invariant when extending navigation. Borrow fields for read access rather than cloning to avoid ownership errors. Future coverage can add multibyte input and empty input. Chapter shipping work remains incomplete.
