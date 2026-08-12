# Codex repository instructions

## Goal

Add a root `AGENTS.md` that Codex can discover automatically when working in this repository. The request was made directly in the 2026-08-10 learning session; no external issue link is available.

## Affected files

- `AGENTS.md`
- `docs/root-causes/README.md`
- `docs/implementation-notes/2026-08-10-codex-agents-instructions.md`

## Implementation

The new instructions describe the repository layout, learner-led exercise workflow, completion records, Cargo verification, Rust review rules, and required documentation policy for future features and bug fixes.

The guidance is adapted to Codex instead of replacing `CLAUDE.md`, so both tools retain repository-specific instructions.

## Verification

- Confirm `AGENTS.md` exists at the repository root.
- Confirm it links only to paths that exist in this repository.
- Confirm the feature and bug-fix documentation rules name `docs/implementation-notes/` and `docs/root-causes/` respectively.

## Maintenance

Keep shared learning guidance aligned with `CLAUDE.md`. Put Codex-specific workflow changes in `AGENTS.md`, and document future material changes under `docs/implementation-notes/`.
