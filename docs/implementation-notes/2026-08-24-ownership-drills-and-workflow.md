# 2026-08-24 — Ownership drills crate and repo session workflow

## Goal

Shift sessions toward code-first learning without losing understanding: turn the read-only
"Predict & fix" snippets from `topics/rust/exercises/ch02-ownership.md` into compiler-graded
drills, and document the repo-wide session routine so every agent guides sessions the same way.

## Affected files

- `code/02-ownership/drills-ownership/` — new lib crate (edition 2024). 12 drills as individual
  files under `tests/` (each compiles as its own target, so one broken drill never blocks another).
  9 fix-it drills ship with intentional compile errors (E0382, E0502, E0499, E0308, E0106);
  3 implement drills ship with `todo!()` bodies and red tests. Each file carries `PREDICT:` and
  `WHY:` lines Liam fills himself. `BRIEF.md` (rules) and `WORKFLOW.md` (drill loop) included.
- `WORKFLOW.md` (repo root) — the daily session routine: pick up → retrieval warm-up →
  code-first work → close out; weekly review; command cheat-sheets; ground rules.
- `AGENTS.md`, `CLAUDE.md` — new "Session workflow" sections pointing at the workflow files and
  forbidding agents from filling PREDICT/WHY lines, fixing drill code, or revealing intended fixes.
- `bacon.toml` — `drills-ownership` (check) and `drills-ownership-test` (test) jobs.
- `topics/rust/exercises/ch02-ownership.md` — pointer from "Predict & fix" to the drills crate.

## Implementation flow

Drill = one integration-test file, graded by `cargo test --test dNN`. The loop:
fill `PREDICT:` before running → run one drill → read the error → minimal fix
(no unjustified `.clone()`) → fill `WHY:` before the next drill. Finish line: all 12 green,
clippy `-D warnings` clean, then the ch02 checkpoint explained without notes.

## Decisions

- `tests/` per-file targets instead of feature flags or commented-out modules — separate
  compilation is what lets broken drills coexist.
- Lib crate (deviates from the `$new-exercise` bin default) — there is nothing to `run`;
  the bacon `-run` job is replaced by a `-test` job.
- One drills crate now, no `$drill` skill yet; the format is repeatable per chapter and a
  skill can automate it later if the format sticks.

## Verification

All 9 fix-it drills confirmed to fail with exactly the intended error codes; all 3 implement
drills compile and fail red on `todo!()`; `bacon --project . --list-jobs` shows both new jobs.

## Maintenance

To add a chapter's drills, copy the crate shape (`drills-<topic>`, drills in `tests/`,
BRIEF + WORKFLOW, two bacon jobs) and source the drill content from that chapter's exercise
file and `pitfalls.md`. Verify each fix-it drill's intended error before shipping it.
