---
title: 13.6 Plugins & Native APIs
tags: [rust, tauri, plugins, capabilities]
---

# 13.6 Plugins & Native APIs

> Official: [Plugin index](https://v2.tauri.app/plugin/) · [Develop plugins](https://v2.tauri.app/develop/plugins/) · [Capabilities & permissions](https://v2.tauri.app/security/capabilities/).

Tauri's core is small on purpose. Native features come from **plugins** — Cargo crates + matching NPM packages that you opt into per project.

## The official plugins you'll actually use

| Plugin | What it gives you |
|---|---|
| [`tauri-plugin-fs`](https://v2.tauri.app/plugin/file-system/) | Read/write files inside scoped paths |
| [`tauri-plugin-dialog`](https://v2.tauri.app/plugin/dialog/) | Native open/save/message/confirm dialogs |
| [`tauri-plugin-store`](https://v2.tauri.app/plugin/store/) | Persisted JSON KV store for settings |
| [`tauri-plugin-notification`](https://v2.tauri.app/plugin/notification/) | OS notifications (with permission prompt) |
| [`tauri-plugin-shell`](https://v2.tauri.app/plugin/shell/) | Spawn processes, open URLs |
| [`tauri-plugin-sql`](https://v2.tauri.app/plugin/sql/) | SQLite/MySQL/Postgres pool |
| [`tauri-plugin-os`](https://v2.tauri.app/plugin/os/) | Platform info, hostname, locale |
| [`tauri-plugin-clipboard-manager`](https://v2.tauri.app/plugin/clipboard/) | Copy/paste text and images |
| [`tauri-plugin-updater`](https://v2.tauri.app/plugin/updater/) | Self-updating apps |

Always check the official catalog for the current list — plugins are added often.

## Adding a plugin (3 steps)

Example: filesystem.

```bash
# 1. From the project root
pnpm tauri add fs
```

That command does all three things below. If you prefer manual:

```toml
# 2. src-tauri/Cargo.toml
[dependencies]
tauri-plugin-fs = "2"
```

```rust
// 3. src-tauri/src/lib.rs
tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
    // ...
```

Then on the frontend:
```bash
pnpm add @tauri-apps/plugin-fs
```

## Granting permissions

Plugins ship with **predefined permissions**. You opt in via `capabilities/default.json`:

```json
{
  "identifier": "default",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "fs:allow-read-text-file",
    "fs:allow-write-text-file",
    {
      "identifier": "fs:scope",
      "allow": [{ "path": "$APPDATA/notes/*" }]
    },
    "dialog:default",
    "notification:default"
  ]
}
```

Two layers:
1. **Permission identifiers** — `<plugin>:<rule>`, e.g. `fs:allow-read-text-file`.
2. **Scopes** — narrow *what* you can act on (only files under `$APPDATA/notes/`, only `https://api.example.com`, etc.).

Read the plugin's docs for its full permission list. Errors at runtime are explicit when something isn't allowed — read them.

## Using a plugin from JS

```ts
import { readTextFile, writeTextFile, BaseDirectory } from "@tauri-apps/plugin-fs";

await writeTextFile("notes/hello.md", "# hi", { baseDir: BaseDirectory.AppData });
const txt = await readTextFile("notes/hello.md", { baseDir: BaseDirectory.AppData });
```

Prefer `BaseDirectory` constants over hardcoded paths — they resolve to the right OS-specific location.

## Using a plugin from Rust

```rust
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
async fn pick_folder(app: tauri::AppHandle) -> Option<String> {
    let folder = app.dialog().file().blocking_pick_folder();
    folder.map(|p| p.to_string())
}
```

The `DialogExt` trait extension pattern is common: `use <plugin>::<Ext>` to get the methods on `AppHandle`.

## Window, menu, tray (built into core)

Not plugins — these come with Tauri itself.

- **Multiple windows** — declare in `tauri.conf.json` or open at runtime with `WebviewWindowBuilder`.
- **Menu** — native menubar (macOS top bar, Windows/Linux app menu) via `tauri::menu::Menu`.
- **Tray icon** — `tauri::tray::TrayIconBuilder` with click handlers and context menu.

Each is a few lines of Rust; start with the official examples in the [develop guides](https://v2.tauri.app/develop/).

## When to write a custom plugin

Most apps never need to. Write one when:
- You're packaging shared functionality across multiple Tauri apps.
- You need to expose a Rust-only ecosystem crate to the frontend with a clean API.
- You need OS-specific code (a Swift/Kotlin binding for mobile).

Otherwise: just put commands in `src-tauri/src/commands/`.

## Exit criteria
- [ ] You can `pnpm tauri add <plugin>` and use it from both Rust and TS.
- [ ] You can read and write a config file under `$APPDATA` with proper scope.
- [ ] You can show a native open-file dialog and a desktop notification.
- [ ] You understand which permissions and scopes the app actually needs (principle of least privilege).
