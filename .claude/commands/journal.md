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
   - **Q:** <the question Liam asked, verbatim or close to it>
     - **Technical answer:** <2–4 sentences explaining the concept properly — name the types, traits, or rules involved. Assume Liam is a Rust newbie: define jargon the first time it appears, and connect it back to something he already knows if possible.>
     - **Plain-English analogy / example:** <a tiny concrete code snippet (3–6 lines, fenced with ```rust) OR a real-world analogy that makes the idea click. Prefer code if the question is about syntax/semantics; prefer analogy if it's about a mental model.>
     - **See also:** <path to the chapter/cheatsheet/pitfall in the vault, e.g. `topics/rust/02-ownership/index.md`, or "—" if none fits>
   - **Q:** <next question, same structure>
     - **Technical answer:** ...
     - **Plain-English analogy / example:** ...
     - **See also:** ...
   **Question to answer later:** <unresolved questions, or "—" if none>
   **Next:** <what's up next per the roadmap>
   ```

4. Fill the fields from **this conversation's context** — what exercise was just done, what compiler errors came up, what idioms were discussed. Be specific: "learned `..=` inclusive range", not "learned ranges". If a field genuinely doesn't apply, write `—`.

   **For the Questions block specifically — Liam is a newbie, so do not be terse here:**
   - Capture **every** question he asked in the session, even small ones ("what does `&mut` mean?", "why doesn't this compile?"). Don't filter for "important" ones — the pattern of what he asks is the signal.
   - For each question, write all three sub-fields (**Technical answer**, **Plain-English analogy / example**, **See also**). Never collapse them into a one-liner.
   - The **Technical answer** must define any Rust jargon it uses (ownership, borrow, lifetime, trait, `Copy` vs `Clone`, etc.) on first mention. Don't assume he remembers terminology from earlier sessions.
   - The **example** should be runnable-looking Rust if at all possible — short enough to read in 5 seconds, concrete enough that he could paste it into a playground. If a real-world analogy genuinely lands better (e.g. "a `Box<T>` is like a coat-check ticket"), use that instead, but lean toward code.
   - Cross-link with **See also** whenever the topic maps to a chapter, cheatsheet, or pitfall already in the vault — this turns the journal into an index of his own questions over time.

5. If a Week milestone in `~/Workspaces/rust-learn/topics/rust/roadmap.md` was completed, tick it by prefixing the milestone line with `✅ `.

6. Reply in chat with one line: the date + topic you logged, and which milestone (if any) you ticked. Nothing else.

## Rules
- Never overwrite prior entries — append only.
- Use the absolute date from `date`, never a relative one like "today".
- Keep each field to one or two lines. If there's nothing to say, write `—`.
