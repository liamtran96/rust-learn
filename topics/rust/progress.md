---
title: Daily Progress Tracker
tags: [rust, progress, log]
---

# Daily Progress Tracker

> One line per day. Don't skip days. Even "0 min — life happened" counts — it keeps the streak honest.
> Pair this with [[journal]] (deeper reflection) and [[study-plan]] (the calendar).

## How to use

1. At the start of each session, copy the **template** below into the next slot under "Daily log".
2. Fill it in as you go (1–2 min total per day).
3. On Sunday, fill in the **Weekly review** for the week you just finished.
4. Update the **Summary** stats table at the top.

## Summary

| Metric | Value |
|---|---|
| Streak (current) | 3 days |
| Streak (best) | 3 days |
| Total minutes | ~75 formally tracked + sessions with duration not recorded |
| Total sessions | 16 (formally tracked) |
| Exercises completed | 9 total (7 / 7 Ch 1 + `strip_margin` and `split_at_mut` Ch 2) |
| Projects shipped | 3 (fizzbuzz, temp-converter, guessing-game) |
| Current phase | Phase 2 — Ownership |
| Current week | Week 3 |
| Last session | 2026-08-27 - `split_at_mut` |
| Days since last session | 0 |

## Template

```
### YYYY-MM-DD — Topic
- Duration: 0 min
- Phase / chapter: Phase X · ChY · section
- What I did: ...
- Exercises: #N done · #M attempted
- Code: <project name> — created / updated / shipped
- Mood: 1–5 (1 = bounced off, 5 = flow)
- Tomorrow's first move: ...
```

Keep it terse. The journal is for prose; this is for facts.

## Daily log

### 2026-04-19 — FizzBuzz
- Duration: ~60 min (estimated)
- Phase / chapter: Phase 1 · Ch 1 · control flow + macros
- What I did: First hands-on Rust program. Wrote FizzBuzz with `if`/`else`, then refactored to `match (i % 3, i % 5)`.
- Exercises: Ch 1 #1 done (two of three required ways — `if` and `match`; iterator `map` version still owed)
- Code: `code/01-fundamentals/fizzbuzz` — shipped
- Mood: 5 — clean compile first try, only `cargo fmt` nits
- Tomorrow's first move: Temperature converter (Ch 1 #2)

### 2026-04-2X — Temperature converter
> Backfill this entry — code exists in `code/01-fundamentals/temp-converter` but no log was kept.
- Duration: — (estimate when you fill this in)
- Phase / chapter: Phase 1 · Ch 1 · stdin, String, parse, match
- What I did: CLI that prompts for unit (C/F) and a temperature, parses, converts both directions with separate functions `c_to_f` / `f_to_c`.
- Exercises: Ch 1 #2 done (working, but the spec asks for a single CLI arg like `25C` — current version uses two stdin prompts; refactor later)
- Code: `code/01-fundamentals/temp-converter` — shipped (commented-out original draft still in `main.rs` — clean it up)
- Mood: —
- Tomorrow's first move: Drill exercises 4–7

### 2026-04-28 — Workflow setup + code review
- Duration: ~60 min (workflow scaffolding) — the recurring 20:00 calendar block formally starts tomorrow
- Phase / chapter: Phase 1 · Ch 1 · meta (no new Rust concept learned today)
- What I did:
  - Set up the daily tracking workflow: `dashboard.html` (visual), `progress.md` (this file), `lessons/` folder + template + index, recurring Google Calendar block (Mon-Sat 20:00-21:00 Asia/Ho_Chi_Minh through 2026-09-15).
  - Re-read previously written code: FizzBuzz (match version with `(i % 3, i % 5)` tuple pattern) and the temperature converter (stdin-based, both directions). Code is solid — temp converter has commented-out scaffolding to clean up and the spec-asked CLI-arg variant is still owed.
  - Backfilled the FizzBuzz lesson summary at `lessons/2026-04-19-control-flow-and-match.md`.
- Exercises: 0 new today (Ch 1 still at 2 of 7)
- Code: none new — only re-read existing crates
- Mood: 4 — feels good to have the rails laid; minor itch that no new code was written
- Tomorrow's first move: Ch 1 exercise #4 (predict shadowing output) at 20:00 sharp

<!-- Add new entries below this line, newest at the bottom -->

### 2026-05-03 — Guessing game
- Duration: ~60 min (one session)
- Phase / chapter: Phase 1 · Ch 1 · `loop` + `match` on `Result` + first external crate
- What I did: Built guessing game end-to-end. Hit and fixed the "single-read, looped parse → infinite Too small!" bug. Wired up `rand` 0.10 (`use rand::RngExt; rand::rng().random_range(1..=100)`) after walking through why `thread_rng`/`gen_range` no longer exist. Heavy Q&A on `Result`, `Ok`/`Err` as enum variants, `match` as a keyword expression, and trait-method-in-scope rules.
- Exercises: Ch 1 #3 done (3 / 7)
- Code: `code/01-fundamentals/guessing-game` — shipped, clippy clean
- Mood: 4 — long detour through the type system but a lot landed
- Tomorrow's first move: Ch 1 exercise #4 (predict shadowing output) — paper exercise, no crate

### 2026-08-10 - Shadowing and integer types review
- Duration: ~15 min
- Phase / chapter: Phase 1 / Ch 1 / variables and data types
- What I did: Re-predicted the shadowing examples correctly, explained block-scoped shadowing, distinguished shadowing from assignment, and explained why `i32 + i64` requires an explicit conversion.
- Exercises: Ch 1 #4 and #5 reviewed successfully (Ch 1 remains 7 / 7 complete)
- Code: none - paper exercises
- Mood: -
- Tomorrow's first move: Ch 1 exercise #6 - rewrite `sign` using an `if` expression without `return`

### 2026-08-12 — `if` expressions and implicit returns
- Duration: not recorded
- Phase / chapter: Phase 1 · Ch 1 · functions and control flow
- What I did: Scaffolded and completed `sign` without `return`; corrected a discarded branch, connected the conditional chain, and used tail expressions.
- Exercises: Ch 1 #6 completed and verified (Ch 1 remains 7 / 7 complete)
- Code: `code/01-fundamentals/sign-expression` — completed; fmt, check, test, and Clippy clean
- Mood: —
- Tomorrow's first move: Finish the iterator-`map` version of FizzBuzz

### 2026-08-12 — String performance and heap follow-up
- Duration: not recorded
- Phase / chapter: Phase 1 · Ch 1 · strings, memory, and measurement
- What I did: Compared `&'static str` with owned `String` using a manual release benchmark and Criterion; explored `black_box`, `Instant`, `Duration`, median, allocations, drops, and heap reuse with an interactive visual.
- Exercises: Ch 1 #6 follow-up; no new exercise completed
- Code: `code/01-fundamentals/sign-expression` — added manual benchmark, Criterion benchmark, and heap visualization
- Mood: —
- Tomorrow's first move: Finish the iterator-`map` version of FizzBuzz

### 2026-08-14 — Iterator `map` FizzBuzz follow-up
- Duration: not recorded
- Phase / chapter: Phase 1 / Ch 1 / iterators and expressions
- What I did: Completed the third FizzBuzz variant with `(1..=100).map(...)`; learned closure syntax, lazy transformation, `String` output, unit `()`, and the tail-expression semicolon rule.
- Exercises: Ch 1 FizzBuzz follow-up completed (official total remains 7 / 7)
- Code: `code/01-fundamentals/fizzbuzz` — iterator variant completed; fmt, check, test, and Clippy clean
- Mood: —
- Tomorrow's first move: Refactor the temperature converter to accept one CLI argument such as `25C`

### 2026-08-17 — Temperature converter CLI argument follow-up
- Duration: not recorded
- Phase / chapter: Phase 1 / Ch 1 / command-line arguments, parsing, and formatting
- What I did: Refactored the converter from two stdin prompts to one argument such as `25C`; inspected `Vec<String>` arguments, validated missing/empty input, split the numeric and unit parts, parsed `f64` with `Result`, dispatched with `match`, and formatted conversions to two decimal places.
- Exercises: Ch 1 temperature-converter follow-up; official total remains 7 / 7
- Code: `code/01-fundamentals/temp-converter` — CLI behavior works; temporary debug output, stale comments, formatting, and final verification remain
- Mood: —
- Tomorrow's first move: Remove debug/stale comments, run `cargo fmt`, then run fmt/check/test/Clippy before beginning Phase 2 ownership

### 2026-08-17 — Temperature converter verification and Phase 1 closeout
- Duration: not recorded
- Phase / chapter: Phase 1 / Ch 1 / verification and closeout
- What I did: Left the existing comments in place, formatted the crate, and verified it with `cargo fmt --check`, `cargo check`, `cargo test`, and `cargo clippy -- -D warnings`.
- Exercises: no new exercise; official Ch 1 total remains 7 / 7
- Code: `code/01-fundamentals/temp-converter` — verified; Phase 1 complete
- Mood: —
- Tomorrow's first move: Begin Phase 2 Week 2 by reading `topics/rust/02-ownership/ownership.md`

### 2026-08-17 — Ownership moves and shared borrowing
- Duration: not recorded
- Phase / chapter: Phase 2 / Ch 2 / ownership and borrowing
- What I did: Completed predict-and-fix snippets A and B; learned `String` moves, shared references with `&`, overlapping-borrow restrictions, `Vec` reallocation, and non-lexical lifetimes.
- Exercises: Ch 2 predict-and-fix A–B reviewed; official Ch 1 total remains 7 / 7
- Code: none — paper exercises
- Mood: —
- Tomorrow's first move: Read `topics/rust/02-ownership/slices.md`, then attempt Ch 2 predict-and-fix snippet D

### 2026-08-21 — Retrieval homework review
- Duration: not recorded
- Phase / chapter: Phase 2 / Ch 1 retrieval review
- What I did: Completed and reviewed all six questions in `topics/rust/homework/2026-08-18-retrieval-mixed.md`; practiced block values and unit, shadowing and assignment, loop values and binding scope, conditional return types, string-literal storage, and safe CLI parsing.
- Exercises: Retrieval homework 6 / 6 reviewed; official Ch 1 total remains 7 / 7
- Code: `code/01-fundamentals/homework-practice/` — saved Questions 4–6; Question 6 passed formatting, compilation, and runtime checks for missing, valid, text, and negative inputs
- Mood: —
- Tomorrow's first move: Resume Phase 2 by reading `topics/rust/02-ownership/slices.md`, then attempt Ch 2 predict-and-fix snippet D

### 2026-08-24 — String slices and borrowed return values
- Duration: not recorded
- Phase / chapter: Phase 2 / Ch 2 / slices and borrowing
- What I did: Completed predict-and-fix snippet D; confirmed that `first_word` returns a borrowed slice, improved its parameter from `&String` to `&str`, and reasoned about why the result cannot outlive its source `String`.
- Exercises: Ch 2 predict-and-fix D reviewed; official Ch 1 total remains 7 / 7
- Code: none — paper exercise
- Mood: —
- Tomorrow's first move: Ownership drills d01–d03 in `code/02-ownership/drills-ownership/` (`cargo test --test d01_move`); `strip_margin` comes after the drills

### 2026-08-25 - Ownership drills d01-d04: moves and borrow lifetimes
- Duration: not recorded
- Phase / chapter: Phase 2 / Ch 2 / ownership and borrowing
- What I did: Completed d01-d04; distinguished moves, `Copy`, shared and mutable borrowing, `Vec` reallocation risk, and how a borrow can end after a reference's last use before its surrounding scope ends.
- Exercises: ownership drills d01-d04 completed; official Ch 1 total remains 7 / 7
- Code: `code/02-ownership/drills-ownership/tests/` - individual d01-d04 test targets passed
- Mood: -
- Tomorrow's first move: Ownership drill d05 (`cargo test --test d05_for_consumes`)

### 2026-08-26 - Ownership drills d05-d09: borrowed iteration and slices
- Duration: not recorded
- Phase / chapter: Phase 2 / Ch 2 / ownership and borrowing
- What I did: Completed d05-d09; practiced borrowed iteration, deref coercion, owned returns, explicit lifetime relationships, and returning a word as a borrowed `&str` slice without allocation.
- Exercises: ownership drills d05-d09 completed; official Ch 1 total remains 7 / 7
- Code: `code/02-ownership/drills-ownership/tests/d05_for_consumes.rs` through `d09_slice_window.rs` - isolated tests passed
- Mood: -
- Tomorrow's first move: Ownership drill d10 (`cargo test --test d10_mut_through_ref`)

### 2026-08-27 - Ownership drills d10-d11: mutable slices and tail expressions
- Duration: not recorded
- Phase / chapter: Phase 2 / Ch 2 / ownership and borrowing
- What I did: Completed d10-d11; mutated elements through `&mut [i32]` and confirmed that a semicolon discards a tail expression's value, causing the enclosing block to evaluate to `()`.
- Exercises: ownership drills d10-d11 completed; official Ch 1 total remains 7 / 7
- Code: `code/02-ownership/drills-ownership/tests/d10_mut_through_ref.rs` and `d11_scoped_return.rs` - isolated tests passed
- Mood: -
- Tomorrow's first move: Ownership drill d12 (`cargo test --test d12_scanner_peek`)

### 2026-08-27 - d12 borrowing-struct visual preview
- Duration: not recorded
- Phase / chapter: Phase 2 / Ch 2 / borrowing structs and lifetimes
- What I did: Previewed the unfamiliar `struct`, `impl`, lifetime, `&self`, and `&mut self` syntax using a bookmark-style SVG diagram; confirmed that no tldraw connector is installed in this Codex session.
- Exercises: ownership drill d12 previewed and deferred to Week 3
- Code: `topics/rust/02-ownership/visuals/d12-scanner-borrowing.svg` - created and XML-validated
- Mood: -
- Tomorrow's first move: Begin the Week 2 `strip_margin` exercise from `topics/rust/exercises/ch02-ownership.md`

### 2026-08-27 - `strip_margin`
- Duration: not recorded
- Phase / chapter: Phase 2 / Ch 2 / string slices and iterators
- What I did: Implemented `strip_margin` by iterating over lines, finding the prefix, returning the suffix as a borrowed slice, collecting the slices, and joining them into an owned `String`; reviewed the iterator and `Option` pipeline.
- Exercises: Ch 2 `strip_margin` completed (8 total exercises officially recorded)
- Code: `code/02-ownership/strip-margin` - formatted; check, test, and strict Clippy clean; no unit tests are defined yet
- Mood: -
- Tomorrow's first move: Scaffold and begin the Week 2 `split_at_mut` exercise

### 2026-08-27 - `split_at_mut`
- Duration: not recorded
- Phase / chapter: Phase 2 / Ch 2 / mutable slices, generics, and exclusive borrowing
- What I did: Implemented the generic `split_at_mut` wrapper, observed why two separately indexed mutable borrows are rejected, used the standard slice method that guarantees disjoint outputs, mutated through both returned slices, and checked split positions `0`, `len`, and beyond `len`.
- Exercises: Ch 2 `split_at_mut` completed (9 total exercises officially recorded)
- Code: `code/02-ownership/split-at-mut` - formatted; check, zero-test harness, strict Clippy, and runtime behavior verified
- Mood: -
- Tomorrow's first move: Resume ownership drill d12 in `code/02-ownership/drills-ownership/` by filling `PREDICT:` before running `cargo test --test d12_scanner_peek`

## Weekly review

> Fill in on Sunday. Five lines max. Be honest, not aspirational.

### Week 1 — Toolchain & syntax (complete)
- Sessions this week: —
- Minutes this week: —
- Shipped: FizzBuzz (three ways), temperature converter (single CLI argument), guessing game
- Skipped: none
- Where I'm slipping: session durations are not consistently recorded
- Adjustment for next week: begin ownership with move/copy/drop and keep logging each session

## Streak rules

- A "session" is **20 minutes or more** of focused work — reading, typing code, or doing exercises.
- A 0-minute day is allowed in the log but **breaks the streak** (this is intentional — the streak is the point).
- Sundays are off by design (per study plan). Sundays do not break the streak.
- If you miss two non-Sunday days in a row, the next session must be a **review** session (re-read last week's notes, redo one solved exercise) before any new material.

## Quick links

- [[study-plan|Calendar — what's planned for this week]]
- [[journal|Journal — confusions, aha-moments]]
- [[lessons/index|Lesson summaries — TL;DR per concept]]
- [[exercises/index|All exercises]]
- [[roadmap|Full roadmap]]
