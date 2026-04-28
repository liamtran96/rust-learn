---
title: 13.4 Events — Pushing from Rust to JS
tags: [rust, tauri, events, ipc]
---

# 13.4 Events — Pushing from Rust to JS

Events are the **pub/sub** half of IPC. Use them when:
- A long task needs to stream progress to the UI.
- A background watcher (file change, timer, subscription) needs to wake the frontend.
- Multiple windows need to react to the same change.

> Official: [Event System guide](https://v2.tauri.app/develop/calling-frontend/) and [Calling Rust from the Frontend](https://v2.tauri.app/develop/calling-rust/) — read them.

## Emit from Rust

```rust
use tauri::{AppHandle, Emitter};

#[tauri::command]
async fn start_download(app: AppHandle, url: String) -> Result<(), String> {
    for pct in (0..=100).step_by(10) {
        app.emit("download-progress", pct).map_err(|e| e.to_string())?;
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }
    app.emit("download-done", &url).map_err(|e| e.to_string())?;
    Ok(())
}
```

Payloads must be `Serialize`. Strings, numbers, structs — anything serde-friendly.

## Listen on the frontend

```ts
import { listen } from "@tauri-apps/api/event";

const unlisten = await listen<number>("download-progress", (event) => {
  console.log("got", event.payload);
});

// later, when the component unmounts:
unlisten();
```

**Always store the unlisten handle** and call it on unmount. Otherwise listeners stack and you'll get duplicate handlers.

## Emit from JS, listen in Rust

The reverse direction works too — useful if some plugin event needs to flow back to the core.

```ts
import { emit } from "@tauri-apps/api/event";
emit("user-clicked-thing", { id: 42 });
```

```rust
use tauri::Listener;

app.listen("user-clicked-thing", |event| {
    println!("payload: {}", event.payload());
});
```

## Targeting specific windows

Multi-window apps: emit to just one window with `emit_to`.

```rust
app.emit_to("settings", "theme-changed", "dark")?;
```

The first argument is the **label** from `tauri.conf.json` (`"main"`, `"settings"`, etc.).

## Commands vs events — a decision table

| You need to… | Use |
|---|---|
| Get a value back | **Command** |
| Report failure with a typed error | **Command** (returns `Result`) |
| Stream progress | **Event** (or a Tauri 2 `Channel<T>`) |
| Notify *all* windows of a state change | **Event** |
| Trigger fire-and-forget side-effects from JS | **Event** or command-without-await |

## Channels — the modern alternative for streaming

Tauri 2 added `Channel<T>` for typed, ordered, per-call streaming. Cleaner than naked events when you want stream → caller (not stream → world).

```rust
use tauri::ipc::Channel;

#[tauri::command]
async fn stream_logs(on_event: Channel<String>) -> Result<(), String> {
    for line in 0..1000 {
        on_event.send(format!("line {line}")).map_err(|e| e.to_string())?;
    }
    Ok(())
}
```

```ts
import { Channel, invoke } from "@tauri-apps/api/core";

const ch = new Channel<string>();
ch.onmessage = (msg) => console.log(msg);
await invoke("stream_logs", { onEvent: ch });
```

Prefer `Channel<T>` over events when **only the caller** cares about the stream.

## Exit criteria
- [ ] You can emit progress from a long-running command and render a progress bar.
- [ ] You always store and call the `unlisten` handle.
- [ ] You can articulate when to use a command, an event, or a `Channel<T>`.
