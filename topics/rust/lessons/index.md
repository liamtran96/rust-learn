---
title: Lesson Summaries
tags: [rust, lessons, index]
---

# Lesson Summaries

> A short, opinionated summary of every lesson you finish. Written for **future-you** who will have forgotten — make it dense enough to refresh the concept in two minutes.
> Pair with [[progress|the daily log]] (facts) and [[journal|the journal]] (prose reflection).

## How summaries get made

When you finish a session and tell me "done" (or run the `/journal` command), I will:
1. Append a daily entry to [[progress]] and [[journal]] (the existing flow).
2. **Create a new file in this folder** — `YYYY-MM-DD-<slug>.md` — using the template below.
3. Add a row to the **Index** table at the bottom of this file.
4. Tick the relevant box in [[study-plan]] / [[roadmap]].

You don't write these. I do. Your job is to read them on Sunday during weekly review.

## Template

The blank template lives at [[_template]]. Each summary has six fixed sections, intentionally short:

```
TL;DR             — one sentence you must remember in 6 months
What it is        — 2–4 sentences in your own words
Canonical form    — minimal Rust snippet
Why Rust does it  — the motivation (usually ownership or zero-cost)
Pitfalls          — the traps (link to pitfalls.md if listed there)
Self-check        — 3 questions you should be able to answer without docs
```

Keep it terse. If a summary needs more than ~40 lines, the lesson was actually two lessons — split it.

## Why this format

- **TL;DR first** so re-reading takes 5 seconds, not 5 minutes.
- **Canonical form** because Rust syntax has just enough idiom that getting the canonical shape into your fingers matters more than knowing all the variants.
- **Self-check** turns each summary into a flashcard. On Sunday review, cover the body and answer the three questions.

## Index

Newest at the top. Tag-filter in Obsidian if you want to slice by topic.

| Date | Title | Chapter | Tags |
|---|---|---|---|
| 2026-04-19 | [[2026-04-19-control-flow-and-match\|Control flow + match]] | Ch 1 | control-flow, match, ranges, macros |

<!-- New rows go above this comment, newest at the top. -->
