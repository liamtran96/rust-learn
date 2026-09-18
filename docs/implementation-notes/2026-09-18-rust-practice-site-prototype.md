# Rust practice site prototype

## Goal

Provide a playable website concept for replaying the real questions from Liam's Rust learning sessions, with his recorded answers, journal explanations, mistakes, and current progress. New learning records should appear automatically rather than being copied into a hard-coded dashboard.

## Affected files

- `practice-site-prototype/src/main.rs` and `Cargo.toml` provide a dependency-free Rust server that reads the repository's Markdown learning records and exposes a local JSON endpoint.
- `practice-site-prototype/index.html`, `app.js`, and `styles.css` provide three switchable layouts.
- `practice-site-prototype/README.md` contains the one-command run instructions and a place to record the winning direction.

## Implementation flow

1. The local Rust server discovers all topic journals under `topics/rust/journal/` and all supporting `topics/rust/**/mistakes.md` files.
2. It extracts context-complete session questions, session title, `Working on` context, original prompt, pre-answer code, Liam's old answer, technical answer, example or analogy, related notes, date, and source path.
3. It reads mistake records as supporting data plus the progress summary and study-plan checkbox counts.
4. The browser requests `/api/practice` every 2.5 seconds and rerenders when any source file's modification time changes.
5. Each exercise replays the original prompt and pre-answer code before asking the journal question. Corrected examples remain behind Reveal, so context does not leak the solution.
6. Liam can type an answer, reveal a side-by-side comparison with the journal's technical answer, revisit the recorded example and related notes, and rate the recall. Typed answers and ratings are intentionally in memory only because this is a design prototype.

## Important decisions

- Repository Markdown remains the source of truth; no second content database was introduced.
- Journal questions are the practice queue. Mistakes remain supporting learning records and are not converted into vague synthetic questions.
- Historical questions without explicit `Prompt context` remain visible with a `historical` label and an honest context-incomplete notice. The journal template and journal skill require `Prompt context`, `Prompt code`, and `Liam's answer` for future sessions.
- The existing `dashboard.html` was not modified because it is a stale, hard-coded snapshot and the desired practice interaction is still being chosen.
- Three structurally different layouts are available through `?variant=A`, `B`, or `C` and the fixed prototype switcher.
- Roboto Mono is the single typeface across headings, body copy, form controls, labels, and the prototype switcher.
- Questions are grouped by journal topic in chapter order. Topics are ordered by their earliest recorded session, and questions within each topic run from the earliest session to the latest.

## Verification

- Run `cargo run --manifest-path practice-site-prototype/Cargo.toml` from the repository root.
- Open `http://127.0.0.1:4173` and check all three variants.
- Confirm `/api/practice` contains the full reviewable journal queue plus `contextReady` metadata and the supporting mistake records. On 2026-09-18 this produced 151 questions: 2 context-ready and 149 historical. The legacy importer accepts the hyphen, en dash, and em dash separators used by older `answered:` entries.
- Confirm the first visible card is the 2026-04-19 `What is cargo?` question under `Control Flow And Iterators`, followed by the other questions in that topic from earliest to latest.
- Edit a topic journal and confirm the visible queue refreshes without restarting the server.

## Maintenance and next step

After Liam chooses a layout, record the verdict in the prototype README. Promote only that interaction model into a maintained site, add parser tests and durable spaced-repetition state if wanted, and delete the other layouts plus the prototype switcher.
