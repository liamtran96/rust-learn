---
title: Full Terminal Emulator Learning and Implementation Plan
tags: [rust, terminal-emulator, learning-plan, capstone]
date: 2026-08-24
---

# Full Terminal Emulator Learning and Implementation Plan

**Purpose:** Learn Rust by manually building a scalable, fast, GPU-accelerated, cross-platform terminal emulator with evidence-backed explanations and production-terminal comparisons.

## 1. Goal

Build a terminal emulator while learning Rust through deliberate practice.

The project should eventually provide:

- Windows, macOS, and Linux support.
- A native operating-system window.
- A custom GPU-rendered terminal surface.
- A terminal engine implemented in Rust.
- A VT parser implemented from scratch.
- Windows ConPTY and Unix PTY adapters.
- Unicode-aware text handling.
- Scrollback, selection, clipboard, resizing, colors, and configuration.
- Architecture that can later support tabs, panes, search, hyperlinks, shell integration, SSH, graphics protocols, and accessibility.
- Correctness claims supported by specifications, production source code, tests, or clearly labeled engineering decisions.

The first usable release is a solid single-session terminal. Feature-rich extensions come afterward.

## 2. Learning Outcome

By the end, Liam should be able to explain:

- The difference between a shell, terminal, console, TTY, PTY, and terminal emulator.
- How keyboard input reaches a shell process.
- How shell output returns as bytes.
- How a VT parser separates text from control sequences.
- How parsed actions mutate terminal state.
- How a screen grid represents cursor, styles, wrapping, and scrollback.
- Why Unicode code points, grapheme clusters, glyphs, and terminal cells are different.
- How text shaping and glyph rasterization work.
- How terminal state reaches a GPU renderer.
- Why UI, PTY I/O, parsing, state mutation, and rendering should not all run on one thread.
- How performance is measured without relying on misleading "fastest terminal" claims.
- How untrusted terminal output can create security risks.
- How production terminals make different compatibility and architecture decisions.
- How Rust ownership, borrowing, enums, traits, channels, threads, errors, and unsafe boundaries apply to a real systems project.

## 3. Teaching Rules

Liam writes the lab and production implementation code manually.

Agents may:

- Explain concepts and compiler messages.
- Create function/type signatures.
- Add `todo!()` bodies.
- Provide non-leading tests.
- Suggest comments for Liam to type.
- Review Liam's implementation.
- Propose one focused refactor.
- Generate a complete commented reference implementation after Liam makes a substantive first attempt and explicitly requests it.

Agents must not:

- Fill Liam's `todo!()` bodies automatically.
- Paste a reference solution into Liam's lab or product code.
- Hide executable behavior inside scaffolding.
- Use `.clone()` merely to bypass borrowing problems.
- Use `unwrap()` in durable code without a justified invariant.
- Introduce Rust concepts beyond recorded progress without teaching them first.
- Claim terminal behavior is universally correct when specifications or implementations disagree.
- Reveal answers through overly detailed pseudocode before Liam's attempt.

The learning loop is:

```text
Learn
  -> predict
  -> attempt by hand
  -> compile and test
  -> explain errors
  -> review attempt
  -> reveal commented reference when requested
  -> close reference
  -> retype or refactor by hand
  -> explain back
  -> integrate by hand
  -> verify
  -> journal
```

## 4. Evidence and Correctness Standard

No single terminal emulator is universally authoritative.

Every important technical claim must be labeled as one of:

- `specified`: directly required by a normative specification.
- `protocol-origin`: documented by the project that introduced a protocol.
- `observed`: reproduced against a reference terminal.
- `implementation-detail`: an engineering choice found in production source.
- `inference`: a conclusion derived from evidence but not explicitly stated.
- `project-decision`: behavior deliberately selected for this project.

Use this authority order:

1. Normative specification.
2. Original protocol documentation.
3. Historical hardware documentation.
4. xterm compatibility behavior.
5. Reproducible behavior across modern terminals.
6. Production source-code decisions.
7. Local project decision.

When sources disagree:

1. Record each source.
2. Create the smallest reproducible byte stream.
3. Test available reference terminals.
4. Record observed output and versions.
5. Select the project behavior.
6. Explain the compatibility consequence.
7. Add a regression fixture.

### Primary sources

- [ECMA-48 control functions](https://ecma-international.org/publications-and-standards/standards/ecma-48/)
- [DEC VT100 technical manual](https://vt100.net/dec/ek-vt100-tm-002.pdf)
- [xterm control sequences](https://www.x.org/docs/xterm/ctlseqs.pdf)
- [Ghostty VT concepts](https://ghostty.org/docs/vt/concepts/sequences)
- [Ghostty VT reference](https://ghostty.org/docs/vt/reference)
- [Ghostty source](https://github.com/ghostty-org/ghostty)
- [Alacritty source](https://github.com/alacritty/alacritty)
- [Alacritty VTE parser](https://github.com/alacritty/vte)
- [Kitty keyboard protocol](https://sw.kovidgoyal.net/kitty/keyboard-protocol/)
- [Kitty graphics protocol](https://sw.kovidgoyal.net/kitty/graphics-protocol/)
- [Unicode text segmentation](https://www.unicode.org/reports/tr29/)
- [Unicode East Asian Width](https://www.unicode.org/reports/tr11/)
- [POSIX General Terminal Interface](https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap11.html)
- [Microsoft ConPTY](https://learn.microsoft.com/en-us/windows/console/createpseudoconsole)
- [vttest](https://www.invisible-island.net/vttest/)
- [Alacritty vtebench](https://github.com/alacritty/vtebench)
- [`winit` documentation](https://docs.rs/winit/latest/winit/)
- [`wgpu` documentation](https://docs.rs/wgpu/latest/wgpu/)
- [`glyphon` documentation](https://docs.rs/crate/glyphon/latest)

Production source references must include:

- Repository URL.
- Commit SHA.
- Relevant file and symbol.
- Access date.
- Explanation of whether the cited behavior is a public contract or only an implementation detail.

## 5. Repository Structure

```text
rust-learn/
|-- AGENTS.md
|-- CLAUDE.md
|-- bacon.toml
|
|-- .agents/
|   `-- skills/
|       |-- terminal-next/
|       |   |-- SKILL.md
|       |   `-- agents/openai.yaml
|       |-- terminal-lab/
|       |   |-- SKILL.md
|       |   `-- agents/openai.yaml
|       |-- terminal-reference/
|       |   |-- SKILL.md
|       |   `-- agents/openai.yaml
|       `-- terminal-review/
|           |-- SKILL.md
|           `-- agents/openai.yaml
|
|-- .claude/
|   `-- commands/
|       |-- terminal-next.md
|       |-- terminal-lab.md
|       |-- terminal-reference.md
|       `-- terminal-review.md
|
|-- topics/rust/
|   |-- 01-fundamentals/ ... 14-unsafe/
|   |-- 15-terminal-emulator/
|   |   |-- index.md
|   |   |-- plan.md
|   |   |-- mental-model.md
|   |   |-- screen-model.md
|   |   |-- vt-parser.md
|   |   |-- terminal-semantics.md
|   |   |-- unicode-and-fonts.md
|   |   |-- scrollback-and-reflow.md
|   |   |-- pty-and-processes.md
|   |   |-- input-encoding.md
|   |   |-- concurrency.md
|   |   |-- gpu-rendering.md
|   |   |-- native-integration.md
|   |   |-- configuration.md
|   |   |-- modern-protocols.md
|   |   |-- performance.md
|   |   |-- correctness-and-security.md
|   |   |-- architecture.md
|   |   |-- compatibility.md
|   |   |-- references.md
|   |   `-- mistakes.md
|   |-- exercises/ch15-terminal-emulator.md
|   |-- homework/terminal/
|   |-- lessons/
|   |-- journal.md
|   |-- progress.md
|   |-- roadmap.md
|   `-- study-plan.md
|
|-- code/15-terminal-emulator/
|   |-- labs/
|   |   |-- 01-byte-inspector/
|   |   |-- 02-screen-grid/
|   |   |-- 03-vt-parser/
|   |   |-- 04-unicode-cells/
|   |   |-- 05-pty-probe/
|   |   |-- 06-bounded-events/
|   |   `-- 07-gpu-grid/
|   |-- reference-implementations/
|   |   `-- <created only after an attempt>/
|   `-- project/
|       |-- Cargo.toml
|       |-- Cargo.lock
|       |-- .cargo/config.toml
|       |-- crates/
|       |   |-- terminal-core/
|       |   |-- terminal-vt/
|       |   |-- terminal-pty/
|       |   |-- terminal-session/
|       |   |-- terminal-render-wgpu/
|       |   |-- terminal-app/
|       |   `-- xtask/
|       |-- fixtures/
|       |   |-- vt/
|       |   |-- unicode/
|       |   `-- recordings/
|       |-- benches/
|       |-- fuzz/
|       `-- docs/adr/
|
`-- docs/implementation-notes/
    |-- 2026-08-24-terminal-emulator-curriculum.md
    `-- 2026-08-24-terminal-learning-commands.md
```

### Folder responsibilities

- `topics/rust/15-terminal-emulator/`: durable theory and evidence.
- `topics/rust/15-terminal-emulator/index.md`: chapter hub and the single milestone table (Section 8).
- `topics/rust/exercises/ch15-terminal-emulator.md`: ordered lab specifications and prerequisites.
- `code/15-terminal-emulator/labs/`: isolated practice owned by Liam.
- `reference-implementations/`: agent-written examples revealed after attempts.
- `project/`: durable terminal emulator authored manually by Liam.
- `fixtures/`: small, original, license-safe input/output examples.
- `docs/adr/`: expensive-to-reverse architecture decisions.
- `journal.md`: substantive questions and verified answers.
- `progress.md`: factual session status.
- `mistakes.md`: durable conceptual misunderstandings.
- `homework/terminal/`: generated retrieval reviews.

### Just-in-time authoring

Only `plan.md` exists today. Everything else in the tree above is created on demand:

- A lesson file (`mental-model.md`, `screen-model.md`, …) is written when its stage becomes `ready` — never all of them up front.
- `exercises/ch15-terminal-emulator.md` starts as an ordered stage list; per-lab specifications are added one at a time as each lab is scaffolded.
- Lab crates, skills, Bacon jobs, and the project workspace follow the setup sequence in Section 23.

Do not create:

- A second terminal journal.
- A second progress tracker.
- A top-level Cargo workspace.
- Empty future product crates.
- A solutions directory inside Liam's labs.
- Vendored copies of Ghostty, Alacritty, xterm, or Kitty.
- A plugin architecture before real extension cases exist.

## 6. Lesson Format

Every terminal lesson uses:

```markdown
# <Subsystem>

## Why this exists
The practical problem this subsystem solves.

## Mental model
A plain-language model and data-flow diagram.

## Vocabulary
Terms with precise definitions.

## Invariants
Rules that must always remain true.

## Algorithm
State machine or data structure with worked examples.

## Rust concepts
Only concepts already learned or an explicit prerequisite.

## Production comparison
Ghostty, Alacritty, xterm, and Kitty choices.

## Evidence
Normative sources, pinned source commits, and observed behavior.

## Project decision
What this project adopts and why.

## Lab
A bounded task Liam implements.

## Verification
Tests, fixtures, compatibility checks, or benchmarks.

## Common mistakes
Likely terminal and Rust misunderstandings.

## Explain-back
Questions Liam answers without notes.

## Next dependency
What becomes available after completing this lesson.
```

## 7. Weekly and Session Cadence

### Entry gate and timeline

The terminal project starts **after the Tauri capstone ships** (roadmap Week 12). Until then, the main Rust curriculum keeps every session. Once the capstone is done, the terminal becomes the main project and takes the daily study slot: five sessions per week at ~60 minutes each.

Hard knowledge floor regardless of calendar: no terminal session begins before the structs/enums/matching chapter is complete and journaled. Stage 1 requires it; later stages add their own prerequisites (Section 12).

Expectation setting — this is a multi-month project even at full pace. Rough stage estimates at 5 sessions/week:

| Stages | Estimate |
|---|---|
| 1–2 (mental model, screen grid) | ~2 weeks |
| 3–4 (parser, VT semantics) | ~3–4 weeks |
| 5–6 (Unicode, scrollback) | ~2–3 weeks |
| 7–9 (PTY, input, concurrency) | ~3–4 weeks |
| 10–12 (GPU, native shell, v1) | ~4–6 weeks |
| 13–15 (expansion, protocols, hardening) | open-ended |

These are planning aids, not deadlines. Progress is measured by milestone status, never by calendar.

### Session structure

```text
10 min  Retrieve the previous terminal concept from memory
10 min  Read the next lesson section
10 min  Inspect its primary source or production example
10 min  Predict behavior and draw the data flow
15 min  Implement one small lab step by hand
 5 min  Run checks and record the next action
```

If a lab is unfinished, continue it next session. Do not start another terminal subsystem merely to maintain momentum.

A terminal session may stop at any point. Record the exact next action rather than rushing to complete the milestone.

## 8. Learning Status Model

Milestone state lives in **one place**: a milestone table in `topics/rust/15-terminal-emulator/index.md` with the columns `stage`, `status`, `flags`, `Rust prerequisite`, and `next action`. `$terminal-next` reads this table; no other tracker duplicates it (see the "no second progress tracker" rule in Section 5).

Each terminal milestone uses six statuses:

```text
blocked -> ready -> attempted -> reviewed -> integrated -> verified
```

Definitions:

- `blocked`: required Rust or terminal knowledge is not completed.
- `ready`: all prerequisites are satisfied; reading and scaffolding happen here.
- `attempted`: Liam wrote substantive implementation code.
- `reviewed`: the attempt and errors were discussed.
- `integrated`: the understood behavior was added to the durable project.
- `verified`: required tests, checks, evidence, and explain-back pass.

Three events are recorded as flags on the milestone row rather than as statuses, because they can occur in any order between `reviewed` and `verified`:

- `reference-revealed`: a full commented example was explicitly requested.
- `retyped`: Liam reproduced or refactored the idea manually.
- `explain-back`: Liam correctly explained the rules and tradeoffs (required before `verified`).

## 9. Cross-Agent Commands

Canonical workflow instructions live in `.agents/skills/`.

Each skill follows the existing repository convention (`journal`, `next`, `homework`, `new-exercise`): a `SKILL.md` plus an `agents/openai.yaml` adapter. Claude Code adapters live in `.claude/commands/` — the same convention as the existing `.claude/commands/journal.md` — and point to the canonical workflow. Claude uses `/terminal-review`; Codex uses `$terminal-review`; generic agents are instructed through `AGENTS.md` to read the corresponding `SKILL.md`. Do not introduce a `.claude/skills/` directory unless the four existing skills migrate to it in the same change.

Automatic discovery remains enabled so natural requests such as "review what I learned about VT parsing" can activate the correct workflow.

### `$terminal-next [focus]`

Select one terminal task.

It reads:

- Official Rust progress.
- Current terminal milestone states.
- Last journal entry.
- Lab prerequisites.
- Existing lab and project code.
- Failed tests or unfinished explain-back questions.

Priority:

1. Continue an explicit next action.
2. Resolve an unfinished review or test.
3. Complete a pending explain-back.
4. Choose the first ready terminal milestone.
5. If blocked, return the exact Rust prerequisite.
6. Never recommend multiple competing tasks.

Output:

```text
Terminal milestone: <name and status>
Rust prerequisite: <completed or blocking concept>
Next action: <one action>
Read: <lesson>
Evidence: <primary source>
Write in: <lab or project path>
Verify with: <one command>
Timebox: <estimate>
Afterward: <next unlocked behavior>
```

### `$terminal-lab <number-or-slug>`

Scaffold an independent exercise.

It must:

- Match an official exercise.
- Validate prerequisites.
- Refuse unknown, duplicate, or blocked labs.
- Create an independent Cargo crate.
- Add a concise `BRIEF.md`.
- Add documented signatures and `todo!()` bodies.
- Add tests only when they do not expose the algorithm.
- Register Bacon jobs.
- Avoid all implementation behavior.

Example scaffold:

```rust
/// Stores the visible terminal dimensions in cells.
///
/// Both dimensions must remain greater than zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TerminalSize {
    columns: usize,
    rows: usize,
}

/// Moves to the next row and scrolls when the bottom margin is reached.
///
/// Implement this using the active scrolling region rather than assuming
/// that the entire screen always scrolls.
fn line_feed(&mut self) {
    todo!("Liam implements line-feed behavior");
}
```

### `$terminal-reference <number-or-slug>`

Reveal a complete commented reference implementation.

Preconditions:

- The lab exists.
- Liam wrote a substantive first attempt.
- The attempt or its failure was reviewed.
- Liam explicitly requested the reference.

Behavior:

1. Preserve Liam's code.
2. Create a separate reference crate.
3. Remain inside completed Rust knowledge.
4. Implement only the current lab scope.
5. Add explanation comments and evidence citations.
6. Run formatting, checking, tests, and Clippy.
7. Explain the reference section by section.
8. Ask Liam to close it.
9. Ask Liam to retype or refactor his lab manually.
10. Never integrate the reference into the product.

### `$terminal-review [topic] [mode]`

Modes:

- `mixed`
- `recall`
- `trace`
- `code`
- `source`
- `architecture`
- `performance`

Default mixed review contains:

1. One terminology or invariant question.
2. One byte/parser/grid trace.
3. One Rust ownership or code-reading problem.
4. One architecture or performance tradeoff.
5. One primary-source verification task.

Save as:

```text
topics/rust/homework/terminal/YYYY-MM-DD-<topic>-review.md
```

Do not include solutions before the attempt.

When grading:

- Mark `correct`, `partly correct`, or `retry`.
- Grade reasoning separately from conclusion.
- Verify terminal claims against primary sources.
- Verify Rust claims against official Rust documentation.
- Compile or test exact behavior when practical.
- Ask a narrower retry before revealing the full answer.
- Use `$journal terminal` only after the review is complete.

### Existing commands

- `$next`: next main Rust lesson.
- `$new-exercise`: scaffold ordinary Rust exercises.
- `$homework terminal`: review Rust concepts using terminal scenarios.
- `$journal terminal`: record the completed session.

## 10. Reference Code and Comment Policy

Liam wants real code with comments, then manually types it again.

Reference code is revealed after the first attempt.

Use comments for:

- Public contracts.
- Invariants.
- Terminal behavior.
- Algorithmic reasoning.
- Safety boundaries.
- Compatibility choices.
- Source citations.
- Non-obvious performance decisions.

Do not use comments for:

- Restating obvious syntax.
- Translating every Rust line into English.
- Hiding unexplained complexity.
- Defending unnecessary clones or allocations.
- Repeating information already clear from a good name.

Example:

```rust
/// Applies a line-feed control to the active screen.
///
/// LF moves the cursor vertically. It does not automatically perform
/// carriage return unless the terminal's newline mode requires it.
fn line_feed(&mut self) {
    // Keep the cursor inside the active scrolling region.
    // When it is already on the bottom margin, the terminal scrolls
    // the region instead of moving the cursor outside the grid.
    if self.cursor.row == self.scroll_region.bottom {
        self.scroll_region_up(1);
    } else {
        self.cursor.row += 1;
    }
}
```

During review, the agent proposes comments, but Liam types them into his implementation.

## 11. Detailed Curriculum and Build Order

| Stage | Study | Algorithms and invariants | Practice | Product result |
|---|---|---|---|---|
| 1 | Terminal mental model | Byte flow, process boundaries, terminal versus shell | Byte inspector | Complete system diagram |
| 2 | Screen grid | Flat indexing, cursor bounds, CR, LF, wrapping, scroll regions | Screen-grid lab | `terminal-core` begins |
| 3 | Parser structure | Incremental deterministic finite-state machine | VT-parser lab | `terminal-vt` begins |
| 4 | VT semantics | CSI parameters, SGR, modes, erase/insert, alternate screen | Parser-to-grid fixtures | Basic compatible terminal state |
| 5 | Unicode | UTF-8, graphemes, combining marks, cell width | Unicode-cells lab | Unicode-aware cell interface |
| 6 | Scrollback | Ring/page storage, logical lines, reflow, stable positions | Resize/reflow properties | Bounded history |
| 7 | PTYs | Unix master/slave and Windows ConPTY | PTY probe | `terminal-pty` |
| 8 | Input | Legacy keys, modes, mouse, paste, IME | Input encoder fixtures | Bidirectional interaction |
| 9 | Concurrency | Workers, bounded queues, snapshots, shutdown | Bounded-events lab | `terminal-session` |
| 10 | GPU rendering | Glyph atlas, batching, dirty rows, frame pacing | GPU-grid lab | `terminal-render-wgpu` |
| 11 | Native shell | Window events, DPI, clipboard, lifecycle | Platform smoke tests | `terminal-app` |
| 12 | Single-session v1 | Configuration, selection, search basics | End-to-end scenarios | Usable terminal |
| 13 | Expansion | Tabs, panes, hyperlinks, shell integration | Feature slices | Feature-rich architecture |
| 14 | Advanced protocols | Kitty keyboard/graphics, synchronized output | Protocol-specific fixtures | Modern terminal support |
| 15 | Hardening | Fuzzing, security, benchmarks, accessibility | Release audit | Stable releases |

## 12. Curriculum-to-Rust Gates

### Ownership and borrowing

Build:

- `TerminalSize`
- `Cursor`
- `Cell`
- Fixed screen grid
- Borrowed row views
- Resize behavior

Do not introduce shared concurrent state yet.

### Structs, enums, and matching

Build:

- Parser states
- `TerminalAction`
- Style enums
- Color enums
- Mode enums
- Parser dispatch

### Collections and errors

Build:

- Scrollback
- Style tables
- Parameter buffers
- Configuration values
- Typed parser and grid errors

### Modules and testing

Build:

- Initial Cargo workspace.
- `terminal-core` and `terminal-vt` separation.
- Unit and integration fixtures.
- Public documentation.
- Compatibility matrix.

### Smart pointers and concurrency

Build:

- Owned terminal session.
- PTY read/write workers.
- Bounded command and event channels.
- Immutable render snapshots.
- Clean shutdown.

### Async

Study event-driven I/O and cancellation. Do not adopt async merely because it is available; blocking PTY worker threads remain acceptable when they produce a simpler ownership model.

### Unsafe and FFI

Audit:

- Windows handles.
- Unix file descriptors.
- Raw window handles.
- GPU surface lifetimes.
- Any platform calls requiring unsafe.
- Resource cleanup and thread ownership.

## 13. Product Architecture

Dependency direction, stated explicitly as "X depends on Y":

```text
terminal-vt          -> terminal-core
terminal-session     -> terminal-core, terminal-vt, terminal-pty
terminal-render-wgpu -> terminal-core
terminal-app         -> terminal-core, terminal-session, terminal-render-wgpu
terminal-core        -> (nothing in this workspace)
terminal-pty         -> (nothing in this workspace)
```

No dependency cycles are allowed.

### Allowed dependencies

Build-versus-buy is decided up front so scope cannot drift mid-project:

- **PTY**: adapters are hand-written (a stated learning goal). The binding layer is `windows-sys` for ConPTY and `libc` or `rustix` for Unix `openpty`. `portable-pty` is off-limits as a dependency — it would do the learning — but is fair game to read as production source.
- **Unicode data**: `unicode-width` and `unicode-segmentation`. Implementing the Unicode databases by hand is out of scope; applying them to cells is in scope.
- **Windowing and rendering**: `winit`, `wgpu`, `glyphon`/`cosmic-text`, and `softbuffer` for the CPU fallback (Section "Risks and Descope").
- **Clipboard**: `arboard`.
- **Configuration**: `serde` + `toml`.
- **Testing**: `proptest` (property tests), `insta` (snapshot fixtures), `criterion` (benchmarks), `cargo-fuzz` (fuzzing, Linux CI only).

All crates use `edition = "2024"`, matching the rest of the repository. Pin `wgpu` and `winit` versions and upgrade them deliberately, one at a time — both change their public APIs frequently, and an accidental double upgrade turns one migration into two.

### `terminal-core`

Responsibilities:

- Grid and scrollback.
- Cursor and styles.
- Terminal modes.
- Primary and alternate screens.
- Selection.
- Resize and reflow.
- Damage tracking.
- Immutable render views.

Public concepts:

- `TerminalSize`
- `TerminalAction`
- `TerminalState`
- `RenderSnapshot`
- `DamageSet`
- `Selection`
- `TerminalEvent`

Cell storage remains private so memory representation can change without breaking callers.

### `terminal-vt`

Responsibilities:

- Incremental UTF-8 handling.
- Parser state machine.
- ESC, CSI, OSC, DCS, and APC framing.
- Parameter parsing.
- Bounded payload handling.
- Conversion into `TerminalAction`.

It does not:

- Own a PTY.
- Mutate a window.
- Render.
- Directly change the grid.

### `terminal-pty`

Responsibilities:

- Spawn a child shell.
- Read output.
- Write input.
- Resize the pseudoterminal.
- Wait for exit.
- Shut down safely.

Implement:

- Windows ConPTY backend.
- Unix PTY backend for Linux and macOS.
- Fake backend for deterministic tests.

### `terminal-session`

Responsibilities:

- Own mutable parser and terminal state.
- Coordinate PTY readers and writers.
- Accept input, resize, and shutdown commands.
- Coalesce update notifications.
- Publish snapshots.
- Report process exit and errors.

The native UI thread must never block waiting for PTY I/O.

### `terminal-render-wgpu`

Responsibilities:

- Convert cells into positioned glyphs.
- Shape text using `glyphon`/`cosmic-text`.
- Manage glyph cache and texture atlas.
- Batch backgrounds and decorations.
- Draw cursor and selection.
- Redraw damaged rows.
- Recover from surface loss.
- Handle DPI changes.

It does not know about shells, PTYs, or VT parsing.

### `terminal-app`

Responsibilities:

- Native `winit` event loop.
- Window lifecycle.
- Keyboard, mouse, focus, and IME.
- Clipboard.
- Configuration.
- Session registry.
- Platform integration.
- Renderer wake-up and presentation.

Use a `SessionId` even for the first single-session release so later tabs and panes do not require rewriting the session boundary.

## 14. Runtime Data Flow

```text
Child shell
   | UTF-8 text and VT sequences
   v
PTY reader
   | bounded byte batches
   v
VT parser
   | TerminalAction
   v
Terminal state
   | DamageSet + RenderSnapshot
   v
Session notification
   | coalesced wake-up
   v
Native event loop
   |
   v
GPU renderer
   |
   v
Window surface
```

Input travels in reverse:

```text
Keyboard/mouse/IME
   | normalized event
   v
Mode-aware input encoder
   | bytes
   v
Session command
   |
   v
PTY writer
   |
   v
Child shell
```

Resize:

```text
Window pixel size
   | DPI and cell metrics
   v
Rows and columns
   |-- terminal-core resize/reflow
   |-- PTY resize
   `-- renderer viewport update
```

## 15. Scalability Rules

- Keep terminal state independent from rendering.
- Keep parsing independent from semantic execution.
- Keep PTY behavior behind platform adapters.
- Keep all cell storage private.
- Use bounded queues.
- Coalesce repeated redraw notifications.
- Cap scrollback and protocol payload memory.
- Add traits only at real replaceable boundaries.
- Add crates only when a subsystem becomes independently meaningful.
- Prefer a clear scalar algorithm before SIMD optimization.
- Benchmark before changing data structures for performance.
- Record expensive-to-reverse decisions as ADRs.
- Do not create a public plugin API until at least two real extension cases exist.
- Keep tabs and pane layout outside `terminal-core`.
- Keep renderer snapshots immutable from the renderer's perspective.
- Ensure shutdown and ownership are explicit rather than relying on process exit.

Future features should extend existing boundaries:

- Tabs/panes: session registry and layout tree.
- Search: scrollback snapshot consumer.
- Hyperlinks: parser action, cell metadata, hit testing.
- SSH: new session transport.
- Graphics: bounded protocol resources and renderer layers.
- Accessibility: additional snapshot consumer.
- Shell integration: OSC parsing and session metadata.
- Plugins: command/event API added only after concrete use cases.

## 16. First Usable Release

Version `0.1.0` includes:

- One shell session.
- Windows, macOS, and Linux builds.
- Native window.
- GPU-rendered text.
- UTF-8 and practical Unicode support.
- 16-color, 256-color, and true-color SGR.
- Bold, italic, underline, inverse, and cursor styles.
- Resize.
- Primary and alternate screens.
- Configurable bounded scrollback.
- Selection.
- Copy and paste.
- Bracketed paste.
- Configurable font, size, colors, cursor, shell, and scrollback.
- Clean child exit handling.
- Useful diagnostic errors.
- Cross-platform CI.
- No routine `unwrap()`.
- No undocumented `unsafe`.

Deferred until after v1:

- Tabs and panes.
- SSH.
- Kitty graphics.
- Sixel.
- Extensive shell integration.
- Plugin API.
- Full settings GUI.
- Remote control.
- Session persistence.
- Advanced accessibility.
- Complete bidirectional terminal layout.

## 17. Risks and Descope

The pipeline must never depend on the hardest milestone landing first. Named risks and their fallbacks:

- **GPU rendering stalls the project (biggest risk).** Stage 10 (`wgpu`, glyph atlas, batching) is the steepest single step. Fallback: an intermediate milestone renders the grid into a **CPU-drawn window using `softbuffer` + `cosmic-text`**. The terminal is then usable end-to-end — PTY → parser → grid → visible window — before any `wgpu` code exists. The v1 scope in Section 16 still requires GPU rendering; the fallback exists so a `wgpu` struggle delays polish, not the whole product.
- **No visible progress for months kills motivation.** The earliest "it's alive" demo comes after Stage 4: real shell output fed through the real parser into the real grid, printed to stdout as a rendered frame. This requires no window, no PTY writer, and no renderer — only `terminal-vt` and `terminal-core`.
- **Windows-only development hides platform bugs.** Unix PTY code cannot be exercised locally. Mitigation: the fake PTY backend keeps `terminal-session` testable everywhere, and the CI matrix (Section 19) is the arbiter for macOS/Linux behavior from the first PTY milestone onward.
- **Scope creep via "one more protocol."** Anything not in the Section 16 v1 list is deferred by default; adding it early requires an ADR explaining why.

## 18. Performance Plan

Measure separate dimensions:

- PTY read throughput.
- Parser throughput.
- Terminal-state mutation throughput.
- Snapshot creation.
- Lock duration.
- Render preparation.
- GPU frame time.
- Input-to-present latency.
- Idle CPU usage.
- Scrollback memory.
- Glyph-cache memory.
- Frame consistency under heavy output.

Rules:

- Benchmark release builds only.
- Record hardware, OS, terminal dimensions, font, and workload.
- Use both synthetic fixtures and recorded real workloads.
- Do not claim general speed from vtebench alone; its own documentation says it measures PTY-read performance rather than total latency or frame rate.
- Establish a baseline before optimizing.
- Investigate regressions greater than 10%.
- Prefer profiling evidence over intuition.
- Add SIMD only after scalar correctness and profiling identify parsing as a meaningful bottleneck.
- Avoid unbounded "render every parser update" behavior.
- Measure lock contention before changing synchronization primitives.

Stress scenarios:

- At least 100 MB of plain output.
- Rapid full-screen updates.
- Unicode-heavy output.
- Frequent resizing.
- Maximum configured scrollback.
- Slow renderer with fast PTY output.
- Slow PTY writer with rapid input.
- Window minimize/restore.
- GPU surface loss where testable.

## 19. Correctness and Testing Plan

### Unit tests

Cover:

- Grid indexing.
- Cursor bounds.
- Wrapping.
- Pending-wrap behavior.
- CR versus LF.
- Scroll regions.
- Insert/delete.
- Erase operations.
- Primary/alternate screens.
- SGR state.
- Resize.
- Scrollback eviction.
- Selection.
- Unicode cell width.
- Combining sequences.
- Input encoding.

### Parser tests

Feed every sequence:

- As one chunk.
- One byte at a time.
- Split at every possible boundary.
- With incomplete termination.
- With cancellation bytes.
- With invalid UTF-8.
- With excessive parameters.
- With oversized OSC, DCS, or APC payloads.
- With unknown sequences.

The parser must always consume input or deliberately wait for more bytes. It must never enter an accidental infinite loop.

### Property tests

Check:

- Cursor never leaves valid bounds.
- Grid dimensions match `TerminalSize`.
- Resize preserves required logical content.
- Scrollback never exceeds its cap.
- Parser state remains valid after arbitrary input.
- Damage ranges remain inside the grid.
- Repeated reset returns to the same baseline state.

### Compatibility tests

Use:

- `vttest`.
- xterm control-sequence fixtures.
- Small Ghostty, Alacritty, and Kitty comparison cases.
- Recorded application output.
- Explicit compatibility decisions.

### Fuzzing

`cargo-fuzz` (libFuzzer) does not work reliably on Windows MSVC, and the primary development machine is Windows. Therefore:

- Fuzz targets live in the repository but **run in CI on `ubuntu-latest`** as a scheduled or manually triggered job, never as a local Windows step.
- The local Windows substitute is `proptest` with high case counts over the same input surfaces.

Fuzz:

- VT byte parser.
- Parameter parser.
- OSC/DCS/APC termination.
- Unicode chunk handling.
- Resize/reflow sequences.
- Input encoder.
- Configuration parser.

### Cross-platform verification

CI matrix:

- `windows-latest`
- `macos-latest`
- `ubuntu-latest`

Run:

```text
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

Interactive GPU, DPI, clipboard, keyboard, IME, and packaged-binary tests remain manual release requirements.

## 20. Security Plan

Treat PTY output as untrusted.

Protect against:

- Oversized OSC, DCS, and APC payloads.
- Infinite or unterminated control strings.
- Excessive parameters.
- Clipboard writes through OSC 52.
- Misleading window titles.
- Unsafe hyperlinks and local file paths.
- Dangerous pasted control characters.
- Image decompression bombs.
- Unbounded image or glyph memory.
- Terminal replies leaking unexpected data.
- Remote-control commands.
- Escape-sequence log injection.
- Resource exhaustion from rapid updates.
- Child-process and handle leaks.
- Unsafe code without documented invariants.

Security-sensitive features require:

- Explicit size limits.
- Safe defaults.
- A threat-model note.
- Negative tests.
- No silent trust of paths supplied by terminal output.
- Clear user control for clipboard or remote-control behavior.

## 21. Shell Commands

Initially register Bacon jobs:

```text
bacon terminal-check
bacon terminal-test
bacon terminal-clippy
bacon terminal-run
bacon terminal-bench
```

Each lab receives:

```text
bacon terminal-lab-<NN>
bacon terminal-lab-<NN>-run
```

After modules and workspaces are learned, add `xtask`:

```text
cargo xtask verify
cargo xtask test-vt
cargo xtask replay
cargo xtask compat
cargo xtask bench
cargo xtask fuzz
cargo xtask package
```

`cargo xtask verify` must:

1. Print every command.
2. Run formatting in check-only mode.
3. Run workspace checks.
4. Run tests.
5. Run Clippy with warnings denied.
6. Stop on the first failure.
7. Return the failing exit status.
8. Never rewrite Liam's source.

## 22. Journal and Review Rules

After a completed terminal session:

- Append the date and topic to `journal.md`.
- Record every substantive question.
- Include a verified technical answer.
- Include an example or analogy.
- Link the relevant terminal note.
- Update `progress.md`.
- Update terminal milestone status.
- Record meaningful misconceptions in terminal `mistakes.md`.
- Update the roadmap only when the milestone is genuinely complete.

Do not update completion merely because:

- A reference was revealed.
- Code compiled once.
- A test was copied.
- The concept was read.
- An agent supplied an explanation.

Completion requires implementation, verification, and explain-back.

## 23. Initial Repository Setup Sequence

When implementation begins:

1. Create the terminal curriculum directory and this plan file.
2. Create the terminal index with the milestone table (Section 8), reference ledger, compatibility matrix, and mistake log. Stage 1's prerequisite in the table is the structs/enums/matching chapter.
3. Add the terminal exercise index (stage list only; per-lab specs are added just in time per Section 5).
4. Create canonical cross-agent skills under `.agents/skills/` (each with `SKILL.md` + `agents/openai.yaml`).
5. Add Claude Code adapters under `.claude/commands/`.
6. Verify that the existing `journal` and `homework` skills accept a `terminal` argument; extend their `SKILL.md` files if they do not.
7. Update `AGENTS.md` and `CLAUDE.md` with discovery instructions.
8. Append a post-capstone phase (Weeks 13+) to `topics/rust/roadmap.md` and `topics/rust/study-plan.md` pointing at this plan, activated only when the Tauri capstone milestone is ticked.
9. Add terminal progress fields without creating a second tracker.
10. Add initial Bacon jobs only when their target crates exist.
11. Create the first lab specification.
12. Scaffold only the first ready lab.
13. Create the durable project workspace with only `terminal-core` when the relevant integration milestone begins.
14. Create the required implementation notes.
15. Validate commands and discovery before beginning the first lab.

## 24. Acceptance Criteria

The learning system is ready when:

- The detailed plan exists inside the repository.
- Terminal lessons have an evidence-aware template.
- Milestones contain explicit Rust prerequisites.
- `$terminal-next` returns one valid task.
- `$terminal-lab` creates signatures and TODOs without solutions.
- `$terminal-reference` refuses unattempted labs.
- `$terminal-review` tests only completed knowledge.
- Codex, Claude Code, and generic agents can discover the canonical workflows.
- Reference implementations are isolated from Liam's code.
- The existing journal, progress, and homework workflows remain authoritative.
- No top-level Cargo workspace is introduced.
- Liam can begin with one small task appropriate to current ownership knowledge.

The terminal product reaches v1 when:

- A default shell starts on all three operating systems.
- Input and output work through native PTYs.
- Required VT behavior passes the defined compatibility suite.
- Unicode, color, resizing, scrollback, selection, clipboard, and configuration work.
- Rendering is GPU accelerated.
- Output pressure cannot create unbounded queues or memory.
- Verification passes on Windows, macOS, and Linux.
- Remaining protocol and platform limitations are documented.
- Liam can explain the entire input/output/rendering pipeline without relying on the source code.

## 25. Final Assumptions

- Liam writes all exercise and production behavior manually.
- Complete commented examples are allowed only after a substantive attempt.
- Reference implementations remain tracked for future study.
- Agents suggest final comments; Liam types them.
- The terminal project starts after the Tauri capstone ships and then takes the daily study slot (five sessions per week); structs/enums/matching is the hard knowledge floor for any terminal session.
- The terminal model, VT parser, input encoder, and PTY adapters are implemented in the project.
- Unicode databases, text shaping, GPU APIs, and operating-system bindings use maintained libraries.
- Windows, macOS, and Linux remain build targets from the first platform-integration milestone.
- Native UI means a native OS window and integrations surrounding a custom GPU-rendered terminal canvas.
- Correctness means evidence-backed and reproducible, with ambiguity explicitly documented.
- Scalability comes from stable boundaries, private internals, bounded resources, tests, and measured evolution--not premature abstraction.
