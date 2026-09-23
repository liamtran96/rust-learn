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
| Streak (best) | 4 days |
| Total minutes | ~75 formally tracked + sessions with duration not recorded |
| Total sessions | 36 (formally tracked) |
| Exercises completed | 21 total (7 / 7 Ch 1 + 6 Ch 2 exercises + 8 / 8 Ch 3 exercises) |
| Projects shipped | 4 (fizzbuzz, temp-converter, guessing-game, hand-written split/dedup) |
| Current phase | Phase 3 — Types & data |
| Current week | Week 4 |
| Last session | 2026-09-23 - Point derives completed |
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

### 2026-08-28 - Ownership drill d12: borrowing struct scanner
- Duration: not recorded
- Phase / chapter: Phase 2 / Ch 2 / borrowing structs and lifetimes
- What I did: Implemented `Scanner<'a>::peek` and `advance`; decoded unfamiliar `struct`, `impl`, `Self`, method-receiver, lifetime, slicing, and `Option<char>` syntax; connected the scanner cursor pattern to parsers used in real applications.
- Exercises: ownership drill d12 completed; official exercise total remains 9
- Code: `code/02-ownership/drills-ownership/tests/d12_scanner_peek.rs` - formatting, isolated target, and full crate tests passed; strict Clippy is blocked by the earlier d07 `let_and_return` warning
- Mood: -
- Tomorrow's first move: Read `topics/rust/02-ownership/lifetimes.md`, then explain how `Scanner<'a>` prevents a borrowed scanner from outliving its source text


### 2026-09-03 - Ownership retrieval homework started
- Duration: not recorded
- Phase / chapter: Phase 2 / Ch 2 / ownership retrieval review
- What I did: Started Question 1 of topics/rust/homework/2026-08-28-retrieval-02-ownership.md; reviewed the differences between moving, copying, borrowing, and cloning, including who owns a String after each operation.
- Exercises: homework Question 1 attempted but not completed; official exercise total remains 9
- Code: none - retrieval review
- Mood: -
- Tomorrow's first move: Resume homework Question 1 by explaining why using .clone() only to silence the borrow checker can hide the intended ownership decision

### 2026-09-04 - Ownership retrieval homework completed
- Duration: not recorded
- Phase / chapter: Phase 2 / Ch 2 / ownership, borrowing, slices, and lifetimes
- What I did: Completed and reviewed all six questions in topics/rust/homework/2026-08-28-retrieval-02-ownership.md; added canonical reference answers after each complete question.
- Exercises: retrieval homework 6 / 6 reviewed; official exercise total remains 9
- Code: none - retrieval review; a temporary strip-margin experiment remains outside this closeout
- Mood: -
- Tomorrow's first move: Read topics/rust/02-ownership/lifetimes.md, then begin the full Scanner exercise from topics/rust/exercises/ch02-ownership.md

### 2026-09-07 - Borrowed Scanner implementation and tests
- Duration: not recorded
- Phase / chapter: Phase 2 / Ch 2 / borrowing structs, methods, Option, and tests
- What I did: Built the borrowed Scanner with guided syntax explanations; implemented peek and advance; wrote three tests; reviewed scope, method calls, match branch values, character versus string types, and UTF-8 byte positions. Codex removed the final redundant binding and formatted the crate at my request.
- Exercises: Ch 2 exercise 3 (borrowed Scanner) completed; official total is 10. Owned Scanner exercise 4 remains unstarted.
- Code: `code/02-ownership/scanner/` - fmt/check/three tests/strict Clippy passed; runtime printed Some('r'), Some('r'), Some('u'). Tests cover ASCII peeking, advancing, and end-of-text; Unicode behavior was explained but not tested.
- Mood: -
- Tomorrow's first move: After a short recall of method receivers and UTF-8 byte positions, begin Ch 2 exercise 4: refactor Scanner to own String in the existing crate.
- Tracker note: Streak restarts at 1 recorded session day after the unrecorded Saturday 2026-09-05; duration was not measured. No new chapter or shipping milestone completed.

### 2026-09-08 - Owned Scanner and allocation versus ownership
- Duration: not recorded
- Phase / chapter: Phase 2 / Ch 2 / owned fields, shared borrowing, and allocation
- What I did: Refactored Scanner to own String, removed the source lifetime parameter, converted all four initializers, and borrowed the field inside peek. Reviewed E0507, method receivers, caller-owned text, and why moving an existing String does not allocate another text buffer; correctly answered that the move example has just one buffer.
- Exercises: Ch 2 exercise 4 (owned Scanner and ownership comparison) completed with guided review; official total is 11.
- Code: `code/02-ownership/scanner/` - fmt/check/three tests/strict Clippy passed. Existing tests cover ASCII peeking, advancing, and end-of-text; Unicode behavior remains untested.
- Mood: -
- Tomorrow's first move: After a short recall of borrowing versus moving a String, scaffold and begin the Week 3 hand-written string-splitting task (`$new-exercise 02 split-text`).
- Tracker note: Phase 2 / Week 3 and 3 shipped projects remain unchanged. Reading checkboxes and the split/dedup shipping milestone remain open; no new chapter completion or automatic homework set.

### 2026-09-08 - Split-text scaffold and borrowing warm-up
- Duration: not recorded
- Phase / chapter: Phase 2 / Ch 2 / shared borrowing versus moving String
- What I did: Scaffolded the Week 3 splitting task and tried the supplied show_text example. Reviewed why passing &message to an &str parameter leaves message usable afterward; requested concrete code after the verbal ownership question was unclear.
- Exercises: borrowing warm-up verified; split-text implementation remains unfinished. Official totals remain 11 exercises and 3 shipped projects.
- Code: `code/02-ownership/split-text/` - formatted; fmt/check/zero-test harness/strict Clippy passed; runtime printed red,blue inside and after the call.
- Mood: -
- Tomorrow's first move: Explain why the final println in the show_text example works, then write a compiling split_text stub using the signature decoded in BRIEF.md.
- Tracker note: No week or chapter milestone completed; independent borrowing recall remains open.

### 2026-09-08 - Hand-written string splitting
- Duration: not recorded
- Phase / chapter: Phase 2 / Ch 2 / borrowed slices, vectors, and UTF-8 boundaries
- What I did: Implemented `split_text` without standard splitting methods by walking `char_indices`, slicing between byte positions, preserving empty pieces, and advancing past a separator by its UTF-8 width. Learned `Vec<T>`, generic type placeholders, built-in string methods, and Rust test syntax.
- Exercises: hand-written `split_text` completed (12 total exercises officially recorded). The combined split/dedup shipping milestone remains open.
- Code: `code/02-ownership/split-text/` - formatted; fmt/check/4 tests/strict Clippy/runtime clean. Tests cover normal, adjacent/edge, absent, empty, and Unicode cases.
- Mood: -
- Tomorrow's first move: After recalling why `char_indices` yields byte positions and why `len_utf8` matters, begin the hand-written `Vec::dedup` half of the Week 3 shipping task.
- Tracker note: Phase 2 / Week 3 and 3 shipped projects remain unchanged; no week or chapter completion and no automatic homework set.

### 2026-09-09 - Hand-written consecutive deduplication
- Duration: not recorded
- Phase / chapter: Phase 2 / Ch 2 / mutable vectors, generics, and in-place mutation
- What I did: Implemented generic `dedup_in_place` without `Vec::dedup` by walking neighboring elements with a mutable cursor, removing only consecutive duplicates, and keeping the cursor in place after removal so shifted elements are rechecked. Added five tests covering repeated runs, empty input, non-adjacent duplicates, duplicates at both edges, and `&str` elements.
- Exercises: Ch 2 exercise 5 completed (13 total exercises officially recorded); the combined hand-written split/dedup shipping milestone is complete.
- Code: `code/02-ownership/dedup-vec/` - check, 5 tests, strict Clippy, runtime, formatting, and final format check passed.
- Mood: -
- Tomorrow's first move: Read `topics/rust/02-ownership/lifetimes.md`, then explain how a lifetime annotation constrains a borrowed value without extending how long it lives.
- Tracker note: Phase 2 / Week 3 remains active with two lifetime-reading boxes unchecked; no chapter completion or automatic homework set.

### 2026-09-14 - Lifetime relationships and elision
- Duration: not recorded
- Phase / chapter: Phase 2 / Ch 2 / lifetimes
- What I did: Read the lifetime note, skimmed the lifetime cheatsheet, traced why a returned reference cannot outlive a shorter-lived input, and distinguished compile-time lifetime constraints from runtime branch selection. Recalled why an in-place dedup cursor stays at the same index after removal.
- Exercises: no new exercise; official total remains 13.
- Code: no repository code changed; a temporary `longest` snippet was compiled to confirm E0597 and then removed.
- Mood: -
- Tomorrow's first move: Without notes, explain the four compiler errors in the Ch 2 checkpoint; open the ownership, borrowing, or slices note only when an explanation is unclear.
- Tracker note: The two Week 3 lifetime-reading boxes are complete. Phase 2 / Week 3 remains active until the Ch 2 checkpoint is explained; the earlier combined Week 2 reading box remains unchecked, so no chapter completion or automatic homework set.

### 2026-09-14 - Chapter 2 ownership checkpoint
- Duration: not recorded
- Phase / chapter: Phase 2 / Ch 2 / ownership checkpoint
- What I did: Explained all four checkpoint diagnostics: overlapping shared and mutable access to a `Vec`, use after moving a `String`, incompatible returned-reference lifetime promises, and use of a reference after its owner is dropped. Corrected the idea that lifetime annotations can keep an owner alive.
- Exercises: no new exercise; official total remains 13. Chapter 2 checkpoint completed.
- Code: no exercise code changed; all 12 ownership drill test targets passed.
- Mood: -
- Tomorrow's first move: Review `topics/rust/02-ownership/ownership.md`, `borrowing.md`, and `slices.md`, then complete the remaining combined Week 2 reading checkbox before closing Chapter 2.
- Tracker note: Phase 2 / Week 3 remains active because the earlier combined Week 2 reading box is still unchecked. No chapter completion or automatic homework set yet.

### 2026-09-14 - Chapter 2 ownership closeout review
- Duration: not recorded
- Phase / chapter: Phase 2 / Ch 2 / ownership, borrowing, and slices review
- What I did: Recalled `String` moves and allocation count, corrected move-versus-drop terminology, reviewed overlapping shared and mutable `Vec` borrows and non-lexical lifetimes, and refreshed UTF-8 byte-boundary rules for string slices.
- Exercises: no new exercise; official total remains 13. Chapter 2 is complete.
- Code: no exercise code changed; paper retrieval review only
- Mood: -
- Tomorrow's first move: Begin Phase 3 / Week 4 with `topics/rust/03-types-and-traits/structs.md` and `enums.md`, then type the first examples.
- Tracker note: The remaining combined Week 2 reading checkbox is complete, advancing the official tracker to Phase 3 / Week 4. Projects shipped remain 4.

### 2026-09-14 - Shape enum and area method
- Duration: not recorded
- Phase / chapter: Phase 3 / Ch 3 / enums and pattern matching
- What I did: Modeled circles, rectangles, and triangles as data-carrying enum variants; implemented an exhaustive `area(&self)` match; demonstrated each variant; and added three unit tests.
- Exercises: Ch 3 `shape-area` completed (14 total exercises officially recorded).
- Code: `code/03-types-and-traits/shape-area/` - formatting, check, 3 tests, strict Clippy, and runtime output passed.
- Mood: -
- Tomorrow's first move: Read `topics/rust/03-types-and-traits/structs.md`, then explain how a struct differs from an enum before starting the next Ch 3 exercise.
- Tracker note: Phase 3 / Week 4 remains active; no week or chapter milestone was completed, so no homework set was generated.

### 2026-09-14 - Struct versus enum retrieval
- Duration: not recorded
- Phase / chapter: Phase 3 / Ch 3 / structs and enums
- What I did: Explained that a `Shape` enum has exactly one active variant and that each variant stores only its relevant fields; contrasted this with an all-in-one struct that could represent multiple shapes or no shape.
- Exercises: no new exercise; official total remains 14.
- Code: scaffolded `code/03-types-and-traits/network-state/` in the preceding commit; no solution code written.
- Mood: -
- Tomorrow's first move: Open `code/03-types-and-traits/network-state/BRIEF.md`, then type the enum and one initial state in `src/main.rs`.
- Tracker note: Phase 3 / Week 4 remains active; no milestone or automatic homework set.

### 2026-09-14 - Network connection state machine
- Duration: not recorded
- Phase / chapter: Phase 3 / Ch 3 / enums, pattern matching, methods, and tests
- What I did: Modeled four mutually exclusive connection states, implemented consuming transition methods for attempts, success, and failure, demonstrated successful and failed paths, and wrote three focused unit tests for the attempt count and stored strings.
- Exercises: Ch 3 network state machine completed (15 total exercises officially recorded).
- Code: `code/03-types-and-traits/network-state/` - formatting, check, 3 tests, strict Clippy, and runtime output passed.
- Mood: -
- Tomorrow's first move: After recalling `self` versus `Self` and `.` versus `::`, scaffold Ch 3 exercise 3 for the generic `largest` functions.
- Tracker note: Phase 3 / Week 4 remains active. The enum-driven state-machine library shipping milestone remains open; projects shipped remain 4.

### 2026-09-18 - Generic largest functions
- Duration: not recorded
- Phase / chapter: Phase 3 / Ch 3 / generics, trait bounds, borrowed slices, and `Option`
- What I did: Completed and reviewed three generic largest-value functions: a `Copy` version, an empty-safe owned version, and a borrowing version for non-`Copy` elements. Added four tests and clarified how `T`, trait bounds, `Option<T>`, and `Option<&T>` affect ownership.
- Exercises: Ch 3 generic `largest` exercise completed (16 total exercises officially recorded).
- Code: `code/03-types-and-traits/generic-largest/` - formatting, check, 4 tests, and strict Clippy passed.
- Mood: -
- Tomorrow's first move: Without notes, explain why an empty slice produces `None` and why a non-`Copy` winner is returned as `&T`; then begin Ch 3 exercise 4, `Pair<T, U>::swap`.
- Tracker note: Phase 3 / Week 4 remains active. The enum-driven state-machine library shipping milestone remains open; projects shipped remain 4.

### 2026-09-21 - Generic pair swap
- Duration: not recorded
- Phase / chapter: Phase 3 / Ch 3 / generic structs, consuming methods, and tests
- What I did: Recalled why an empty slice returns `None` and why a non-`Copy` winner is borrowed; implemented `Pair<T, U>::swap`, demonstrated it with two different field types, and added a focused unit test.
- Exercises: Ch 3 generic `Pair<T, U>::swap` exercise completed (17 total exercises officially recorded).
- Code: `code/03-types-and-traits/pair-swap/` - formatting, check, runtime output, 1 test, and strict Clippy passed.
- Mood: -
- Tomorrow's first move: After recalling the difference between a type parameter and a value, scaffold Ch 3 exercise 5 for the `Animal` trait (`$new-exercise 03 animal-trait`).
- Tracker note: Phase 3 / Week 4 remains active. The enum-driven state-machine library shipping milestone remains open; projects shipped remain 4.

### 2026-09-21 - Animal trait and default method
- Duration: not recorded
- Phase / chapter: Phase 3 / Ch 3 / traits, method receivers, and default methods
- What I did: Defined `Animal` with required and default methods; implemented it for `Dog` and `Cat`; demonstrated inherited behavior; and added two unit tests covering both implementations.
- Exercises: Ch 3 `Animal` trait exercise completed (18 total exercises officially recorded).
- Code: `code/03-types-and-traits/animal-trait/` - formatting, check, runtime output, 2 tests, and strict Clippy passed.
- Mood: -
- Tomorrow's first move: Recall required versus default trait methods, then scaffold Ch 3 exercise 6 (`$new-exercise 03 summary-trait`).
- Tracker note: Phase 3 / Week 4 remains active. The combined Ch 3 exercise checkbox and enum-driven state-machine shipping milestone remain open; projects shipped remain 4.

### 2026-09-22 - Static and dynamic trait dispatch
- Duration: not recorded
- Phase / chapter: Phase 3 / Ch 3 / trait bounds, trait objects, and dynamic dispatch
- What I did: Implemented `Summary` for two types, wrote generic `notify<T: Summary>` with static dispatch, wrote `notify_dyn` over a borrowed slice of boxed trait objects, and demonstrated a heterogeneous collection. Clarified why consumer functions belong outside the trait and how `Box<dyn Summary>` enables runtime dispatch.
- Exercises: Ch 3 `Summary` notification exercise completed (19 total exercises officially recorded).
- Code: `code/03-types-and-traits/summary-trait/` - formatting, check, runtime output, zero-test harness, and strict Clippy passed.
- Mood: -
- Tomorrow's first move: Recall why `Vec<Box<dyn Summary>>` can mix concrete types, then scaffold Ch 3 exercise 7 (`$new-exercise 03 typed-ids`).
- Tracker note: Phase 3 / Week 4 remains active. The combined Ch 3 exercise checkbox and enum-driven state-machine shipping milestone remain open; projects shipped remain 4.

### 2026-09-23 - Typed IDs with newtypes
- Duration: not recorded
- Phase / chapter: Phase 3 / Ch 3 / tuple structs and the newtype pattern
- What I did: Created distinct `UserId` and `OrderId` wrappers around `u64`, wrote a function that accepts only `UserId`, and observed compiler error E0308 when intentionally passing `OrderId`. Reviewed why `Vec<Box<dyn Summary>>` has one concrete outer element type while hiding different inner implementors.
- Exercises: Ch 3 typed-ID newtype exercise completed (20 total exercises officially recorded).
- Code: `code/03-types-and-traits/typed-ids/` - formatting, check, runtime output, zero-test harness, and strict Clippy passed.
- Mood: -
- Tomorrow's first move: Recall why two newtypes remain distinct despite wrapping the same primitive, then scaffold Ch 3 exercise 8 (`$new-exercise 03 point-derives`).
- Tracker note: Phase 3 / Week 4 remains active. The combined Ch 3 exercise checkbox and enum-driven state-machine shipping milestone remain open; projects shipped remain 4.

### 2026-09-23 - Point derives started
- Duration: not recorded
- Phase / chapter: Phase 3 / Ch 3 / structs, derives, and trait-generated behavior
- What I did: Scaffolded `Point`, derived six standard traits, learned that derive generates trait implementations at compile time, and added passing tests for `Debug` formatting and `PartialEq` equality. Corrected an assertion that compared formatted `String` output with a `Point` value.
- Exercises: Ch 3 point-derives exercise remains in progress; official total remains 20.
- Code: `code/03-types-and-traits/point-derives/` - formatting clean and 2 tests pass; `cargo check` reports dead code and strict Clippy fails because the normal binary does not construct `Point`. `Clone`, `Copy`, `Eq`, and `Hash` verification remains.
- Mood: -
- Tomorrow's first move: Construct and debug-print one `Point` in `main`, then rerun `cargo clippy -- -D warnings` before adding the remaining trait tests.
- Tracker note: Phase 3 / Week 4, 20 exercises, and 4 shipped projects remain unchanged; no week or chapter milestone was completed.

### 2026-09-23 - Point derives completed
- Duration: not recorded
- Phase / chapter: Phase 3 / Ch 3 / derives, generic trait bounds, and hash-based collections
- What I did: Constructed and debug-printed `Point`; verified `Copy` through post-assignment use, `Clone` and `Eq` through generic trait-bound checks, and `Hash` by storing `Point` in a `HashSet`. Clarified generic substitution, test-only item scope, and why `contains` borrows its lookup value.
- Exercises: Ch 3 point-derives exercise completed (21 total exercises officially recorded; Ch 3 exercises are 8 / 8 complete).
- Code: `code/03-types-and-traits/point-derives/` - formatting, check, 6 tests, and strict Clippy passed.
- Mood: -
- Tomorrow's first move: Begin the Week 4 enum-driven state-machine library shipping milestone.
- Tracker note: Phase 3 / Week 4 and 4 shipped projects remain unchanged. The Ch 3 exercise checkbox is complete; the state-machine shipping milestone and reading checkboxes remain open.


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
