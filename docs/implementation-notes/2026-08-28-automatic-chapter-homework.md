# Automatic chapter-closeout homework

## Goal

Automatically create a personalized retrieval homework set whenever a learning session
newly completes a Rust chapter.

## Affected files

- `.agents/skills/journal/SKILL.md`
- `.agents/skills/homework/SKILL.md`
- `AGENTS.md`
- `WORKFLOW.md`

## Implementation flow

The journal workflow detects a genuine chapter transition from incomplete to complete. It
first updates progress, journals, and mistake logs, then calls the homework generation
workflow with that chapter as the explicit focus. The generated file is staged with the
rest of the chapter-closeout records in the same focused commit.

## Important decisions

- The automation runs as part of journal closeout, not as a background scheduler.
- It triggers only on a new completion transition, so later sessions concerning an
  already-complete chapter do not create duplicates.
- Homework is generated after learning records are current, ensuring the latest questions
  and mistakes influence the prompts.
- Existing homework safety rules still apply: do not cross the completed-learning boundary
  and do not reveal solutions.

## Verification

- `git diff --check` passed; only pre-existing line-ending warnings were reported.
- Manually verified both skill frontmatter blocks, folder names, trigger conditions,
  operation ordering, duplicate protection, and final-response requirements.
- The bundled `quick_validate.py` validator was attempted but could not run because this
  environment has no usable Python executable.

## Maintenance

Keep chapter-completion detection in the journal workflow and question construction in the
homework skill. If the definition of chapter completion changes, update the journal trigger
without duplicating homework-generation logic.

## Source request

Liam's request on 2026-08-28: automatically create homework after each chapter is finished.
