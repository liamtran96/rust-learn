---
title: 13.8 Capstone — Pick a Track and Ship
tags: [rust, tauri, capstone, project]
---

# 13.8 Capstone — Pick a Track and Ship

The point of the capstone is **to ship something real, not perfect**. Pick the track that sounds least boring and bias toward done.

> Reach for the [official Develop guide](https://v2.tauri.app/develop/) and [Plugin index](https://v2.tauri.app/plugin/) constantly. Copy from official examples; don't invent.

## Track A — Markdown Notes (recommended default)

A local-first notes app: create, edit, search, persist as `.md` files on disk.

**Why this track:** exercises every Tauri primitive — fs plugin + dialog for import, commands for CRUD, state for the open document, events for autosave indicators, store plugin for settings. No SQL, no async I/O nightmares. You'll finish.

**Stack**
- Frontend: Svelte or React + Tailwind + a markdown renderer (`marked` or `markdown-it`)
- Editor: a textarea is fine; CodeMirror 6 if you want syntax highlighting
- Backend: `tauri-plugin-fs`, `tauri-plugin-dialog`, `tauri-plugin-store`
- Storage: one `.md` file per note in `$APPDATA/<your-app>/notes/`, metadata sidecar in `index.json`

**Milestones**
1. **List + open** — show all `.md` files in the notes dir; click → load text into editor.
2. **Edit + save** — debounce 500ms, write to disk, emit `note-saved` event for a "Saved ✓" indicator.
3. **Create + delete** — `+` button creates `untitled-<n>.md`; delete confirms via native dialog.
4. **Search** — full-text search across all notes (just `String::contains` is enough for v1).
5. **Settings** — theme (light/dark), notes folder location, font size — all in `tauri-plugin-store`.
6. **Polish** — keyboard shortcuts (Cmd/Ctrl+N, Cmd/Ctrl+S), drag-to-reorder, export to PDF (stretch).
7. **Ship** — icon, identifier, README, signed (or unsigned) installer.

---

## Track B — Pomodoro Tray Timer

A focus timer that lives in the system tray and pings you every 25 minutes.

**Why this track:** narrower scope, but pushes you on tray icons, native notifications, background timers, and tray-only-no-window UX.

**Stack**
- Frontend: tiny — settings + stats. Could be one screen.
- Backend: `tauri::tray`, `tauri-plugin-notification`, `tauri-plugin-store` for stats history
- Timer: `tokio::time::interval` running in a spawned task; emit events to update the tray menu label

**Milestones**
1. **Tray icon only** — app starts hidden; tray menu has Start/Pause/Skip/Quit.
2. **Tomato cycle** — 25min focus, 5min break, 4 cycles → 15min long break.
3. **Notifications** — native notification on phase change.
4. **Stats** — count completed pomodoros per day, persist to store; show in a small window.
5. **Settings** — durations, sound on/off, autostart break.
6. **Autostart on login** — `tauri-plugin-autostart`.
7. **Ship.**

---

## Track C — Expense Tracker (SQLite)

A local expense ledger with charts. Hardest of the three but real.

**Why this track:** SQL via `tauri-plugin-sql`, real domain modeling, a chart library on the frontend, CSV import/export. Skip if it's your first Rust project.

**Stack**
- Frontend: React + a chart lib (Recharts, Chart.js)
- Backend: `tauri-plugin-sql` (SQLite), `tauri-plugin-dialog` for CSV import/export, `tauri-plugin-fs`
- Schema: `expenses(id, amount_cents, category, note, date)`, `categories(id, name, color)`

**Milestones**
1. **Schema + migrations** — tauri-plugin-sql migrations on first run.
2. **CRUD commands** — `add_expense`, `list_expenses(filter)`, `update_expense`, `delete_expense`. All return `Result<_, AppError>`.
3. **Aggregations** — totals by category by month; expose as a single command returning a `Vec<MonthlyTotal>`.
4. **Charts** — render the aggregations on the frontend.
5. **CSV import/export** — open dialog, parse with `csv` crate, batch-insert.
6. **Ship.**

---

## Universal milestones (apply to whichever track you pick)

| # | Milestone | What "done" looks like |
|---|---|---|
| M0 | Repo + scaffold | `pnpm create tauri-app`, push to GitHub, CI runs `cargo check` |
| M1 | One real command end-to-end | Frontend button → typed Rust call → typed response, with `Result<_, AppError>` |
| M2 | State + persistence | App restart preserves user data |
| M3 | Native integration | At least one of: dialog, notification, tray, menu |
| M4 | Polish | Icons replaced, app name set, no `console.log`s, no `unwrap`s in commands |
| M5 | Ship | `pnpm tauri build` produces an installer; you've installed and used it |
| M6 | README | Screenshot, install instructions, license, "made with Tauri" badge |

## Estimating

- **Track A:** ~30–40 hours.
- **Track B:** ~20–25 hours.
- **Track C:** ~50+ hours.

If you're over budget by 50%, **cut scope, don't extend the deadline**. Ship a smaller v0.1 and add features in v0.2.

## When you're stuck

In order:
1. Read the compiler error fully.
2. Read the Tauri error fully (often "permission not granted: …").
3. Search the [official docs](https://v2.tauri.app/) for the exact API.
4. Search [tauri-apps/tauri](https://github.com/tauri-apps/tauri) issues for your error message.
5. Ask in the [Tauri Discord](https://discord.com/invite/tauri).

## Exit criteria — you are done with Phase 6 when…

- [ ] You have a tagged `v0.1.0` release.
- [ ] An installer exists for at least one OS.
- [ ] You used the installed app for a day without crashes.
- [ ] You wrote a journal entry in `topics/rust/journal.md` with what you'd do differently.
