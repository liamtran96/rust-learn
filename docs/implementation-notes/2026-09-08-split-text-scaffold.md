# Split-text exercise implementation

## Goal and source

Liam approved scaffolding `$new-exercise 02 split-text` from the recorded next action. The source is `topics/rust/study-plan.md`, Week 3's split/dedup shipping bullet; the chapter exercise file has no separate splitting specification.

## Files and decisions

- `code/02-ownership/split-text/Cargo.toml` and `src/main.rs`: binary crate with Liam's hand-written splitter, a Unicode runtime example, and four unit tests.
- `code/02-ownership/split-text/BRIEF.md`: single-character, borrowed-slice interface, examples, syntax explanations, milestones, and completion status.
- `bacon.toml`: check/run aliases with explicit crate paths and watches.

The function walks `char_indices`, uses byte positions only at valid UTF-8 boundaries, and advances by `char::len_utf8`. It returns `Vec<&str>`, so pieces reuse the input text rather than allocating owned strings. The scoped behavior matches single-character `str::split`, including empty pieces at adjacent, leading, and trailing separators. The combined split/dedup shipping checkbox stays open until dedup is complete.

## Verification

`cargo fmt --check`, `cargo check`, `cargo test` (4 passed), `cargo clippy -- -D warnings`, and `cargo run` passed. Runtime output was `["red", "blue", "green"]` for a multibyte emoji separator. Tests cover normal splitting, adjacent/edge separators, missing separator and empty input, and a Unicode separator.

## Maintenance

Keep the aliases aligned if the crate moves. A later extension could return a custom lazy iterator or accept richer separator patterns; retain `char_indices` or another boundary-aware API for UTF-8 safety. The separate dedup task remains open.
