---
title: Learning Journal
tags: [rust, journal]
---

# Learning Journal

> Journal entries are grouped by chapter so each note stays easy to open and review.
> Pair this with [[progress]] for session facts and each chapter's `mistakes.md` for retrieval targets.

## Topic journals

- **Fundamentals**
  - [[journal/01-fundamentals/control-flow-and-iterators|Control flow and iterators]]
  - [[journal/01-fundamentals/cli-input-and-results|CLI input and results]]
  - [[journal/01-fundamentals/variables-and-types|Variables and types]]
  - [[journal/01-fundamentals/functions-and-expressions|Functions and expressions]]
  - [[journal/01-fundamentals/strings-and-memory|Strings and memory]]
  - [[journal/01-fundamentals/retrieval-review|Retrieval review]]
- **Ownership**
  - [[journal/02-ownership/moves-and-borrowing|Moves and borrowing]]
  - [[journal/02-ownership/slices-and-lifetimes|Slices and lifetimes]]
  - [[journal/02-ownership/expressions-review|Expressions review]]

Create a topic journal when a genuinely new subject receives its first session entry. The `$journal` skill appends to the closest existing topic or creates and indexes a concise new one.

## Entry template

```markdown
### YYYY-MM-DD - Topic
**Working on:** <exercise name and path under code/>
**What clicked:** <specific concepts>
**What didn't:** <confusion or repeated attempts>
**Questions asked this session:**
- **Q:** <question>
  - **Technical answer:** <answer>
  - **Plain-English analogy / example:** <analogy or short Rust example>
  - **See also:** <repository path or ->
**Question to answer later:** <question or ->
**Next:** <next roadmap action>
```
