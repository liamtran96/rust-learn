---
description: Scaffold a new Rust exercise crate under code/ with a brief copied from the exercise spec
argument-hint: <chapter-num> <exercise-slug>   (e.g. "01 guessing-game" or "02 string-search")
allowed-tools: Read, Write, Edit, Bash(cargo new *), Bash(ls *), Bash(test *), Bash(date *)
---

Scaffold a new Cargo crate under `code/` for one of Liam's exercises, and drop a `BRIEF.md` inside it that tells him *exactly* what to do — the spec, the concepts, one pitfall, and a minimal checklist. He writes the code himself; this command only sets up the runway.

**Arguments:** `$ARGUMENTS` — first token is the chapter number (1–14), second is the project slug (kebab-case). If either is missing, ask before doing anything.

## Steps

1. **Parse arguments.**
   - `<chapter>` → zero-pad to two digits (e.g. `1` → `01`).
   - `<slug>` → lowercase kebab-case. Reject if it contains spaces or capitals; ask Liam to rename.
   - Resolve the phase folder name by listing `~/Workspaces/rust-learn/topics/rust/` and finding the directory that starts with `<NN>-` (e.g. `01-fundamentals`, `02-ownership`).

2. **Pre-flight checks.**
   - Confirm `code/<phase>/` exists; create the parent dir only if missing.
   - If `code/<phase>/<slug>/` already exists, **stop** and tell Liam — don't clobber his work.

3. **Create the crate.**
   ```
   cargo new code/<phase>/<slug>
   ```
   (No `--lib`, no `--vcs none`. The default binary crate is what Week-1-style exercises want.)

4. **Read the matching exercise spec** from `~/Workspaces/rust-learn/topics/rust/exercises/ch<NN>-*.md`. Find the section that matches `<slug>` (by name or number). If you can't find a clean match, ask Liam which exercise number this is for rather than guessing.

5. **Write `code/<phase>/<slug>/BRIEF.md`** using this template. Fill every field from the spec + chapter notes — do not leave placeholders.

   ```markdown
   # <Exercise title> — Brief

   > Liam writes this himself. Claude only sets up the runway.
   > When it compiles, runs, and you say "done", run `/journal` to log it.

   ## Spec
   <Copy the relevant prompt verbatim from topics/rust/exercises/ch<NN>-*.md>

   ## Concepts in play
   - <concept 1 — e.g. `match` on tuples>
   - <concept 2 — e.g. `String` vs `&str`>
   - <concept 3>

   ## Watch out for
   <One pitfall from topics/rust/pitfalls.md that's likely to bite on this exercise. One sentence.>

   ## References (read first if stuck)
   - Chapter: `topics/rust/<phase>/index.md`
   - Specific notes: `topics/rust/<phase>/<relevant-file>.md`
   - Cheatsheet: `topics/rust/cheatsheets/<topic>.md` (only if there's a relevant one)

   ## Checklist
   - [ ] Read the chapter notes above (or skim if already read)
   - [ ] `cargo run` compiles and prints something — even a stub
   - [ ] Implement the spec
   - [ ] `cargo clippy -- -D warnings` is clean
   - [ ] `cargo fmt` applied
   - [ ] (If the exercise asks for tests) `cargo test` passes
   - [ ] Tell Claude "done" → `/journal` logs it and ticks the roadmap

   ## Run
   ```
   cd code/<phase>/<slug>
   cargo run
   ```
   ```

6. **Register Bacon aliases** in the repository-root `bacon.toml`.
   - Use `<slug>` and `<slug>-run`; if `<slug>` belongs to another crate, use `<NN>-<slug>` and `<NN>-<slug>-run`.
   - Do not duplicate existing jobs or overwrite jobs for another crate.
   - Both jobs use `workdir = "code/<phase>/<slug>"`, `watch = ["code/<phase>/<slug>/src", "code/<phase>/<slug>/Cargo.toml"]`, and `default_watch = false`.
   - The first runs `["cargo", "check"]`. The `-run` job runs `["cargo", "run"]` with `need_stdout = true`.
   - Create `bacon.toml` if needed. When Bacon is installed, validate with `bacon --project . --list-jobs`; do not build or run the crate.

7. **Reply in chat with exactly this format** (no extra prose):
   ```
   Scaffolded: code/<phase>/<slug>
   Spec: topics/rust/exercises/ch<NN>-*.md → <section heading you matched>
   Watch: bacon <alias> (run: bacon <alias>-run)
   Open BRIEF.md, then start writing in src/main.rs.
   ```

## Rules

- **Never write `src/main.rs` content** beyond what `cargo new` produces. Liam is learning by writing the code himself — that is the whole point of this repo (see `CLAUDE.md`).
- **Never write a solution into BRIEF.md.** No code snippets that solve the spec. Concepts and pitfalls only.
- **Never run `cargo build` / `cargo run` after scaffolding.** Liam runs it.
- If the exercise spec is genuinely missing from `topics/rust/exercises/`, stop and ask Liam to add it first — don't invent a spec.
- The BRIEF is a *brief*, not a tutorial. Keep each section to ~3 lines. If you can't keep it short, the exercise is probably two exercises.
