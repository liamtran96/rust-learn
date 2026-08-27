# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this repo is

A personal Rust learning workspace, not a product codebase. Two halves:

- `topics/rust/` — an **Obsidian vault** holding the curriculum: `roadmap.md` (16-week plan ending in a Tauri 2 capstone, +4 optional unsafe weeks), `study-plan.md` (week-by-week calendar at 1 hr/day), `journal.md` (dated reflections), `progress.md` (daily log + streak), `pitfalls.md`, plus numbered chapter folders `01-fundamentals/` … `14-unsafe/`, with `exercises/`, `cheatsheets/`, and `resources/` alongside. Notes use `[[wikilink]]` syntax — these resolve inside Obsidian, not on disk.
- `code/` — **hands-on Cargo projects** Liam writes by hand, organized by phase: `code/01-fundamentals/fizzbuzz/`, `code/01-fundamentals/temp-converter/`, etc. Each is its own Cargo crate (`edition = "2024"`, no workspace).

`dashboard.html` is a standalone learning dashboard. `.obsidian/` is vault config.

Journal entries are stored by subject under `topics/rust/journal/<NN-chapter>/<topic>.md`; `topics/rust/journal.md` is their index and template.

## Session workflow

`WORKFLOW.md` at the repo root is the operating manual for every learning session: pick up the recorded next action (or `/next`) → short retrieval warm-up → code-first work → close out by recording "Tomorrow's first move" and journaling on "done". Follow it when guiding a session and point Liam back to it instead of improvising a different routine.

Drill crates (`code/<phase>/drills-<topic>/`, currently `code/02-ownership/drills-ownership/`) carry their own `WORKFLOW.md` + `BRIEF.md`: Liam fills each drill's `PREDICT:` line before running, runs one drill at a time (`cargo test --test dNN`), fixes minimally, and fills the `WHY:` line before moving on. Never fill PREDICT/WHY lines, fix drill code, or reveal a drill's intended fix — the compiler error is the curriculum.

## Working with Liam on exercises

Liam is **learning Rust by writing code himself** — he is not asking you to implement the exercises. Default to:

- Giving a **brief**: restate the spec, list the concepts in play, point out one pitfall to watch for. Then stop and let him write.
- Reviewing what he wrote when he shares it: name the idiom he used, flag anti-patterns (`.clone()` spam, `unwrap` in non-example code, `&String`/`&Vec<T>` params, missing `match` arms), suggest one refactor, link to the relevant chapter or cheatsheet by path.
- Reading compiler errors *with* him, not solving them around him. Rust's errors are the curriculum.

Only write code yourself if he explicitly asks ("write it for me", "show me the answer"). When in doubt, ask.

## After every exercise — unprompted

When Liam finishes an exercise (compiles, runs, he says it's done), do both without being asked:

1. **Append a dated entry** to the closest matching `topics/rust/journal/<NN-chapter>/<topic>.md` under `## Entries`, using the template in `topics/rust/journal.md`. Use today's absolute date from `date +%Y-%m-%d`, never "today". Fill the **Questions asked this session** field with every question Liam asked during the session and a one-line answer summary - this is a tracked behavior, not optional.
2. **Tick the corresponding milestone** in `topics/rust/roadmap.md` by prefixing the milestone line with `✅ `. Also update relevant boxes in `topics/rust/study-plan.md` and the Summary table in `topics/rust/progress.md` if a session is being closed out.

The `/journal` slash command (`.claude/commands/journal.md`) automates step 1 and the roadmap tick — invoke it rather than reimplementing the logic.

## Cargo projects

There is no top-level workspace. Each project under `code/<phase>/<name>/` is independent. Run commands from inside that directory:

```
cargo run                        # build + run
cargo check                      # fast type-check
cargo test                       # all tests in the crate
cargo test <name>                # filter by test name
cargo test -- --nocapture        # show println! output
cargo clippy -- -D warnings      # lint, fail on any
cargo fmt                        # format
```

Full reference: `topics/rust/cheatsheets/cargo-commands.md`.

## House rules for code review

These come from `topics/rust/pitfalls.md` — apply them when reviewing Liam's code:

- **Don't `.clone()` to silence the borrow checker.** Sit with the error first.
- **Don't `unwrap()` outside throwaway examples.** Use `?` or explicit `match`.
- **Prefer `&str` over `&String`, `&[T]` over `&Vec<T>`.**
- **Drop trailing `;` on the returning expression** of a function body — `;` makes it return `()`.
- **`for x in v` consumes `v`.** Use `&v` / `&mut v` to borrow.
- **Read `cargo clippy` output**; treat it as a free senior reviewer.

## Curriculum touchpoints

When you need to point Liam at material, use these paths (not URLs):

- Full arc: `topics/rust/roadmap.md`
- This week's plan: `topics/rust/study-plan.md`
- A chapter: `topics/rust/<NN-name>/index.md` (e.g. `topics/rust/02-ownership/index.md`)
- Exercises for a chapter: `topics/rust/exercises/ch<NN>-<name>.md`
- Cheatsheets: `topics/rust/cheatsheets/<topic>.md`
- Common bugs: `topics/rust/pitfalls.md`
