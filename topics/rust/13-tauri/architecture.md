---
title: 13.2 Architecture — Two Processes, One Bridge
tags: [rust, tauri, architecture]
---

# 13.2 Architecture — Two Processes, One Bridge

## The two-process model

```
┌─────────────────────┐         ┌──────────────────────┐
│  WebView Process    │  IPC    │   Rust Core Process  │
│  (your frontend)    │ ◄────► │   (src-tauri/)       │
│  HTML/CSS/JS/TS     │         │   commands · state   │
│  no fs, no spawn    │         │   plugins · events   │
└─────────────────────┘         └──────────────────────┘
```

- The **webview** runs your frontend. It has **no direct access** to the filesystem, network, shell, or OS APIs. It can only ask.
- The **Rust core** owns the OS. It exposes a curated surface (commands + events + allow-listed plugins) to the webview.
- Communication is over an IPC channel that serializes payloads as JSON (with `serde`).

This split is the security model. A compromised webview (e.g. via XSS) cannot `rm -rf ~/`. It can only call the commands you defined.

## Two communication patterns

| Pattern | Direction | Shape | Use when |
|---|---|---|---|
| **Command** | JS → Rust → JS | Request / response, can return `Result<T, E>` | "Save this file", "Get me the user list" |
| **Event** | either direction, broadcast | Fire-and-forget, JSON payload | "Download progress: 42%", "User clicked tray icon" |

You'll use commands 80% of the time. Events are for streaming and notifications.

## The capability model (v2)

Tauri 2 replaced v1's allowlist with **capabilities**: per-window declarations of which commands and plugin APIs the frontend may use.

`src-tauri/capabilities/default.json`:
```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Default capabilities for the main window",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "fs:allow-read-text-file",
    "dialog:default"
  ]
}
```

Forget to grant a permission and the call fails at runtime with a clear error. **Read the error.**

## File-by-file tour of `src-tauri/`

| Path | What it's for |
|---|---|
| `src/main.rs` | Trivial entry; usually just `your_app_lib::run()` |
| `src/lib.rs` | The actual app: builder, commands, state, setup hook |
| `Cargo.toml` | Rust deps; pin `tauri = "2"` and add plugins here |
| `tauri.conf.json` | App identifier, window config, bundle (icons, signing), security |
| `capabilities/*.json` | Permission grants per window |
| `icons/` | Pre-generated icon set; replace with `pnpm tauri icon path/to/source.png` |
| `build.rs` | Wires `tauri-build` — don't touch unless you know why |

## Where to put your Rust code

Small app: everything in `lib.rs` is fine.

Real app: split into modules:
```
src-tauri/src/
├── lib.rs          # builder + setup
├── commands/
│   ├── mod.rs
│   ├── notes.rs
│   └── settings.rs
├── state.rs        # AppState struct
├── error.rs        # AppError enum (thiserror)
└── domain/         # pure logic, no Tauri imports
```

The **domain/** convention matters: pure Rust logic with zero Tauri dependencies is testable with plain `cargo test` and reusable if you ever target mobile or build a CLI variant.

## Exit criteria
- [ ] You can draw the two-process diagram from memory.
- [ ] You know which calls require a capability grant and which don't.
- [ ] You can find where commands, state, capabilities, and the bundle config live.
- [ ] You can explain why the webview can't open `/etc/passwd`.
