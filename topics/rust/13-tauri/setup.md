---
title: 13.1 Setup & First Project
tags: [rust, tauri, setup]
---

# 13.1 Setup & First Project

## Prerequisites by OS

All platforms need:
- **Rust** — `rustup` with stable toolchain (`rustup default stable && rustup update`)
- **Node** — v20+ (use `fnm` or `nvm`); **pnpm** is the recommended package manager

Then OS-specific:

### Linux
```bash
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```
(Adapt for your distro — Fedora/Arch have equivalent packages.)

### macOS
```bash
xcode-select --install
```

### Windows
- Install [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with the "Desktop development with C++" workload.
- WebView2 ships with Windows 10 (1803+) and 11. No extra step.

> **Verify with the doctor:**
> ```bash
> pnpm dlx @tauri-apps/cli@latest info
> ```
> It tells you exactly what's missing.

## Create a project

```bash
pnpm create tauri-app@latest
```

You'll be asked:
- **Project name** — kebab-case, becomes the binary name
- **Frontend language** — pick **TypeScript** (you'll thank yourself)
- **Package manager** — pnpm
- **UI template** — start with **Svelte** or **vanilla**; React is fine if you already know it
- **UI flavor** — TypeScript

Then:
```bash
cd your-app
pnpm install
pnpm tauri dev
```

First build takes a few minutes (Rust crates compile from source). Subsequent runs are fast.

## What just got generated

```
your-app/
├── src/                     # frontend (HTML/CSS/JS/TS)
├── src-tauri/               # Rust backend
│   ├── src/
│   │   ├── main.rs          # entry point — usually one line
│   │   └── lib.rs           # the actual app — commands, state, setup
│   ├── capabilities/
│   │   └── default.json     # what the frontend is allowed to do
│   ├── icons/               # app icons (multiple sizes)
│   ├── tauri.conf.json      # app metadata, build config, bundle settings
│   └── Cargo.toml
├── package.json
└── vite.config.ts           # or similar, depending on template
```

The split is sacred: `src/` is browser-land, `src-tauri/` is Rust-land. They cannot import each other directly — only via the IPC bridge.

## The dev loop

| Command | What it does |
|---|---|
| `pnpm tauri dev` | Hot-reload dev server + Rust rebuild on change |
| `pnpm tauri build` | Production binary + installer for your OS |
| `pnpm tauri build --debug` | Production-ish binary with debug symbols |
| `pnpm tauri info` | Diagnostic dump — paste this in bug reports |

In the running app, **right-click → Inspect Element** opens devtools (only in dev builds by default).

## Exit criteria
- [ ] `pnpm tauri dev` opens a window with the template app.
- [ ] You can edit a string in `src/` and see it hot-reload.
- [ ] You can edit `src-tauri/src/lib.rs`, save, and watch it rebuild.
- [ ] You know which directory is which.
