---
title: 13.3 Commands — Calling Rust from JS
tags: [rust, tauri, ipc, serde]
---

# 13.3 Commands — Calling Rust from JS

A **command** is a Rust function the frontend can call. It's the request/response half of IPC.

## The smallest possible command

```rust
// src-tauri/src/lib.rs
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

From the frontend:
```ts
import { invoke } from "@tauri-apps/api/core";

const msg = await invoke<string>("greet", { name: "Liam" });
console.log(msg); // "Hello, Liam!"
```

Three rules:
1. Argument names in JS (`{ name }`) must match Rust parameter names (`name: &str`) — camelCase on the JS side maps to snake_case on Rust if you prefer.
2. The function must be registered in `generate_handler!`. Forgetting this is the #1 beginner bug.
3. All argument and return types must be `Serialize`/`Deserialize` (strings, numbers, `Vec`, `HashMap`, your own structs with `#[derive(Serialize, Deserialize)]`).

## Custom payloads with `serde`

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Note {
    id: String,
    title: String,
    body: String,
}

#[tauri::command]
fn save_note(note: Note) -> Note {
    // ... persist somewhere ...
    note
}
```

```ts
const saved = await invoke<Note>("save_note", { note: { id: "1", title: "Hi", body: "..." } });
```

Add to `Cargo.toml`:
```toml
serde = { version = "1", features = ["derive"] }
```

## Returning errors

Don't `panic!` in commands — it crashes the app. Return `Result<T, E>` where `E: Serialize`.

```rust
use thiserror::Error;
use serde::Serialize;

#[derive(Debug, Error, Serialize)]
#[serde(tag = "kind", content = "message")]
pub enum AppError {
    #[error("note not found: {0}")]
    NotFound(String),
    #[error("io error: {0}")]
    Io(String),
}

// thiserror's #[from] won't impl Serialize for std::io::Error, so map manually:
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self { AppError::Io(e.to_string()) }
}

#[tauri::command]
fn read_note(id: String) -> Result<Note, AppError> {
    Err(AppError::NotFound(id))
}
```

On the JS side:
```ts
try {
  const note = await invoke<Note>("read_note", { id: "missing" });
} catch (err) {
  // err is the serialized AppError: { kind: "NotFound", message: "..." }
}
```

## Async commands

If the function is `async`, Tauri runs it on its async runtime — perfect for I/O, HTTP, database.

```rust
#[tauri::command]
async fn fetch_user(id: u32) -> Result<User, AppError> {
    let user = reqwest::get(format!("https://api.example.com/users/{id}"))
        .await
        .map_err(|e| AppError::Io(e.to_string()))?
        .json::<User>()
        .await
        .map_err(|e| AppError::Io(e.to_string()))?;
    Ok(user)
}
```

> **Pitfall:** Don't hold a `std::sync::Mutex` guard across an `.await`. Use `tokio::sync::Mutex` for state accessed from async commands.

## Accessing `AppHandle` and `Window`

Need to emit events, open windows, or call plugins from inside a command? Take them as parameters — Tauri injects them automatically.

```rust
use tauri::{AppHandle, Emitter, Manager};

#[tauri::command]
async fn long_task(app: AppHandle) -> Result<(), AppError> {
    for i in 0..=100 {
        app.emit("progress", i).ok();
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    Ok(())
}
```

## Permission to invoke

Every command you define is implicitly `allow-<command_name>`. Add it to a capability file:

```json
{
  "permissions": [
    "core:default",
    { "identifier": "allow-greet" }
  ]
}
```

For most apps, the auto-generated `core:default` covers your own commands and you only grant explicit permissions to plugin commands.

## Exit criteria
- [ ] You can write a command that takes a struct, returns a struct, and call it from TS with full types.
- [ ] You return `Result<T, E>` where `E: Serialize` and you can `catch` it on the frontend.
- [ ] You know when to use `async fn` vs sync.
- [ ] You can inject `AppHandle` and use it to emit an event.
