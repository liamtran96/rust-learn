# Repo Workflow — how a learning session runs

> The daily operating manual for this repo. What lives where: `CLAUDE.md` / `AGENTS.md` (agent rules),
> `topics/rust/roadmap.md` (the arc), `topics/rust/study-plan.md` (the calendar), this file (the routine).

## The daily loop (25–60 min)

```text
1. Pick up   ->  2. Warm up   ->  3. Code   ->  4. Close out
   (2 min)        (5 min)         (most)        (5 min)
```

### 1. Pick up — never start with a decision

- Open `topics/rust/progress.md` → read yesterday's "Tomorrow's first move". That's the task.
- No entry, or unsure what's next? Ask the agent: `$next` (Codex) / `/next` (Claude). It returns exactly one task.

### 2. Warm up — retrieval before new material

- Spend ~5 minutes recalling the previous session's concept from memory (no notes).
- Periodically (about weekly), do a generated review instead: `$homework` / `/homework` →
  saved under `topics/rust/homework/`, graded on attempt.

### 3. Code — the bulk of the session

Code first; read notes only to unblock. Depending on the task:

- **Drills** (compiler-graded micro-exercises):
  work in `code/<phase>/drills-<topic>/` following that crate's `WORKFLOW.md` —
  predict → run one drill (`cargo test --test dNN`) → read the error → minimal fix → write WHY → next.
  Currently: `code/02-ownership/drills-ownership/`.
- **Exercises** (bigger, spec-driven):
  scaffold with `$new-exercise <chapter> <slug>` → work in `code/<phase>/<slug>/` from its `BRIEF.md`.
  Loop: `cargo check` (or `bacon <slug>`) while writing → `cargo run` / `cargo test` →
  `cargo clippy -- -D warnings` → `cargo fmt`.
- **Reading a chapter** (only when a chapter is genuinely new):
  read `topics/rust/<NN-name>/index.md`, but cap it — 10 minutes of reading, then write code that uses it.

House rules while coding (from `topics/rust/pitfalls.md`):
no `.clone()` to silence the borrow checker · no `unwrap()` outside throwaway code ·
`&str` over `&String`, `&[T]` over `&Vec<T>` · watch the trailing `;` on return expressions ·
`for x in v` consumes `v` · treat clippy as a free senior reviewer.

Stuck >10 minutes? Ask the agent about the **error message** — it reads errors with you, not around you.

### 4. Close out — even when unfinished

- Note the exact next action ("Tomorrow's first move") in `topics/rust/progress.md` — one line, template at the top of that file.
- **Finished something** (compiles, runs, you'd call it done)? Say **"done"** — the agent runs `$journal` / `/journal`, which:
  appends the dated entry to the closest subject file under `topics/rust/journal/<NN-chapter>/` (including every question you asked),
  ticks the milestone in `roadmap.md`, and updates `study-plan.md` / `progress.md`.
- **Finished an entire chapter?** The same closeout automatically creates one chapter-focused retrieval homework set under `topics/rust/homework/` after the learning records are current.
- Stopping mid-task is fine; a recorded next action beats a rushed finish.

## Weekly (Sundays, ~10 min)

- Fill the "Weekly review" block in `topics/rust/progress.md` and update its Summary table.
- Glance at `topics/rust/study-plan.md` — are you on the week you think you're on? Adjust honestly, don't backfill guilt.

## Commands cheat-sheet

```text
cargo run / check / test / clippy -- -D warnings / fmt    # from inside a crate dir
cargo test --test dNN                                     # one drill, drills crates only
bacon <slug> / <slug>-run / drills-ownership-test         # watch mode, from repo root
```

Full reference: `topics/rust/cheatsheets/cargo-commands.md`.

## Agent commands

| Command | Does |
|---|---|
| `$next` / `/next` | Pick exactly one next task from progress + roadmap |
| `$new-exercise <ch> <slug>` | Scaffold an exercise crate + BRIEF.md (no solution code) |
| `$homework` / `/homework` | Generate a retrieval review; graded on attempt |
| `$journal` / `/journal` | Log the finished session; tick roadmap/progress |

Agents never write exercise solutions unless explicitly asked ("write it for me") — they brief, review, and read errors with you.

## Ground rules that make this work

- **Protect the streak, not the schedule.** 25 minutes daily beats a weekend marathon; "0 min — life happened" is a valid log line.
- **Code : notes ≈ 45 : 10** within a session. If you're 30 minutes in with no code written, that's the failure mode.
- **Done = implemented + verified + explained**, not "the reference compiled" or "I read the chapter".
- **One task at a time.** Don't open a new chapter to avoid a hard exercise; `$next` exists to prevent exactly that.
