# `$next` unfamiliar-syntax guidance

## Goal

Make `$next` actively help when the selected Rust task requires syntax Liam has not learned yet, instead of only naming the next task and reference note.

## Affected file

- `.agents/skills/next/SKILL.md`

## Implementation

The skill now checks the progress log and relevant journal entries for the selected task's syntax. When syntax is unfamiliar, its response explains the real-world purpose, decodes the syntax, provides a small first typing step, and offers up to two graduated hints.

The guidance preserves the repository's teaching boundary: examples may clarify syntax, but Liam still makes the implementation decisions and writes the exercise solution unless he explicitly requests otherwise.

## Verification

- Run the skill validator against `.agents/skills/next`.
- Inspect the diff to confirm the existing source-of-truth and single-next-action rules remain intact.

## Maintenance

Keep syntax help proportional to the learning record. If a concept is already learned, prefer a short recall prompt; if it is new, explain it before asking Liam to use it.
