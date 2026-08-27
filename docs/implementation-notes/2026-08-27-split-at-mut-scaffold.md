# Split-at-mut exercise scaffold

## Goal

Prepare the Chapter 2 `split-at-mut` exercise so Liam can implement it by hand
with syntax-first guidance and repository-level Bacon watch commands.

## Affected files

- `code/02-ownership/split-at-mut/`: Cargo-generated binary crate.
- `code/02-ownership/split-at-mut/BRIEF.md`: exercise behavior, signature
  breakdown, incremental prompts, and relevant notes.
- `bacon.toml`: check and run jobs for the crate.
- `.agents/skills/new-exercise/SKILL.md`: future brief requirements.

## Implementation and decisions

Cargo created the crate with its normal binary template. The generated
`src/main.rs` remains unchanged. The brief explains unfamiliar syntax token by
token and directs the learner to investigate the borrow-checker diagnostic
before consulting the standard slice API; it does not provide the solution.
The brief now has an explicit expected input/output section that distinguishes
function arguments and return values from terminal input and printed output.
The scaffolding skill requires this section in every future exercise brief.

## Verification

- Confirm the generated `src/main.rs` still contains Cargo's default program.
- Run `bacon --project . --list-jobs` and confirm both new aliases are listed.
- Run `git diff --check` for whitespace errors.
- Validate the updated `new-exercise` skill structure.
- Do not build or run the exercise during scaffolding.

## Maintenance

Keep future changes aligned with `topics/rust/exercises/ch02-ownership.md` and
preserve the learner-authored implementation in `src/main.rs`.
