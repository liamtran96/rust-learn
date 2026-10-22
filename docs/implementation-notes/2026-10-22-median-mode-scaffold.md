# Median and mode exercise scaffold

## Goal and source
Liam requested the `median-mode` scaffold in chat. Source: `topics/rust/exercises/ch04-collections.md`, exercise 1 (Median & mode).

## Affected files and flow
- `code/04-collections/median-mode/Cargo.toml` and `src/main.rs`: default binary crate from `cargo new`.
- `code/04-collections/median-mode/BRIEF.md`: expected function input/output, decoded syntax, incremental milestones, and references.
- `bacon.toml`: `median-mode` check and `median-mode-run` run jobs, scoped to this crate.

## Decisions
Use the chapter-matching directory `04-collections`. Preserve generated Hello World source; Liam implements every exercise line. The brief identifies tied modes and empty input as unspecified policies to decide, and starts with non-empty, unique-mode examples. Completion records remain unchanged.

## Verification
`cargo new` succeeded after clearing its empty partial directory and retrying with filesystem access. Inspected the generated manifest and unchanged Hello World source. `bacon --project . --list-jobs` validates both aliases; `git diff --check` checks whitespace. No exercise build, run, or tests are part of scaffolding.

## Maintenance
Keep Bacon workdir/watch paths aligned if the crate moves. Extend the brief when behavior policies are chosen, without adding the learner's solution.
