---
title: Ch 13 — Tauri Exercises
tags: [rust, tauri, exercises]
---

# Ch 13 — Tauri Exercises

A graded ladder from "hello window" to "ship the capstone". Use the [official docs](https://v2.tauri.app/) as your primary reference for every exercise.

## Setup gate (do these first)

1. **Install + verify** — Run `pnpm dlx @tauri-apps/cli@latest info` and screenshot the output. Every line should be green/satisfied.
2. **Hello, Tauri** — `pnpm create tauri-app`, pick TS + Svelte (or your choice), run `pnpm tauri dev`. Edit the heading text in the frontend and watch hot-reload work.

## Commands warm-up

3. **`add(a, b) -> i64`** — write a sync command, register it, call from the frontend, log the result. Make sure both args are properly typed end-to-end.
4. **`fizzbuzz(n: u32) -> Vec<String>`** — return the FizzBuzz sequence to the frontend; render as a list. Reuses your Ch 1 logic.
5. **Typed errors** — extend `add` to reject `i64` overflow and return a `Result<i64, AppError>` with a `thiserror`-derived `AppError`. Catch it on the JS side and display the kind + message.
6. **Async I/O** — write `async fn fetch_and_parse(url: String) -> Result<usize, AppError>` that GETs a URL with `reqwest` and returns the byte length. Display in the UI.

## State

7. **Counter app** — `Mutex<u32>` managed state; commands `increment`, `decrement`, `reset`, `get`. Two windows must agree on the count.
8. **Decompose state** — refactor `AppState { settings, items }` so each field is independently locked. Demonstrate that locking `settings` doesn't block a command that only touches `items`.
9. **Async-safe** — convert one of the commands above to `async` and intentionally hold the guard across an `.await`. Observe the compiler error. Fix by switching to `tokio::sync::Mutex` *or* by dropping the guard before the await.

## Events & channels

10. **Progress bar** — command `start_work(seconds: u32)` that emits `progress` events 0..=100 and a final `done` event. Render a `<progress>` bar from the events.
11. **Unlisten correctly** — verify your listener really stops by toggling subscriptions in the UI; show in the Rust logs that no further dispatches happen after unlisten.
12. **`Channel<T>` streaming** — rewrite the progress example using `tauri::ipc::Channel<u8>` instead of global events. Write down (in `journal.md`) when you'd choose one over the other.

## Plugins & native integration

13. **Read-write a config file** — use `tauri-plugin-fs` to read/write `$APPDATA/<your-app>/config.json`. Add a proper `fs:scope` to capabilities. Confirm that paths outside the scope fail loudly.
14. **Native dialog → file** — button opens a save dialog (`tauri-plugin-dialog`), writes "hello" to the chosen path. Handle "user cancelled" without crashing.
15. **Notification** — show a desktop notification on a button press. Handle the permission prompt the first time.
16. **Tray icon** — add a tray icon with a "Quit" menu item. Stretch: a "Toggle window" item that hides/shows the main window.

## Persistence

17. **Settings via `tauri-plugin-store`** — store theme + font size; load on startup; persist on change. Survive app restart.
18. **Roll your own persistence** — replicate exercise 17 using only `tauri-plugin-fs` + `serde_json`. Compare ergonomics.

## Packaging gate

19. **Replace icons** — generate from a 1024×1024 source with `pnpm tauri icon`. Confirm the bundled app shows your icon in the dock/taskbar.
20. **First bundle** — `pnpm tauri build`. Install the bundle (not `tauri dev`). Use it for an hour. Find and fix one bug you couldn't see in dev.
21. **Tune `[profile.release]`** — apply the size-optimized profile from [[../13-tauri/packaging|13.7]]. Record the binary size before and after in your journal.

## Capstone

22. **Pick a track** — A (notes), B (pomodoro tray), or C (expense tracker). Write your decision and the reason in `journal.md`.
23. **Ship M0–M2** — repo, first command, persisted state. Open a draft PR or tag `v0.0.1`.
24. **Ship M3–M5** — native integration, polish, installer. Tag `v0.1.0`.
25. **Retrospective** — write 5 bullet points in `journal.md`: what was harder than expected, what was easier, what you'd reuse, what you'd cut, what's next.

## Checkpoint

You're done with Ch 13 when you have a tagged `v0.1.0` release, you've installed your own bundle, and you can explain — without notes — when to use a command, an event, and a `Channel<T>`.
