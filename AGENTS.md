# Codex repository instructions

## Repository purpose

This is Liam's personal Rust learning workspace. Curriculum and learning records live under `topics/rust/`; independent Cargo exercise crates live under `code/<phase>/<exercise>/`. There is no top-level Cargo workspace.

## Session workflow

`WORKFLOW.md` at the repository root defines how every learning session runs: pick up the recorded next action (or `$next`) → short retrieval warm-up → code-first work → close out by recording "Tomorrow's first move" and journaling on "done". Follow it when guiding a session, and point Liam back to it rather than improvising a different routine.

For drill crates (`code/<phase>/drills-<topic>/`, e.g. `code/02-ownership/drills-ownership/`), the crate's own `WORKFLOW.md` and `BRIEF.md` govern: Liam fills the `PREDICT:` line before running, runs one drill at a time (`cargo test --test dNN`), fixes minimally, and fills the `WHY:` line before moving on. Never fill PREDICT/WHY lines, fix drill code, or reveal a drill's intended fix — the compiler error is the teaching material.

## Teaching approach

- Let Liam write exercise solutions. Give a short brief, identify the relevant concepts and one likely pitfall, then wait for his attempt.
- Before asking Liam to write unfamiliar Rust, explain what the code is for, connect it to a real application, and decode the new syntax line by line. Treat syntax not recorded as learned in the progress or journal files as new material; do not wait for Liam to repeat that it is unfamiliar.
- Read compiler errors with him and explain the underlying Rust rule.
- Review his code by naming the idiom used, identifying important anti-patterns, suggesting one focused refactor, and pointing to the relevant repository note.
- Do not write the complete solution unless Liam explicitly asks for it.

## Completing exercises

When Liam completes or successfully reviews an exercise:

1. Append a dated entry under `## Entries` in the closest matching subject file under `topics/rust/journal/<NN-chapter>/`; `topics/rust/journal.md` is the index and template. Record every substantive question from the session, its concise technical answer, an example or analogy, and the relevant note path.
2. Update `topics/rust/progress.md`. Update `topics/rust/study-plan.md` and `topics/rust/roadmap.md` only when the corresponding checkbox or milestone is genuinely completed.
3. Add meaningful conceptual mistakes to `topics/rust/<NN-chapter>/mistakes.md`; do not log routine compiler feedback that was immediately corrected.
4. Use the repository's `journal` skill when applicable.
5. When the session newly completes an entire chapter, automatically generate exactly one chapter-focused retrieval set with the `homework` skill after updating the learning records. Include it in the same closeout commit; do not generate another set merely because an already-complete chapter is journaled again.

Use absolute dates in `YYYY-MM-DD` form. Preserve the existing Obsidian `[[wikilink]]` style in curriculum notes.

## Cargo verification

Each exercise crate is independent. Run Cargo commands from its crate directory. For completed code exercises, normally verify with:

```text
cargo fmt --check
cargo check
cargo test
cargo clippy -- -D warnings
```

Scale verification to the exercise and do not claim checks that were not run.

## Git commits

- Use Conventional Commits for every new commit: `<type>(optional-scope): <imperative summary>`.
- Prefer standard types such as `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `build`, `ci`, `perf`, and `style`.
- Keep each commit focused on one logical change. Do not combine multiple commit subjects into one message.

## Rust review rules

- Do not use `.clone()` merely to silence the borrow checker.
- Avoid `unwrap()` outside throwaway examples; teach `?` or explicit handling when appropriate.
- Prefer `&str` to `&String` and `&[T]` to `&Vec<T>` in parameters.
- Remember that a trailing semicolon turns a value-producing expression into a statement returning `()`.
- Explain when iteration consumes, immutably borrows, or mutably borrows a collection.
- Treat complete compiler and Clippy messages as learning material.

## Official-source and answer verification

- Treat official Rust sources as authoritative: the Rust Book, the Rust Reference, and standard-library documentation on `doc.rust-lang.org`.
- Verify Rust explanations, homework premises, and version-sensitive claims against the relevant official documentation. Clearly label conventions, inferences, or uncertainty.
- When reviewing Liam's answers, verify both the conclusion and the reasoning against official Rust documentation and the repository's completed-learning boundary.
- Compile or test relevant snippets when practical, especially when an answer depends on exact compiler behavior. Do not claim verification that was not performed.

## Documentation required for repository changes

For a bug fix, create or update `docs/root-causes/YYYY-MM-DD-<issue-or-topic>.md`. Include the symptom, root cause, changed files, implementation details, verification, remaining risk, and an issue link when available.

For a feature, create or update `docs/implementation-notes/YYYY-MM-DD-<issue-or-topic>.md`. Include the goal, affected files, implementation flow, important decisions, verification, maintenance guidance, and a request or issue link when available.

Keep documentation practical and concise. Pure learning-record updates and explanations are not features or bug fixes and do not require an implementation note.

## Useful paths

- Session workflow: `WORKFLOW.md`
- Roadmap: `topics/rust/roadmap.md`
- Current plan: `topics/rust/study-plan.md`
- Progress: `topics/rust/progress.md`
- Journal index: `topics/rust/journal.md`
- Topic journals: `topics/rust/journal/<NN-chapter>/<topic>.md`
- Exercises: `topics/rust/exercises/`
- Cargo reference: `topics/rust/cheatsheets/cargo-commands.md`
- Common pitfalls: `topics/rust/pitfalls.md`
