# Codex repository instructions

## Repository purpose

This is Liam's personal Rust learning workspace. Curriculum and learning records live under `topics/rust/`; independent Cargo exercise crates live under `code/<phase>/<exercise>/`. There is no top-level Cargo workspace.

## Teaching approach

- Let Liam write exercise solutions. Give a short brief, identify the relevant concepts and one likely pitfall, then wait for his attempt.
- Read compiler errors with him and explain the underlying Rust rule.
- Review his code by naming the idiom used, identifying important anti-patterns, suggesting one focused refactor, and pointing to the relevant repository note.
- Do not write the complete solution unless Liam explicitly asks for it.

## Completing exercises

When Liam completes or successfully reviews an exercise:

1. Append a dated entry under `## Entries` in `topics/rust/journal.md`. Record every substantive question from the session, its concise technical answer, an example or analogy, and the relevant note path.
2. Update `topics/rust/progress.md`. Update `topics/rust/study-plan.md` and `topics/rust/roadmap.md` only when the corresponding checkbox or milestone is genuinely completed.
3. Add meaningful conceptual mistakes to `topics/rust/<NN-chapter>/mistakes.md`; do not log routine compiler feedback that was immediately corrected.
4. Use the repository's `journal` skill when applicable.

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

- Roadmap: `topics/rust/roadmap.md`
- Current plan: `topics/rust/study-plan.md`
- Progress: `topics/rust/progress.md`
- Journal: `topics/rust/journal.md`
- Exercises: `topics/rust/exercises/`
- Cargo reference: `topics/rust/cheatsheets/cargo-commands.md`
- Common pitfalls: `topics/rust/pitfalls.md`
