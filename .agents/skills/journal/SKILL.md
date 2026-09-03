---
name: journal
description: Record a completed Rust learning session, update learning records, log meaningful mistakes, then commit and push the focused session change to main. Use when Liam invokes $journal or finishes an exercise and asks to record the session.
---

# Record the Rust learning session

Treat the repository root as the base for every path. Interpret text following `$journal` as an optional topic or exercise hint.

1. Determine today's absolute date as `YYYY-MM-DD` from the current environment.
2. Read the template and topic list in `topics/rust/journal.md`. Determine the chapter from the current phase or exercise path, then select the closest existing subject file under `topics/rust/journal/<NN-chapter>/`. If no existing subject fits cleanly, create a concise kebab-case topic file with frontmatter, a link back to `[[../../journal|Journal index]]`, and an `## Entries` heading; add its wikilink to the index.
3. When closing a completed drill under `code/<phase>/drills-<topic>/tests/`:
   - Verify its isolated test target passes and Liam's `PREDICT:` and `WHY:` lines are non-empty before treating it as complete.
   - Preserve Liam's `PREDICT:` and `WHY:` text exactly; never rewrite, correct, or replace either answer.
   - Immediately below Liam's `WHY:` line, add a separate `//! REVIEW:` comment with a concise, technically correct version of the explanation. Directly answer the drill's `WHY:` prompt and correct any remaining misconception by naming the relevant types and Rust rule.
   - Add `REVIEW:` only after Liam has attempted the explanation and the drill is green, so it cannot reveal the intended fix early. If a `REVIEW:` comment already exists, update only that comment and continue preserving Liam's answer.
4. Append a new entry under `## Entries` in the matching topic journal; never overwrite or reorder prior entries. Keep `topics/rust/journal.md` as an index and template, not an entry store.
5. Populate the entry from the current conversation. Capture the exercise path, concepts that clicked, confusion, every question Liam asked, unresolved questions, and the next roadmap action.
6. For every recorded question, include:
   - the question, verbatim or nearly so;
   - a 2–4 sentence technical answer that defines Rust jargon on first use;
   - a 3–6 line runnable-looking Rust example, or a clearer real-world analogy;
   - a relevant repository note path, or `—` when none fits.
7. Use this entry shape unless the journal's current template supersedes it:

   ```markdown
   ### YYYY-MM-DD — <Topic>
   **Working on:** <exercise name and path under code/>
   **What clicked:** <specific concepts>
   **What didn't:** <confusion or repeated attempts>
   **Questions asked this session:**
   - **Q:** <question>
     - **Technical answer:** <answer>
     - **Plain-English analogy / example:** <analogy or short Rust example>
     - **See also:** <repository path or —>
   **Question to answer later:** <question or —>
   **Next:** <next roadmap action>
   ```

8. If a week milestone was completed, update the corresponding milestone in `topics/rust/roadmap.md`. Also update relevant completion state in `topics/rust/study-plan.md` and the summary in `topics/rust/progress.md` when closing an exercise session.
9. Log meaningful conceptual mistakes in `topics/rust/<NN-chapter>/mistakes.md` under **Open mistakes**. Do not log routine compiler feedback Liam fixed immediately. If the file is absent, follow the structure of `topics/rust/01-fundamentals/mistakes.md`.
10. Use this mistake shape:

   ```markdown
   ### YYYY-MM-DD — <short title> (<exercise or context>)
   - **What I wrote:** <exact quote>
   - **Why it's wrong:** <underlying misunderstanding>
   - **The rule:** <durable principle>
   - **Status:** 🟥 fresh
   ```

11. If this session changes a chapter from incomplete to complete, automatically generate
    one retrieval homework set focused on that chapter by following the **Generate
    homework** and **Automatic chapter closeout** sections in
    `.agents/skills/homework/SKILL.md`:
    - Run this only for a new completion transition, not whenever an already-complete
      chapter is journaled again.
    - Generate after updating progress, journals, and mistakes, and before staging files.
    - Include the generated homework file in the same focused closeout commit.
    - Report the homework path in the final response.
12. After the session files are updated and relevant verification passes, commit and push the completed session:
   - Confirm the current branch is `main`. If it is not, do not switch branches, merge, commit, or push; report the mismatch.
   - Inspect `git status` and the diff. Stage only the exercise and learning-record files changed for this completed session; never include unrelated user changes. If session changes cannot be isolated safely, stop before committing and explain why.
   - Review the staged diff, then create one focused Conventional Commit whose message describes the completed learning work. Do not amend an existing commit.
   - Push `main` to its configured upstream with an ordinary push. Never force-push. Request any tool or network approval required at push time; if the remote, authentication, network, or push fails, keep the local commit and report the failure without retrying destructively.
13. Reply with one line containing the date and topic, the milestone updated if any, the number and destination of mistake entries, the chapter homework path when generated, the commit hash, and the push result.

Keep fields concise. Use `—` for genuinely inapplicable fields. Never use a relative date such as “today.”
