---
title: Personal Study Plan — 1 hr/day
tags: [rust, plan, schedule]
---

# Personal Study Plan — 1 hr/day

A week-by-week calendar against [[roadmap|the Roadmap]], calibrated for **~7 hours/week** (60 min × 6 days, Sunday off). Tick the checkboxes as you go.

> **Total budget:** 16 weeks to ship a Tauri app, +4 weeks if you take Track C (unsafe). Add a buffer week every 4 weeks for life → **plan for 20 weeks** to actually finish.

## Daily structure (60 min)

```
0–15 min  · re-read yesterday's notes / glance at relevant cheatsheet
15–45 min · new material — type every example into a scratch cargo project
45–60 min · exercise from the chapter's exercises file
```

Stop at the timer even mid-problem. Resume tomorrow. The compiler is teaching you in your sleep.

**Skip Sundays.** 6 days × 70 min beats 7 × 60.

## Phase 1 — Foundations

### Week 1 — Toolchain & syntax
- [x] Install `rustup`, verify `cargo --version`
- [x] Read [[01-fundamentals/toolchain|toolchain]], [[01-fundamentals/variables|variables]], [[01-fundamentals/data-types|data types]]
- [x] Read [[01-fundamentals/functions|functions]], [[01-fundamentals/control-flow|control flow]]
- [x] Do [[exercises/ch01-fundamentals|Ch 1 exercises]] 1–7
- [x] **Ship:** FizzBuzz (3 ways), temperature converter, ✅ guessing game

## Phase 2 — Ownership (THE chapter)

### Week 2 — Ownership & borrowing

> Code first: the drills are the curriculum; the notes are the rescue rope.

- [x] Do drills d01–d08 in `code/02-ownership/drills-ownership/` — predict → run → fix → explain (its `WORKFLOW.md` has the loop)
- [ ] Read [[02-ownership/ownership|ownership]], [[02-ownership/borrowing|borrowing]], [[02-ownership/slices|slices]] — **only to unblock a drill**, ≤10 min per sitting
- [x] Do [[exercises/ch02-ownership|Ch 2 exercise]] `strip_margin`
- [x] Do [[exercises/ch02-ownership|Ch 2 exercise]] `split_at_mut`

### Week 3 — Lifetimes
- [x] Do drills d09–d11 in `code/02-ownership/drills-ownership/` (borrowed returns, `&mut` slices, tail expressions)
- [x] Do drill d12 in `code/02-ownership/drills-ownership/` (`Scanner<'a>`)
- [ ] Read [[02-ownership/lifetimes|lifetimes]] after drill d08 — read it twice; it doesn't click the first time
- [ ] Skim [[cheatsheets/lifetimes-cheatsheet|lifetimes cheatsheet]] daily
- [x] Do [[exercises/ch02-ownership|Ch 2 exercise 3]]: borrowed `Scanner` with three tests
- [x] Do [[exercises/ch02-ownership|Ch 2 exercise 4]]: refactor `Scanner` to own `String` and compare ownership choices
- [x] **Ship:** Re-implement `str::split` and `Vec::dedup` by hand

> 🛑 **Checkpoint.** Don't move on if ownership feels fuzzy. Ownership not clicking now = months of pain later. Spend an extra week if needed.

## Phase 3 — Types & data

### Week 4 — Structs, enums, traits
- [ ] Read [[03-types-and-traits/structs|structs]], [[03-types-and-traits/enums|enums]]
- [ ] Read [[03-types-and-traits/pattern-matching|pattern matching]]
- [ ] Read [[03-types-and-traits/generics|generics]], [[03-types-and-traits/traits|traits]], [[03-types-and-traits/trait-objects|trait objects]]
- [ ] Do [[exercises/ch03-types|Ch 3 exercises]]
- [ ] **Ship:** State-machine library driven by an `enum`

### Week 5 — Collections & errors
- [ ] Read [[04-collections/strings|strings]], [[04-collections/vec|Vec]], [[04-collections/hashmap|HashMap]]
- [ ] Read [[05-error-handling/result-option|Result/Option]], [[05-error-handling/question-mark|`?`]], [[05-error-handling/custom-errors|thiserror/anyhow]]
- [ ] Do [[exercises/ch04-collections|Ch 4]] + [[exercises/ch05-errors|Ch 5]] exercises
- [ ] **Ship:** Word-frequency CLI with proper error handling

## Phase 4 — Organization & quality

### Week 6 — Modules & testing
- [ ] Read [[06-modules/modules-and-paths|modules]], [[06-modules/crates-and-workspaces|workspaces]]
- [ ] Read [[07-testing/unit-tests|unit tests]], [[07-testing/integration-tests|integration tests]], [[07-testing/doc-tests|doc tests]]
- [ ] Do [[exercises/ch06-07-tooling|Ch 6–7 exercises]]
- [ ] **Ship:** Publish a small crate (private or public) with docs and CI

## Phase 5 — Idiomatic Rust

### Week 7 — Closures & iterators
- [ ] Read [[08-closures-iterators/closures|closures]] (`Fn`, `FnMut`, `FnOnce`)
- [ ] Read [[08-closures-iterators/iterators|iterators]] — adapters: `map`, `filter`, `collect`, `fold`, `zip`, `chain`, `flat_map`
- [ ] Memorize [[cheatsheets/iterator-methods|iterator cheatsheet]]
- [ ] Do [[exercises/ch08-iterators|Ch 8 exercises]]
- [ ] **Ship:** Re-solve 5 earlier exercises using only iterator chains — no explicit loops

### Week 8 — Smart pointers
- [ ] Read [[09-smart-pointers/box|Box]], [[09-smart-pointers/rc-arc|Rc/Arc]]
- [ ] Read [[09-smart-pointers/refcell|RefCell/Cell]], [[09-smart-pointers/deref|Deref/Drop]]
- [ ] Do [[exercises/ch09-smart-pointers|Ch 9 exercises]]
- [ ] **Ship:** Tree with parent pointers (forces `Rc`/`Weak`)

### Week 9 — Concurrency
- [ ] Read [[10-concurrency/threads|threads]], [[10-concurrency/channels|channels]]
- [ ] Read [[10-concurrency/shared-state|shared state]], [[10-concurrency/send-sync|Send/Sync]]
- [ ] Do [[exercises/ch10-concurrency|Ch 10 exercises]]
- [ ] **Ship:** Concurrent port scanner

### Week 10 — Async
- [ ] Read [[11-async/async-await|async/await]], [[11-async/futures|futures]], [[11-async/tokio|tokio]]
- [ ] Do [[exercises/ch11-async|Ch 11 exercises]]
- [ ] **Ship:** Async chat server with tokio

> 🛑 **Checkpoint.** You now know enough Rust to read most production code. The next phase is *applying* it.

## Phase 6 — Tauri capstone

### Week 11 — Tauri foundations
- [ ] Install Node + pnpm + OS deps ([[13-tauri/setup|13.1 Setup]])
- [ ] `pnpm create tauri-app` — pick a frontend
- [ ] Read [[13-tauri/architecture|13.2 Architecture]]
- [ ] Do [[exercises/ch13-tauri|Ch 13 exercises]] 1–4 (setup gate + commands warm-up)
- [ ] **Ship:** Button in frontend calls Rust function, renders typed result

### Week 12 — IPC: commands, events, state
- [ ] Read [[13-tauri/commands|13.3 Commands]], [[13-tauri/events|13.4 Events]]
- [ ] Read [[13-tauri/state|13.5 Managed State]]
- [ ] Do [[exercises/ch13-tauri|Ch 13 exercises]] 5–12
- [ ] **Ship:** Counter app with persisted state, progress-bar via events

### Week 13 — Plugins & native APIs
- [ ] Read [[13-tauri/plugins|13.6 Plugins]] — fs, dialog, store, notification, tray
- [ ] Configure capabilities/permissions
- [ ] Do [[exercises/ch13-tauri|Ch 13 exercises]] 13–18
- [ ] **Ship:** App reads/writes config in `$APPDATA`, shows native dialog + notification

### Week 14 — Capstone build (1/2)
- [ ] Pick a track in [[13-tauri/capstone|13.8 Capstone]]: A (notes), B (pomodoro), C (expense)
- [ ] Set up repo, scaffold UI, hit M0–M2 (repo, first command, persisted state)
- [ ] Tag `v0.0.1`

### Week 15 — Capstone build (2/2)
- [ ] Hit M3–M4 (native integration, polish)
- [ ] Read [[13-tauri/packaging|13.7 Packaging]]
- [ ] Replace icons; tune `[profile.release]`
- [ ] **Ship:** Working dev build with no `unwrap`s and no placeholder text

### Week 16 — Ship
- [ ] `pnpm tauri build` produces an installer
- [ ] Install your own bundle; use it for an hour; fix what you find
- [ ] Write README with screenshot + install steps
- [ ] Do [[exercises/ch13-tauri|Ch 13 exercises]] 25 (retrospective)
- [ ] **Ship:** Tagged `v0.1.0` ✅

> 🎉 **Goal achieved.** You shipped a Rust + Tauri desktop app. Stop here if that was the goal.

## Phase 7 — Track C: Unsafe (optional, +4 weeks)

### Week 17 — The unsafe contract
- [ ] Read [[14-unsafe/index|Ch 14 index]], [[14-unsafe/why-unsafe|14.1 Why unsafe]]
- [ ] Read [[14-unsafe/raw-pointers|14.2 Raw pointers]]
- [ ] Install Miri: `rustup +nightly component add miri`
- [ ] Do [[exercises/ch14-unsafe|Ch 14 exercises]] 1–9

### Week 18 — Aliasing, MaybeUninit, Miri
- [ ] Read [[14-unsafe/aliasing|14.3 Aliasing]], [[14-unsafe/uninit-and-cells|14.4 MaybeUninit/UnsafeCell]]
- [ ] Read [[14-unsafe/miri|14.5 Miri]]
- [ ] Do [[exercises/ch14-unsafe|Ch 14 exercises]] 10–18

### Week 19 — Patterns
- [ ] Read [[14-unsafe/patterns|14.6 Patterns]]
- [ ] Implement `MyVec<T>` Miri-clean
- [ ] Do [[exercises/ch14-unsafe|Ch 14 exercises]] 19–23

### Week 20 — FFI safety + audit
- [ ] Read [[14-unsafe/ffi-safety|14.7 FFI safety]], [[14-unsafe/review-checklist|14.8 Review checklist]]
- [ ] Do [[exercises/ch14-unsafe|Ch 14 exercises]] 24–28 (capstone: typed arena / SPSC / FFI)
- [ ] **Ship:** A Miri-clean primitive with documented `// SAFETY:` comments

## When you fall behind

Life happens. The plan is built to absorb a missed week or two. Rules:

1. **Never skip ownership (W2–3) or async (W10).** Stay an extra week if needed. Everything else can be paused.
2. **Prefer doing fewer exercises completely over many half-done.** A finished exercise teaches more than three abandoned ones.
3. **If you're 3+ weeks behind**, drop the Tauri capstone scope (pick Track B — pomodoro) or skip Phase 7 entirely. Don't drop quality on what you do build.
4. **Don't restart from scratch when you return after a break.** Re-read the cheatsheets, do one easy exercise, then continue from where you stopped.

## Sustaining the habit

- **Tell one person you're doing this.** Accountability triples completion rates.
- **Keep [[journal|journal.md]] updated weekly.** One paragraph: what clicked, what didn't, what's next.
- **Read the compiler's full output.** Always. It's the best teacher in the language.
- **Run `cargo clippy` on every project.** It's a free senior reviewer.
- **Write to your future self.** Comment the *why* in your code so Week-12-you can read Week-3-you.

## Tracking progress

```
Phase 1  ▣ ▣                    weeks 1
Phase 2  ▢ ▢ ▢ ▢                weeks 2–3
Phase 3  ▢ ▢ ▢ ▢                weeks 4–5
Phase 4  ▢ ▢                    week 6
Phase 5  ▢ ▢ ▢ ▢ ▢ ▢ ▢ ▢        weeks 7–10
Phase 6  ▢ ▢ ▢ ▢ ▢ ▢ ▢ ▢ ▢ ▢ ▢ ▢ weeks 11–16  ← v0.1.0 ships here
Phase 7  ▢ ▢ ▢ ▢ ▢ ▢ ▢ ▢        weeks 17–20  (optional)
```

Replace `▢` with `▣` as you complete each ~half-week chunk. When you can see most of them filled, you'll know how far you've come — and on the days you can't see progress, the page will remind you that you have.
