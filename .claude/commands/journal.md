---
description: Append a dated entry to the Rust learning journal + log any mistakes to the chapter's mistakes.md
argument-hint: [optional: topic / exercise name]
allowed-tools: Read, Edit, Write, Bash(date *)
---

Append a new entry to the matching topic file under `topics/rust/journal/` capturing what Liam just worked on in this conversation, AND append any mistakes / misunderstandings from the session to the relevant chapter's `mistakes.md`.

**Optional topic hint from the user:** $ARGUMENTS

## Steps

1. Run `date +%Y-%m-%d` to get today's absolute date.
2. Read `topics/rust/journal.md` for the template and topic list. Determine the chapter from the current phase or exercise path, then select the closest subject file under `topics/rust/journal/<NN-chapter>/`. Create and index a concise topic file when no existing subject fits.
3. Append a new entry **below any existing entries** in the selected topic journal (under the `## Entries` heading), using exactly this template:

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

6. **Log any mistakes from this session** to the relevant chapter's `mistakes.md` file (`topics/rust/<NN-chapter>/mistakes.md`):

   - Determine the chapter from the work done (e.g. Ch 1 fundamentals → `topics/rust/01-fundamentals/mistakes.md`).
   - If the file doesn't exist yet, create it with the same skeleton as `topics/rust/01-fundamentals/mistakes.md` (frontmatter, intro, **Open mistakes** section, **Resolved** section, **How to use this file** footer).
   - For each mistake / misunderstanding Liam made during the session, append a new entry under **Open mistakes** in this exact format:

     ```
     ### YYYY-MM-DD — <short title> (<exercise # or context if relevant>)
     - **What I wrote:** <quote what Liam actually wrote, don't paraphrase>
     - **Why it's wrong:** <the misunderstanding underneath, not just the surface error>
     - **The rule:** <the durable principle that prevents this in future>
     - **Status:** 🔴 fresh
     ```

   - Always start a new entry as 🔴. Liam upgrades the status himself during reviews (🔴 → 🟡 → 🟢).
   - A "mistake" worth logging is anything that revealed a conceptual gap: wrong predictions, broken syntax he wrote, idiomatic gaps (e.g. using `match` where `if` fits), keyword confusions, etc. Don't log normal compiler-error feedback loops he caught and fixed in 30 seconds.
   - If no mistakes happened this session, skip this step entirely.

7. Reply in chat with one line: the date + topic you logged, which milestone (if any) you ticked, and how many mistakes (if any) you appended to which `mistakes.md`. Nothing else.

## Rules
- Never overwrite prior entries - append only (in topic journals and `mistakes.md`). Keep `topics/rust/journal.md` as the index and template.
- Use the absolute date from `date`, never a relative one like "today".
- Keep each field to one or two lines. If there's nothing to say, write `—`.
