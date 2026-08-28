# Explain unfamiliar Rust before implementation

## Goal

Persist Liam's request that new Rust syntax be explained before he is asked to use it, without taking implementation work away from him.

## Affected files

- `AGENTS.md`
- `topics/rust/journal/02-ownership/slices-and-lifetimes.md`

## Implementation flow

The teaching rules now require three pieces of context before unfamiliar syntax is used in an exercise: what the code is for, how it appears in a real application, and a line-by-line syntax explanation. The agent must use the recorded progress and journal as the learned-material boundary instead of waiting for Liam to repeat that syntax is unfamiliar.

## Decisions

- Liam still writes exercise and drill implementations.
- Explanations precede implementation only for syntax not already recorded as learned.
- Compiler output remains teaching material after Liam attempts the code.

## Verification

- Reviewed the new rule in `AGENTS.md` against the existing code-first and no-complete-solutions rules.
- Recorded the request and its intended application in the 2026-08-28 ownership journal entry.

## Maintenance

When new syntax becomes familiar, record that outcome in the journal. Future agents should continue explaining genuinely new syntax while keeping explanations shorter for concepts already demonstrated successfully.
