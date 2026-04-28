---
title: 13. Tauri — Ship a Desktop App
tags: [rust, tauri, capstone]
---

# 13. Tauri — Ship a Desktop App

This chapter is the payoff. You've spent eight weeks fighting the borrow checker; now you turn that into something you can double-click.

[Tauri 2](https://v2.tauri.app/) builds tiny, fast desktop (and mobile) apps with a Rust backend and a webview frontend. Compared to Electron: ~10× smaller binaries, fraction of the RAM, native OS integration, and Rust where the speed actually matters.

## Contents
- [[setup|13.1 Setup & first project]]
- [[architecture|13.2 Architecture — two processes, one bridge]]
- [[commands|13.3 Commands — calling Rust from JS]]
- [[events|13.4 Events — pushing from Rust to JS]]
- [[state|13.5 Managed state — `tauri::State<T>` + `Mutex`]]
- [[plugins|13.6 Plugins & native APIs]]
- [[packaging|13.7 Packaging & distribution]]
- [[capstone|13.8 Capstone project — pick a track]]

## Prerequisites — what you must already know

Tauri sits on top of real Rust. Don't start until these click:

| You'll need | From chapter |
|---|---|
| `Result<T, E>`, `?`, `thiserror` | [[../05-error-handling/index\|Ch 5]] |
| Modules & cargo workspaces | [[../06-modules/index\|Ch 6]] |
| `Arc<Mutex<T>>`, `Send`, `Sync` | [[../10-concurrency/index\|Ch 10]] |
| `async`/`await`, tokio basics | [[../11-async/index\|Ch 11]] |
| `serde::{Serialize, Deserialize}` | introduced here |

Some frontend literacy helps too — enough HTML/CSS/JS to wire a button to a function. **You do not need to be a frontend expert.** Start with vanilla TS; reach for React/Svelte only when the UI demands it.

## Mental model in one paragraph

Your app is **two processes**: a Rust **core** (`src-tauri/`) and a **webview** rendering your frontend. They talk over a typed IPC bridge. You expose Rust functions as **commands** (request/response) and push updates with **events** (pub/sub). Long-lived data lives in **managed state** behind a `Mutex`. Everything else — filesystem, dialogs, notifications, system tray — comes from **plugins** that you allow-list in `capabilities/`.

## The bet

Most desktop frameworks make easy things easy and hard things impossible. Tauri makes easy things easy *and* gives you an entire systems language for the hard things. The price is that you have to actually learn the systems language. You did. Now collect.

## Official resources (always cite the source)

Treat these as the **single source of truth**. This chapter summarizes and sequences them — when in doubt, the official docs win.

- 📘 [Tauri 2 official docs](https://v2.tauri.app/) — start here, bookmark it
- 🚀 [Start / Prerequisites](https://v2.tauri.app/start/prerequisites/)
- 🔧 [Develop guide](https://v2.tauri.app/develop/) — commands, state, events, plugins
- 🔌 [Plugins index](https://v2.tauri.app/plugin/) — official plugin catalog
- 🛡️ [Security & capabilities](https://v2.tauri.app/security/) — the v2 permission model
- 📦 [Distribute](https://v2.tauri.app/distribute/) — bundling, code signing, app stores
- 💬 [Tauri Discord](https://discord.com/invite/tauri) — fastest place to get unstuck
- 📚 [The Rust Programming Language (official book)](https://doc.rust-lang.org/book/) — the prerequisites for this chapter

## Exit criteria
- [ ] A new Tauri app builds and runs in dev mode on your machine.
- [ ] You can wire a frontend button to a Rust command that returns a typed `Result`.
- [ ] You understand when to use a command vs an event.
- [ ] You can persist state to disk and restore it on next launch.
- [ ] You ship the [[capstone|capstone]] as a signed installer.
