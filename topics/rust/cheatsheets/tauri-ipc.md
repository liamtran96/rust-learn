---
title: Tauri IPC Cheatsheet
tags: [rust, tauri, ipc, cheatsheet]
---

# Tauri IPC Cheatsheet

Quick reference for the Rust ↔ JS bridge. Authoritative source: [v2.tauri.app/develop](https://v2.tauri.app/develop/).

## Define a command

```rust
#[tauri::command]
fn greet(name: String) -> String { format!("Hello, {name}") }

#[tauri::command]
async fn save(state: tauri::State<'_, Mutex<AppState>>, note: Note)
    -> Result<NoteId, AppError>
{ /* ... */ }
```

Register in the builder:
```rust
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, save])
```

## Call a command

```ts
import { invoke } from "@tauri-apps/api/core";

const msg = await invoke<string>("greet", { name: "Liam" });
try {
  const id = await invoke<string>("save", { note });
} catch (err) { /* err is your serialized AppError */ }
```

## Emit an event (Rust → JS)

```rust
use tauri::Emitter;
app.emit("progress", 42)?;            // all listeners
app.emit_to("settings", "theme", "dark")?; // specific window label
```

```ts
import { listen } from "@tauri-apps/api/event";
const unlisten = await listen<number>("progress", e => console.log(e.payload));
// when done:
unlisten();
```

## Emit an event (JS → Rust)

```ts
import { emit } from "@tauri-apps/api/event";
emit("user-clicked", { id: 42 });
```

```rust
use tauri::Listener;
app.listen("user-clicked", |e| { /* e.payload() is &str JSON */ });
```

## Streaming with `Channel<T>` (preferred for caller-only streams)

```rust
use tauri::ipc::Channel;

#[tauri::command]
async fn tail(on_line: Channel<String>) -> Result<(), String> {
    on_line.send("first".into()).map_err(|e| e.to_string())?;
    Ok(())
}
```

```ts
import { Channel, invoke } from "@tauri-apps/api/core";
const ch = new Channel<string>();
ch.onmessage = (line) => console.log(line);
await invoke("tail", { onLine: ch });
```

## Managed state

```rust
app.manage(Mutex::new(AppState::default()));

#[tauri::command]
fn count(state: tauri::State<'_, Mutex<AppState>>) -> u32 {
    state.lock().unwrap().counter
}
```

| Sync command | `std::sync::Mutex` |
| Async command holding guard across `.await` | `tokio::sync::Mutex` |

## Errors (must be `Serialize`)

```rust
use thiserror::Error;
use serde::Serialize;

#[derive(Debug, Error, Serialize)]
#[serde(tag = "kind", content = "message")]
pub enum AppError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("io: {0}")]
    Io(String),
}
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self { AppError::Io(e.to_string()) }
}
```

## When to pick what

| Scenario | Tool |
|---|---|
| One question → one answer | command |
| Long task → progress to caller | `Channel<T>` |
| Long task → notify many windows | event |
| Background watcher → UI update | event |
| Mutate shared data | command + `State<Mutex<…>>` |
| Open a file dialog | `tauri-plugin-dialog` (from a command) |

## Common pitfalls

- **Forgot `generate_handler!`** — runtime "command not found".
- **Argument name mismatch** — JS `{ noteId }` vs Rust `note_id` is fine; `{ noteid }` is not.
- **Returning a non-`Serialize` type** — won't compile; check error location.
- **Holding `std::sync::MutexGuard` across `.await`** — `!Send`, won't compile (good, it would deadlock).
- **No `unlisten()`** — listener leaks pile up across re-renders.
- **Missing capability/permission** — runtime error names exactly which permission you need.
- **Panicking in a command** — poisons the mutex; return `Result<_, AppError>` instead.
