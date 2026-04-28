---
title: 13.5 Managed State
tags: [rust, tauri, state, mutex]
---

# 13.5 Managed State — `tauri::State<T>` + `Mutex`

> Official: [State Management guide](https://v2.tauri.app/develop/state-management/).

Apps need long-lived data — current user, open document, in-memory cache, DB pool. Tauri provides **managed state**: store a value once, inject it into commands as `tauri::State<T>`.

## Register state at startup

```rust
use std::sync::Mutex;
use tauri::Manager;

#[derive(Default)]
struct AppState {
    counter: u32,
    notes: Vec<Note>,
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![increment, get_count])
        .run(tauri::generate_context!())
        .unwrap();
}
```

## Access from a command

```rust
use tauri::State;

#[tauri::command]
fn increment(state: State<'_, Mutex<AppState>>) -> u32 {
    let mut s = state.lock().unwrap();
    s.counter += 1;
    s.counter
}

#[tauri::command]
fn get_count(state: State<'_, Mutex<AppState>>) -> u32 {
    state.lock().unwrap().counter
}
```

That's the whole pattern: `State<'_, Mutex<T>>` → `.lock()` → mutate → drop guard.

## Why `Mutex`?

Tauri's IPC handlers are called from a thread pool. Without a lock, two simultaneous commands could race the same data. Rust won't let you skip this — `&mut` aliasing is a compile error. So we wrap in `Mutex` (or `RwLock` for read-heavy access).

You don't need an explicit `Arc` — `app.manage` clones an internal `Arc` under the hood.

## Sync vs async — pick the right Mutex

| Command kind | Mutex |
|---|---|
| `fn` (sync) | `std::sync::Mutex` |
| `async fn` that holds the guard across `.await` | `tokio::sync::Mutex` |
| `async fn` that locks → reads → drops before any `.await` | `std::sync::Mutex` is fine |

Holding a `std::sync::MutexGuard` across `.await` is a bug — at best it deadlocks, at worst the compiler refuses (`!Send`).

## Larger state — split it up

One giant `Mutex<AppState>` becomes a contention point. Decompose:

```rust
struct AppState {
    settings: Mutex<Settings>,
    notes: Mutex<HashMap<NoteId, Note>>,
    db: SqlitePool, // already thread-safe internally
}

app.manage(AppState { /* … */ });
```

Then commands lock only what they need:
```rust
fn rename_note(state: State<AppState>, id: NoteId, title: String) { /* … */ }
```

## Persisting state across restarts

Two common patterns:

1. **`tauri-plugin-store`** — a key-value JSON store managed for you. Best for settings.
2. **Roll your own** — serialize to a file in `app.path().app_data_dir()` on every change (or debounce).

```rust
use tauri::Manager;

let dir = app.path().app_data_dir()?;
std::fs::create_dir_all(&dir)?;
let path = dir.join("state.json");
std::fs::write(&path, serde_json::to_string_pretty(&snapshot)?)?;
```

For anything bigger than ~MB, use SQLite via `tauri-plugin-sql` or `sqlx`.

## Pitfall: poisoned mutexes

If a command panics while holding a lock, the `Mutex` is **poisoned** — every subsequent `.lock()` returns `Err`. Either:
- Don't panic in commands (return `Result<_, AppError>`), or
- Use `parking_lot::Mutex` (no poisoning, slightly faster, no `.unwrap()` needed).

## Exit criteria
- [ ] You can register a `Mutex<AppState>` at startup and mutate it from a command.
- [ ] You know which `Mutex` to use for sync vs async commands.
- [ ] You've persisted state to disk and restored it on next launch.
- [ ] You have a strategy for splitting a large state into independent locks.
