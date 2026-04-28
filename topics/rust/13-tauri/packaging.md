---
title: 13.7 Packaging & Distribution
tags: [rust, tauri, packaging, signing]
---

# 13.7 Packaging & Distribution

> Official: [Distribute guide](https://v2.tauri.app/distribute/) — covers every target. Read the section for **your** OS.

## What `pnpm tauri build` produces

Per platform, by default:

| Host | Artifact(s) |
|---|---|
| macOS | `.app` bundle, `.dmg` installer (in `src-tauri/target/release/bundle/macos` and `/dmg`) |
| Windows | `.msi` (WiX) and/or `.exe` (NSIS) under `target/release/bundle/{msi,nsis}` |
| Linux | `.deb`, `.rpm`, `.AppImage` under `target/release/bundle/{deb,rpm,appimage}` |

Cross-compiling between OSes is painful — **build each platform on its native host or CI runner**.

## Identity: get this right before you ship anything

`src-tauri/tauri.conf.json`:
```json
{
  "productName": "Notes",
  "version": "0.1.0",
  "identifier": "com.yourdomain.notes",
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "category": "Productivity",
    "shortDescription": "Local-first markdown notes",
    "longDescription": "..."
  }
}
```

- **`identifier`** must be reverse-DNS unique (`com.you.app`). Changing it later means existing installs are treated as a different app.
- **`version`** drives installer versions. Bump on every release.
- **`icon`** — generate from a single 1024×1024 PNG: `pnpm tauri icon path/to/source.png`.

## Optimize the release binary

`src-tauri/Cargo.toml`:
```toml
[profile.release]
opt-level = "s"      # optimize for size; "z" is even smaller but sometimes slower
lto = true           # link-time optimization
codegen-units = 1    # better optimization, slower builds
panic = "abort"      # smaller binary, no unwinding
strip = true         # strip symbols
```

Expect ~3–10 MB binaries. Tauri itself adds little — your dependencies are usually the bulk.

## Code signing (the part that matters for shipping)

| OS | What you need |
|---|---|
| macOS | Apple Developer account ($99/yr), Developer ID certificate, **notarization** via `notarytool`. Without notarization, Gatekeeper blocks the app. |
| Windows | Code-signing certificate from Sectigo/DigiCert/etc. (~$200–400/yr). Without it, SmartScreen warns scary things. |
| Linux | No signing required for `.deb`/`.AppImage`; sign with `gpg` if distributing through repos. |

Tauri integrates signing into the bundle step — set the env vars from the [signing docs](https://v2.tauri.app/distribute/sign/) and `pnpm tauri build` does the rest.

For **personal projects and side projects**, an unsigned build is fine — your users will dismiss one warning. Don't pay for certs until you have users who'd care.

## Auto-updates

The official [`tauri-plugin-updater`](https://v2.tauri.app/plugin/updater/) checks a JSON manifest you host (S3, GitHub Releases, your own server), verifies a signature, downloads, and installs.

Setup once, then every release is a `pnpm tauri build && upload`. Worth doing even for v0.1.

## CI: GitHub Actions in 30 seconds

The official [`tauri-action`](https://github.com/tauri-apps/tauri-action) builds on macOS, Windows, and Linux runners, attaches artifacts to a release, and signs if you provide secrets. Drop their template into `.github/workflows/release.yml` and you're done.

## A pre-flight checklist

Before tagging `v0.1.0`:

- [ ] `productName`, `identifier`, `version` are correct.
- [ ] Icons replaced (no more "Tauri logo" anywhere).
- [ ] `tauri.conf.json` → `bundle.category` set.
- [ ] `[profile.release]` is tuned.
- [ ] `capabilities/` lists only what you actually use.
- [ ] `cargo clippy --release -- -D warnings` is clean.
- [ ] `pnpm tauri build` succeeds on a clean clone.
- [ ] You installed your own bundle and used it for an hour.
- [ ] LICENSE file exists.
- [ ] README has install instructions and a screenshot.

## Exit criteria
- [ ] `pnpm tauri build` produces an installer for your OS.
- [ ] You've installed it from the bundle (not from `tauri dev`) and used it.
- [ ] You can explain what would change to enable code signing.
- [ ] You have (or know how to set up) a release CI workflow.
