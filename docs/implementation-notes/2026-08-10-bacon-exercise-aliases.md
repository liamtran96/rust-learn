# Bacon exercise aliases

## Goal

Provide short, npm-script-like watcher commands for every standalone Rust exercise and make the Codex and Claude new-exercise workflows register future crates automatically.

Source request: user request in the Codex session on 2026-08-10.

## Affected files

- `bacon.toml`
- `.agents/skills/new-exercise/SKILL.md`
- `.claude/commands/new-exercise.md`
- `docs/implementation-notes/2026-08-10-bacon-exercise-aliases.md`

## Implementation flow

Each crate has two root-level Bacon jobs. `<slug>` continuously runs `cargo check`; `<slug>-run` continuously runs `cargo run` and displays stdout. Both jobs execute inside their crate and watch its `src/` directory and `Cargo.toml`.

For example:

```text
bacon count-digits
bacon count-digits-run
```

Both new-exercise workflows now add the same pair after scaffolding. Registration is idempotent. A slug collision uses `<NN>-<slug>` so an existing alias is never overwritten.

## Important decisions

- Alias names normally match exercise slugs, making future registration deterministic.
- Check and run are separate jobs so the common compiler-feedback loop stays quiet and fast.
- Watching only source and manifest paths avoids self-triggered rebuild loops from `target/` and other generated files.
- The Codex skill and matching Claude command remain aligned.
- Manual `cargo new` calls do not update `bacon.toml`; use a new-exercise workflow or add the two jobs manually.

## Verification

Completed on 2026-08-10:

1. `bacon --project . --list-jobs` parsed the configuration and listed all ten custom jobs.
2. `bacon --project . --headless count-digits` completed `cargo check` successfully and stayed idle afterward.
3. Confirmed every discovered `code/**/Cargo.toml` has both a check and run alias.
4. Checked the Codex skill's frontmatter, folder name, output contract, and UI metadata manually. The bundled Python validator was unavailable because no Python launcher is installed.

## Maintenance

When scaffolding through a new-exercise workflow, keep the generated check/run job pair in `bacon.toml`. Preserve existing aliases and use the chapter-prefixed collision fallback.

## Remaining risk

Bacon must be installed and Cargo's binary directory must be on `PATH`. Interactive programs may not be comfortable under the `-run` watcher because each save restarts them.
