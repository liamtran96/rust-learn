# `$next` progress fidelity

## Goal

Prevent `$next` from replacing the status recorded in `topics/rust/progress.md` with an
independent assessment based on specifications, crate contents, or stale study-plan boxes.

## Affected file

- `.agents/skills/next/SKILL.md`

## Implementation

- Made `progress.md` an explicit source-of-truth invariant.
- Prohibited recalculating or changing recorded exercise and project completion.
- Classified unfinished variants, cleanup, and spec differences as follow-up work unless
  `progress.md` changes their status.
- Added a final status-fidelity check before responding.
- Renamed `Remaining this week` to `Study-plan boxes still unchecked` so planning metadata
  is not presented as official progress.

## Before and after

Before, a `7 / 7` status could be recomputed as `5 / 7` after inspecting follow-up work.
After this change, `$next` must preserve `7 / 7` and describe that work separately.

## Verification

- Validate the skill metadata and structure with the skill validator.
- Re-run the prior scenario: official status stays `7 / 7`; iterator-style FizzBuzz remains
  the next recorded follow-up.

## Maintenance

Change completion in `topics/rust/progress.md` first. Other curriculum files may inform the
next action but must not silently override that record.

## Remaining risk

This is instruction-driven rather than executable enforcement. The explicit invariant and
final fidelity check reduce, but cannot mathematically eliminate, model error.
