---
title: 6. Modules, Crates, Workspaces
tags: [rust, modules, crates]
---

# 6. Modules, Crates, Workspaces

How Rust organizes code.

## Contents
- [[modules-and-paths|6.1 Modules & paths]]
- [[crates-and-workspaces|6.2 Crates & workspaces]]

## Key ideas
- **Crate** — the compilation unit. Each binary/library you publish is one crate.
- **Module** — a namespace *within* a crate. Files become modules implicitly; `mod` statements declare them.
- **Workspace** — a group of crates sharing a lockfile, for multi-crate projects.

## Exit criteria
- [ ] You can split a single-file program into a multi-module crate.
- [ ] You understand `pub`, `pub(crate)`, `pub(super)`, private.
- [ ] You can set up a workspace with a binary + shared library crate.
