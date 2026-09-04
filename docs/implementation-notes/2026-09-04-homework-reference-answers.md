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
reference answer and saves that answer alongside the tracked attempts.

## Important decisions

- Reference answers appear only after the complete question is correct, preserving
  retrieval practice.
- Learner attempts remain unchanged so progress in reasoning stays visible.
- The reference answers cover the original prompt, not only the final follow-up.

## Verification

- Confirmed the rule follows the existing retry and answer-tracking steps.
- Backfilled Question 1 with a complete ownership reference answer after it was marked
  correct.
- git diff --check passed.

## Maintenance

Keep reference answers concise and verify Rust claims against official Rust documentation.
Do not reveal or save a reference answer while a question is still marked partly correct
or retry.

## Source request

Liam's request on 2026-09-04 to always receive a final reference answer after completing
each whole homework question.
