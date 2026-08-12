---
name: next
description: Determine the single best next Rust learning task from this repository's progress, study plan, exercise specifications, and existing crates. Use when Liam invokes $next or asks what to work on next.
---

# Choose the next Rust learning task

Treat the repository root as the base for every path. This skill takes no arguments.

1. Read `topics/rust/progress.md`. From its summary and latest daily log, identify the current phase and week, completed exercises, last shipped project, and any explicit “Tomorrow's first move.”
2. Read the matching week in `topics/rust/study-plan.md` and all bullets beneath it.
3. Read the matching `topics/rust/exercises/ch<NN>-*.md` and list its exercises internally.
4. Inspect `code/<phase>/`; an existing directory means the exercise is at least scaffolded.
5. Select exactly one next action using the first applicable rule:
   1. Follow a specific “Tomorrow's first move.”
   2. Choose the next unchecked exercise whose crate does not exist.
   3. Choose the next unchecked `**Ship:**` bullet for the current week.
   4. If the week is complete, choose the first task of the next week.
6. Trust `progress.md` when it conflicts with the study plan, but mention unchecked work from an earlier week in one line.
7. Name the chapter concept note alongside the exercise specification. Suggest a lowercase kebab-case slug of at most three words when a crate is needed.
8. Reply in this shape:

   ```text
   You're on: <Phase> → <Week>
   Done so far: <shipped projects and paper exercises>
   Remaining this week: <count> bullet(s)

   → Next: <exercise title>
     Spec: topics/rust/exercises/ch<NN>-*.md → "<exercise heading>"
     Concept note: topics/rust/<NN-name>/<file>.md
     <One-line build description>
     <For a crate>: Use `$new-exercise <NN> <slug>` to scaffold it at code/<phase>/<slug>/.
     <For paper work>: No crate needed — predict, run, log. About 10–15 min.

   How topics/ and code/ connect for this exercise:
     Read in   topics/rust/<NN-name>/<file>.md       → concept
     Spec in   topics/rust/exercises/ch<NN>-*.md    → what to build
     Write in  code/<phase>/<slug>/                  → Cargo crate
     Log in    topics/rust/progress.md + journal.md  → reflection with `$journal`

   After this: <next item>
   <following item>

   Want me to scaffold it now?
   ```

Give one next action, not the whole plan. Do not write Rust code or scaffold anything until Liam agrees. If he agrees, follow the `$new-exercise` workflow.
