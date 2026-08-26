# Journal closeout automation

## Goal

Preserve Liam's original drill explanations while placing a clearer technical explanation directly beneath them, then finish a journaled session with a focused commit and ordinary push to `main`.

## Affected files

- `.agents/skills/journal/SKILL.md`
- `docs/implementation-notes/2026-08-26-journal-drill-review.md`

## Implementation flow

For a drill session, the skill first verifies that the isolated test passes and that Liam has filled both learning prompts. It leaves `PREDICT:` and `WHY:` unchanged, then adds a distinct `//! REVIEW:` comment immediately after `WHY:` before updating the journal and trackers.

After verification, the skill confirms it is on `main`, stages only files belonging to the completed session, reviews the staged diff, creates one Conventional Commit, and pushes `main` to its configured upstream.

## Important decisions

- The review is added only after Liam has attempted the explanation and made the drill green, so it does not disclose the fix prematurely.
- `REVIEW:` is separate from Liam's answer, preserving an honest learning record.
- Re-running the skill updates an existing review comment rather than adding duplicates.
- Unrelated working-tree changes are never staged; an overlap that cannot be isolated stops the commit.
- The workflow never switches branches, amends commits, or force-pushes. A failed push leaves the local commit intact and is reported.

## Verification

- Run the skill validator against `.agents/skills/journal/`.
- Inspect the instructions for the green-test gate, answer-preservation rule, placement of `REVIEW:`, and duplicate handling.
- Inspect the Git closeout instructions for the `main` branch gate, focused staging, Conventional Commit requirement, and non-force push failure behavior.

## Maintenance

Keep review comments concise and specific to the drill's `WHY:` question. If drill comment conventions change, update the marker and placement rules here and in the skill together. If the repository's target branch or remote workflow changes, update the Git closeout instructions before relying on the automation.
