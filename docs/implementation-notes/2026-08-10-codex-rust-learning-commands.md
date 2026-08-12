# Repository Codex Rust learning commands

## Goal

Expose the repository's three Rust-learning workflows as repo-scoped Codex skills rather than deprecated global custom prompts.

Source request: user request in the Codex session on 2026-08-10.

## Affected files

- `.agents/skills/journal/SKILL.md`
- `.agents/skills/journal/agents/openai.yaml`
- `.agents/skills/new-exercise/SKILL.md`
- `.agents/skills/new-exercise/agents/openai.yaml`
- `.agents/skills/next/SKILL.md`
- `.agents/skills/next/agents/openai.yaml`
- `docs/implementation-notes/2026-08-10-codex-rust-learning-commands.md`

The original `.claude/commands/*.md` files remain unchanged for Claude Code.

## Implementation flow

Codex discovers repository skills from `.agents/skills/` beneath the repository root. Each workflow has a concise `SKILL.md` and UI metadata in `agents/openai.yaml`.

Invoke them inside this repository as:

- `$journal`
- `$new-exercise 01 guessing-game`
- `$next`

The skill instructions use paths relative to the repository root. This replaces the old `~/Workspaces/rust-learn` path, which did not match this checkout at `C:\Users\LiamTran\orca\rust-learn`.

## Important decisions

- Scope is repository-only; these skills should not appear in unrelated repositories.
- Skills replace deprecated `/prompts:*` custom prompts and use current `$skill-name` invocation.
- Exercise scaffolding preserves the learning rule: Codex creates the crate and brief but does not write the solution.
- The obsolete global prompt copies under `C:\Users\LiamTran\.codex\prompts` were removed after the repo skills were validated.

## Verification

1. Validate each skill's YAML front matter and folder name.
2. Confirm every `agents/openai.yaml` includes a display name, 25–64 character short description, and a default prompt containing its `$skill-name`.
3. Search the new skills for obsolete `/prompts:*`, `/journal`, `/new-exercise`, and `~/Workspaces/rust-learn` references.
4. Confirm the three matching files no longer exist under `C:\Users\LiamTran\.codex\prompts`.
5. Start or reload a Codex session in this repository and confirm `$journal`, `$new-exercise`, and `$next` appear in the skill picker.

## Maintenance

Keep the matching `.claude/commands/` and `.agents/skills/` workflows aligned when behavior changes. Prefer repository-relative paths so the skills continue working when the checkout moves.

## Remaining risk

Codex may need a new or reloaded session before the newly created repo skills appear in the skill picker.
