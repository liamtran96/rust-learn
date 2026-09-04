# Completed-homework reference answers

## Goal

After Liam completes an entire numbered homework question, always provide a concise
canonical answer for comparison and future review.

## Affected files

- .agents/skills/homework/SKILL.md
- topics/rust/homework/2026-08-28-retrieval-02-ownership.md
- docs/implementation-notes/2026-09-04-homework-reference-answers.md

## Implementation flow

The homework review loop continues to grade attempts and use narrower follow-ups for
retries. Once the complete numbered question is correct, it presents a clearly labeled
reference answer and saves that answer alongside the tracked attempts. Attempts remain
uncommitted during the question; the complete review is committed once after the reference
answer is saved.

## Important decisions

- Reference answers appear only after the complete question is correct, preserving
  retrieval practice.
- Learner attempts remain unchanged so progress in reasoning stays visible.
- The reference answers cover the original prompt, not only the final follow-up.
- Individual retries are not committed. One focused commit records the entire completed
  numbered question, while unrelated working-tree changes remain unstaged.

## Verification

- Confirmed the rule follows the existing retry and answer-tracking steps.
- Backfilled Question 1 with a complete ownership reference answer after it was marked
  correct.
- git diff --check passed.

## Maintenance

Keep reference answers concise and verify Rust claims against official Rust documentation.
Do not reveal or save a reference answer while a question is still marked partly correct
or retry. Do not commit a tracked answer until its entire numbered question is correct.

## Source request

Liam's request on 2026-09-04 to always receive a final reference answer after completing
each whole homework question, and to commit only after the whole question is finished.
