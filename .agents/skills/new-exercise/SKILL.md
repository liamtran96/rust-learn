---
name: new-exercise
description: Scaffold a new Rust exercise crate and concise BRIEF.md in this learning repository without implementing the solution. Use when Liam invokes $new-exercise with a chapter number and exercise slug or asks Codex to prepare an exercise workspace.
---

# Scaffold a new Rust exercise

Treat the repository root as the base for every path. Interpret text following `$new-exercise` as `<chapter-number> <exercise-slug>`.

1. Require both arguments. Zero-pad the chapter to two digits. Require a lowercase kebab-case slug; ask for corrected input if it contains spaces or capitals.
2. Find the directory under `topics/rust/` whose name starts with `<NN>-`; use that full directory name as `<phase>`.
3. Confirm `code/<phase>/` exists, creating only that parent when absent. If `code/<phase>/<slug>/` already exists, stop without changing it.
4. Run `cargo new code/<phase>/<slug>` with the default binary-crate settings. Do not add `--lib` or `--vcs none`.
5. Read `topics/rust/exercises/ch<NN>-*.md` and locate the section matching the slug by exercise name or number. If there is no clean match, ask Liam for the exercise number instead of guessing.
6. Read the relevant chapter notes and `topics/rust/pitfalls.md`.
7. Create `code/<phase>/<slug>/BRIEF.md` for a learner who may not recognize the Rust syntax yet. Explain only the syntax required to start the exercise; Liam still chooses the algorithm and writes every implementation line. Never include a complete or near-complete solution.

   Include one small input/output example when the specification defines behavior clearly. Decode the required function or type signature token by token (`fn`, parameter names and types, borrowing markers, return arrow, generics, or lifetimes as applicable). Then give incremental coding milestones that let Liam compile early without prescribing specific methods or control flow.

   Use this structure and fill every field from the source material:

   ````markdown
   # <Exercise title> — Brief

   > You write the implementation yourself. This brief only explains the task and unfamiliar syntax.
   > When it compiles, runs, and Liam says "done," use `$journal` to log it.

   ## What you are building
   <Restate the relevant exercise prompt in plain language.>

   <One small input/output example when behavior is unambiguous.>

   ## Required Rust syntax
   `<Required signature or declaration from the exercise spec>`

   <Explain each unfamiliar token and type in plain language without implementing the body.>

   ## Your coding steps
   1. <Smallest learner-written starting point that can compile.>
   2. <First behavior to implement.>
   3. <Next behavior or test to add.>

   ## Concepts in play
   - <concept 1>
   - <concept 2>
   - <concept 3>

   ## Watch out for
   <One relevant pitfall in one sentence.>

   ## References (read only if stuck)
   - Chapter: `topics/rust/<phase>/index.md`
   - Specific notes: `topics/rust/<phase>/<relevant-file>.md`
   - Cheatsheet: `topics/rust/cheatsheets/<topic>.md` (only when relevant)

   ## Checklist
   - [ ] I can explain each part of the required syntax
   - [ ] `cargo run` compiles and prints a stub
   - [ ] Implement the spec
   - [ ] `cargo clippy -- -D warnings` is clean
   - [ ] `cargo fmt` applied
   - [ ] Tests pass when the exercise requires them
   - [ ] Tell Codex “done” so `$journal` logs the session

   ## Run
   ```text
   cd code/<phase>/<slug>
   cargo run
   ```
   ````

8. Register the crate in the repository-root `bacon.toml`:
   - Use `<slug>` for the check-job name and `<slug>-run` for the run-job name.
   - If `<slug>` is already assigned to a different crate, use `<NN>-<slug>` and `<NN>-<slug>-run` instead.
   - Add neither job when both already point to this crate. Never duplicate or overwrite an unrelated job.
   - Give both jobs `workdir = "code/<phase>/<slug>"`, `watch = ["code/<phase>/<slug>/src", "code/<phase>/<slug>/Cargo.toml"]`, and `default_watch = false`.
   - The check job uses `command = ["cargo", "check"]`.
   - The run job uses `command = ["cargo", "run"]` and `need_stdout = true`.
   - Create `bacon.toml` when absent. If Bacon is installed, validate the configuration with `bacon --project . --list-jobs`; do not run a build.
9. Do not change the generated `src/main.rs`, write solution code, run `cargo build`, or run `cargo run`. If the source specification is missing, stop and ask Liam to add it.
10. Keep briefs concise, but use enough space to explain genuinely unfamiliar syntax. Prefer plain language and small behavioral examples over jargon. Do not introduce unlearned Rust features merely to make the exercise shorter or more idiomatic.
11. Reply using exactly:

   ```text
   Scaffolded: code/<phase>/<slug>
   Spec: topics/rust/exercises/ch<NN>-*.md → <matched section heading>
   Watch: bacon <alias> (run: bacon <alias>-run)
   Open BRIEF.md, then start writing in src/main.rs.
   ```
