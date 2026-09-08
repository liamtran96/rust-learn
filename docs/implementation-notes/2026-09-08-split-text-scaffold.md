# Split-text exercise scaffold

## Goal and source

Liam approved scaffolding `$new-exercise 02 split-text` from the recorded next action. The source is `topics/rust/study-plan.md`, Week 3's split/dedup shipping bullet; the chapter exercise file has no separate splitting specification.

## Files and decisions

- `code/02-ownership/split-text/Cargo.toml` and `src/main.rs`: default Cargo binary scaffold; generated greeting retained.
- `code/02-ownership/split-text/BRIEF.md`: proposed single-character, borrowed-slice interface, examples, syntax explanations, and incremental learner milestones. Algorithm and implementation remain Liam's work.
- `bacon.toml`: check/run aliases with explicit crate paths and watches.

The brief makes the broad study-plan task actionable without introducing custom iterator traits. Splitting behavior is checked against official standard-library documentation linked in the brief. Completion counts and shipping checkboxes are unchanged.

## Verification

Cargo creation succeeded after retrying with normal repository access; the initial sandbox attempt left only empty directories, which were removed before retrying. `bacon --project . --list-jobs` passed and listed both aliases; `git diff --check` passed. Inspected the manifest and unchanged generated greeting. No exercise build or run was performed.

## Maintenance

Keep the aliases aligned if the crate moves. Extend the interface only after this version is completed; the separate dedup task remains open. Exercise correctness will be verified after Liam implements it.
