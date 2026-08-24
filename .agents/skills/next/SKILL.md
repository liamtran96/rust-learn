---
name: next
description: Determine the single best next Rust learning task from this repository's progress, study plan, exercise specifications, and existing crates. Use when Liam invokes $next or asks what to work on next.
---

# Choose the next Rust learning task

Treat the repository root as the base for every path. This skill takes no arguments.

## Source-of-truth invariant

`topics/rust/progress.md` is the authoritative learning record. Copy its phase, week,
exercise count, and shipped-project count without recalculating them from exercise
specifications, study-plan checkboxes, crate contents, or verification evidence.

- Never downgrade or upgrade an exercise or project recorded in `progress.md`.
- Treat unfinished variants, cleanup, and spec differences recorded after completion as
  follow-up work unless `progress.md` itself changes the completion status.
- Use the study plan, specifications, and crates to select and describe the next action,
  not to override official progress.
- If another file conflicts, keep the `progress.md` status and label the discrepancy as
  a tracker note. Do not present an inferred status as fact.

1. Read `topics/rust/progress.md`. From its summary and latest daily log, identify the current phase and week, completed exercises, last shipped project, and any explicit “Tomorrow's first move.”
2. Read the matching week in `topics/rust/study-plan.md` and all bullets beneath it.
3. Read the matching `topics/rust/exercises/ch<NN>-*.md` and list its exercises internally.
4. Inspect `code/<phase>/`; an existing directory means the exercise is at least scaffolded. If a `code/<phase>/drills-<topic>/` crate exists for the current chapter, check its drill status: a drill is unfinished when its test target is not green or its `WHY:` line is empty.
5. Select exactly one next action using the first applicable rule:
   1. Follow a specific “Tomorrow's first move.”
   2. Choose the next unfinished drill in the current chapter's `code/<phase>/drills-<topic>/` crate. Name the exact command (`cargo test --test dNN`) and point at the crate's `WORKFLOW.md`.
   3. Choose the next unchecked exercise whose crate does not exist.
   4. Choose the next unchecked `**Ship:**` bullet for the current week.
   5. If the week is complete, choose the first task of the next week.

Prefer a task that has Liam typing code within two minutes over one that starts with reading; name the concept note as the unblock resource, not the first step (repo rule: code first, read to unblock — see `WORKFLOW.md` at the repository root).
6. Run a status-fidelity check before replying: every phase, week, exercise-count, and shipped-project claim must match `progress.md`. Mention conflicting unchecked work in one tracker-note line without changing the official status.
7. Name the chapter concept note alongside the exercise specification. Suggest a lowercase kebab-case slug of at most three words when a crate is needed.
8. Reply in this shape:

   ```text
   You're on: <Phase> → <Week>
   Official progress: <exercise count and shipped-project count copied from progress.md>
   Done so far: <shipped projects and paper exercises recorded in progress.md>
   Study-plan boxes still unchecked: <count>
   <Optional> Tracker note: <conflict or follow-up; official progress is unchanged>

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

Give one next action, not the whole plan. Never infer a new completion count from the specification or code. Do not write Rust code or scaffold anything until Liam agrees. If he agrees, follow the `$new-exercise` workflow.
