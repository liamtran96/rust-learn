# Topic-Based Learning Journals

## Goal
Keep the learning journal easy to open and review by replacing one 1,200-line entry store with a short index and subject-based journal files grouped under each chapter.

## Affected files
- `topics/rust/journal.md`: stable Obsidian index and entry template.
- `topics/rust/journal/01-fundamentals/*.md`: 15 existing entries across six subjects.
- `topics/rust/journal/02-ownership/*.md`: 15 existing entries across three subjects.
- Journal readers and writers in `.agents/`, `.claude/`, `AGENTS.md`, `CLAUDE.md`, `WORKFLOW.md`, and `topics/rust/index.md`.

## Implementation flow and decisions
Existing entries were partitioned by subject without rewriting their content; the later Fundamentals retrieval session remains with Fundamentals. The original `journal.md` path remains as the index so existing `[[journal]]` links continue to resolve. Future journal runs select the closest indexed subject from the current phase and exercise, creating a new topic file only when no existing subject fits.

## Verification
- Confirmed the migration preserved all 30 dated entries across nine subject files: 15 Fundamentals and 15 Ownership.
- Confirmed every migrated heading occurs exactly once across the topic journals.
- Checked active journal writers and homework readers for obsolete single-file assumptions.

## Maintenance
Keep entry content in `topics/rust/journal/<NN-chapter>/<topic>.md` and navigation/template content in `topics/rust/journal.md`. Prefer an existing subject file; create and index a new one only for a genuinely distinct topic.
