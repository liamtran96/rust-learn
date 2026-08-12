---
name: journal
description: Record a completed Rust learning session in this repository's journal, update completed milestones, and log meaningful mistakes. Use when Liam invokes $journal or finishes an exercise and asks to record the session.
---

# Record the Rust learning session

Treat the repository root as the base for every path. Interpret text following `$journal` as an optional topic or exercise hint.

1. Determine today's absolute date as `YYYY-MM-DD` from the current environment.
2. Read `topics/rust/journal.md`, including its template and existing entries.
3. Append a new entry under `## Entries`; never overwrite or reorder prior entries.
4. Populate the entry from the current conversation. Capture the exercise path, concepts that clicked, confusion, every question Liam asked, unresolved questions, and the next roadmap action.
5. For every recorded question, include:
   - the question, verbatim or nearly so;
   - a 2–4 sentence technical answer that defines Rust jargon on first use;
   - a 3–6 line runnable-looking Rust example, or a clearer real-world analogy;
   - a relevant repository note path, or `—` when none fits.
6. Use this entry shape unless the journal's current template supersedes it:

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

7. If a week milestone was completed, update the corresponding milestone in `topics/rust/roadmap.md`. Also update relevant completion state in `topics/rust/study-plan.md` and the summary in `topics/rust/progress.md` when closing an exercise session.
8. Log meaningful conceptual mistakes in `topics/rust/<NN-chapter>/mistakes.md` under **Open mistakes**. Do not log routine compiler feedback Liam fixed immediately. If the file is absent, follow the structure of `topics/rust/01-fundamentals/mistakes.md`.
9. Use this mistake shape:

   ```markdown
   ### YYYY-MM-DD — <short title> (<exercise or context>)
   - **What I wrote:** <exact quote>
   - **Why it's wrong:** <underlying misunderstanding>
   - **The rule:** <durable principle>
   - **Status:** 🟥 fresh
   ```

10. Reply with one line containing the date and topic, the milestone updated if any, and the number and destination of mistake entries.

Keep fields concise. Use `—` for genuinely inapplicable fields. Never use a relative date such as “today.”
