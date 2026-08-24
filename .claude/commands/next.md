---
description: Figure out what Liam should work on next and offer to scaffold it
argument-hint: (no args)
allowed-tools: Read, Bash(ls *), Bash(test *)
---

Tell Liam exactly what he should do next, based on his materials. He should never have to dig through `study-plan.md` to find his to-do list — that's this command's job.

## Steps

1. **Read `topics/rust/progress.md`** (repo-relative) — find the "Summary" table and the most recent daily-log entry. From them, derive:
   - Current phase + week (e.g. "Phase 1 · Week 1")
   - Exercises completed count for current chapter (e.g. "2 / 7")
   - Last shipped project
   - Any explicit "Tomorrow's first move" line in the most recent daily log

2. **Read `topics/rust/study-plan.md`** — find the week heading that matches the current week. Read every bullet under that week. These are the to-do list.

3. **Read `topics/rust/exercises/ch<NN>-*.md`** for the chapter that matches the current week. List every exercise.

4. **List `code/<phase>/`** to see which crates already exist. A folder existing = that exercise is at least scaffolded. If a `code/<phase>/drills-<topic>/` crate exists for the current chapter, check its drill status: a drill is unfinished when its test target isn't green or its `WHY:` line is empty.

5. **Decide the single best next action** by walking this priority list, top to bottom, stopping at the first match:
   1. The "Tomorrow's first move" line from the latest daily log, if it points to something specific.
   2. The next unfinished drill in the current chapter's `code/<phase>/drills-<topic>/` crate — name the exact command (`cargo test --test dNN`) and point at the crate's `WORKFLOW.md`.
   3. The next unchecked exercise in `exercises/ch<NN>-*.md` whose corresponding crate doesn't yet exist under `code/<phase>/`.
   4. The next unchecked `**Ship:**` bullet in `study-plan.md` for the current week.
   5. If everything in the current week is done → tell him to start the next week and point to its first bullet.

   Repo rule (root `WORKFLOW.md`): code first, read to unblock. Prefer the task that has Liam typing code within two minutes; name the concept note as the unblock resource, not the first step.

6. **Reply in chat with exactly this format:**

   ```
   You're on: <Phase> · <Week>
   Done so far: <list of shipped projects + paper exercises completed>
   Remaining this week: <count> bullet(s)

   ▶ Next: <exercise title>
     Spec: topics/rust/exercises/ch<NN>-*.md → "<exercise heading>"
     Concept note: topics/rust/<NN-name>/<file>.md  (the chapter note that teaches the idea)
     <One-line description of what to build>
     <If it needs a crate>: Run `/new-exercise <NN> <suggested-slug>` to scaffold it at code/<phase>/<slug>/.
     <If it's a paper exercise>: No crate needed — predict, run, log. ~10–15 min.

   How topics/ and code/ connect for this exercise:
     • Read in   topics/rust/<NN-name>/<file>.md      ← concept
     • Spec in   topics/rust/exercises/ch<NN>-*.md    ← what to build
     • Write in  code/<phase>/<slug>/                 ← the Cargo crate
     • Log in    topics/rust/progress.md + journal.md ← reflection (use /journal)

   After this: <next 2 things in line, one line each, no detail>
   ```

7. **Then ask** at the end: "Want me to scaffold it now?" — if Liam says yes, invoke the `/new-exercise` flow with the suggested args. Otherwise stop.

## How the two halves of the repo fit together

Always include the "How topics/ and code/ connect" block in the reply. The repo is split into a vault and a lab — Liam should be reminded each time which file he reads, which file he writes in, and where the result lands:

- `topics/rust/` is the **Obsidian vault** (the textbook + notebook). Chapter notes under `topics/rust/<NN-name>/` teach concepts. Exercise specs under `topics/rust/exercises/ch<NN>-*.md` brief what to build. Cheatsheets, pitfalls, journal, and progress all live here. `[[wikilinks]]` resolve only inside Obsidian.
- `code/` is the **lab** (real Cargo crates). One crate per exercise that needs one, organized by phase: `code/01-fundamentals/<slug>/`. `cargo run` / `cargo test` happen here.
- They reference each other by **path**, never by import. The flow per session is: read concept in `topics/` → read spec in `topics/exercises/` → write Rust in `code/<phase>/<slug>/` → log back in `topics/rust/progress.md` + `journal.md`.

When picking the next action, always name the concept note path (chapter note in `topics/rust/<NN-name>/`) alongside the exercise spec path, so Liam knows what to read before coding.

## Rules

- **One next action.** Not three. Not "here's everything left this week." Liam asked to be told *what to do next*, not given the whole plan again.
- **Suggest a slug** for any exercise that needs a crate. Liam doesn't have to invent one — derive it from the exercise title (lowercase, kebab-case, ≤ 3 words).
- **Don't write code.** This command tells him what to do next; `/new-exercise` sets up the runway; he writes the code.
- **Don't restate the whole study plan.** Just the one next thing + a 2-line "after this" tail.
- If `progress.md` and `study-plan.md` disagree (e.g. progress says Week 2, plan suggests Week 1 isn't done), trust `progress.md` for *where he is*, but flag the gap in one line: "Note: Week 1 still has X unchecked — flag if you meant to skip."
