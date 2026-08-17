# Retrieval homework command

## Goal

Add a repository-local `$homework` command that turns Liam's completed Rust learning record
into personalized retrieval practice without exposing answers or introducing future topics.

## Affected files

- `.agents/skills/homework/SKILL.md`
- `.agents/skills/homework/agents/openai.yaml`

Generated homework is written under `topics/rust/homework/` only when the command is invoked.

## Implementation flow

The skill treats `topics/rust/progress.md` as the learned-scope boundary, extracts prior
questions and confusion from `topics/rust/journal.md`, and prioritizes fresh entries from
chapter `mistakes.md` files. By default it creates six mixed questions: recall, prediction or
diagnosis, a small coding task, and transfer to a new example.

Homework contains answer spaces but no solutions, hints, or direct links to source answers.
It is saved with a dated, non-overwriting filename. A follow-up review mode grades reasoning
and gives a narrower retry before revealing a full answer.

## Important decisions

- Official progress limits the testable material; unchecked future notes are not evidence
  that a concept has been learned.
- Fresh and repeated mistakes receive the highest retrieval weight.
- Generating or reviewing homework does not itself change progress or journal records.
- The command preserves the repository's teaching rule that Liam writes solutions first.

## Verification

- Validate the skill metadata and folder structure.
- Inspect the skill for explicit scope, prioritization, no-answer, and non-overwrite rules.
- Confirm the UI prompt invokes `$homework` by name.

## Maintenance

Adjust the question mix or default timebox in `.agents/skills/homework/SKILL.md`. Keep
`progress.md` authoritative if new learning sources are added, and add those sources to the
selection steps rather than weakening the learned-scope check.
