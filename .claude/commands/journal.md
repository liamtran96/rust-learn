---
description: Append a dated entry to the Rust learning journal
argument-hint: [optional: topic / exercise name]
allowed-tools: Read, Edit, Bash(date *)
---

Append a new entry to `~/Workspaces/rust-learn/topics/rust/journal.md` capturing what Liam just worked on in this conversation.

**Optional topic hint from the user:** $ARGUMENTS

## Steps

1. Run `date +%Y-%m-%d` to get today's absolute date.
2. Read `~/Workspaces/rust-learn/topics/rust/journal.md` to see existing entries and the template at the top.
3. Append a new entry **below any existing entries** (under the `## Entries` heading), using exactly this template:

   ```
   ### YYYY-MM-DD — <Topic>
   **Working on:** <exercise name + path under code/>
   **What clicked:** <concrete concepts that landed — name them specifically>
   **What didn't:** <anything that confused Liam or took multiple tries>
   **Questions asked this session:**
   - <question 1> — <one-line answer summary or "still open">
   - <question 2> — <...>
   **Question to answer later:** <unresolved questions, or "—" if none>
   **Next:** <what's up next per the roadmap>
   ```

4. Fill the fields from **this conversation's context** — what exercise was just done, what compiler errors came up, what idioms were discussed. Be specific: "learned `..=` inclusive range", not "learned ranges". If a field genuinely doesn't apply, write `—`.

5. If a Week milestone in `~/Workspaces/rust-learn/topics/rust/roadmap.md` was completed, tick it by prefixing the milestone line with `✅ `.

6. Reply in chat with one line: the date + topic you logged, and which milestone (if any) you ticked. Nothing else.

## Rules
- Never overwrite prior entries — append only.
- Use the absolute date from `date`, never a relative one like "today".
- Keep each field to one or two lines. If there's nothing to say, write `—`.
