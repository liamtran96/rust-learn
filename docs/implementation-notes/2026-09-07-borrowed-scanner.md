# Borrowed Scanner learning exercise

## Goal and source
Complete exercise 3 under Lifetime reasoning in `topics/rust/exercises/ch02-ownership.md`. This is a guided learning implementation; the owned-String variant is separate and unstarted.

## Files and flow
- `code/02-ownership/scanner/src/main.rs`: borrowed source, byte cursor, peek/advance methods, demonstration, and three tests.
- `code/02-ownership/scanner/Cargo.lock`: dependency-free binary crate lockfile.
- Crate `BRIEF.md`, progress, study plan, slices-and-lifetimes journal, and chapter mistakes: session completion and retrieval targets.

`peek` slices from the current cursor and reads the first char. `advance` matches that Option, moves by the char's UTF-8 width when present, and returns it; None leaves the cursor unchanged. The source remains borrowed, and the implementation allocates no text copy.

## Verification and maintenance
Formatting, cargo check, three tests, and strict Clippy passed. Tests cover ASCII peeking without movement, advancing, and staying at the end. The demonstration prints Some('r'), Some('r'), Some('u').

Direct slicing requires an in-range cursor on a UTF-8 boundary. The methods preserve this from cursor zero, but fields can be set directly within the module. Unicode and initially empty input lack dedicated tests. Preserve this invariant when extending the exercise; constructors, encapsulation, and the owned variant can be considered in subsequent lessons.
