# Beginner-Friendly Exercise Briefs

## Goal
Make every future `$new-exercise` brief understandable before Liam is familiar with the Rust syntax, while preserving the rule that Liam writes the implementation.

## Affected files
- `.agents/skills/new-exercise/SKILL.md`

## Implementation flow and decisions
The brief template now requires a plain-language task description, a small behavioral example when unambiguous, a token-by-token syntax explanation, and incremental coding milestones. It explicitly prohibits complete or near-complete solutions and avoids introducing unlearned features for convenience.

## Verification
- `git diff --check` passed for the updated skill.
- Manually confirmed the required `name` and `description` frontmatter; the bundled validator could not run because no Python launcher is installed.
- Reviewed the template to ensure it separates syntax guidance from implementation choices.

## Maintenance
When extending the template, keep syntax explanations scoped to what the current exercise needs. Algorithm selection and implementation code must remain Liam's work.
