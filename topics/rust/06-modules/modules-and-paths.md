---
title: 6.1 Modules & Paths
tags: [rust, modules]
---

# 6.1 Modules & Paths

## Declaration — files = modules

```
src/
├── main.rs            # crate root for binary
├── lib.rs             # crate root for library (if any)
├── config.rs          # module `config`
└── net/
    ├── mod.rs         # module `net` (old style, still valid)
    ├── client.rs      # module `net::client`
    └── server.rs      # module `net::server`
```

**Modern (2018+) layout**: use `net.rs` + `net/` sibling directory instead of `net/mod.rs`:

```
src/
├── main.rs
├── net.rs             # declares `mod client;` and `mod server;`
└── net/
    ├── client.rs
    └── server.rs
```

Equivalent; modern is preferred — keeps `mod.rs` files from dominating file lists.

## Connecting modules — `mod`

In `src/lib.rs` or `src/main.rs`:

```rust
mod config;        // loads src/config.rs
mod net;           // loads src/net.rs (which itself declares its submodules)
```

## Visibility

Everything is **private** by default — only accessible within the declaring module (and its descendants).

```rust
pub struct Public;        // exported from the module
struct Internal;          // only within this module & its children

pub(crate) struct CratePublic;    // anywhere in this crate
pub(super) struct ParentPublic;   // parent module can see
pub(in crate::net) struct Scoped; // specific path
```

For structs, **field visibility is separate from type visibility**. A public struct with private fields is a classic pattern for enforcing invariants.

## Paths

```rust
use crate::net::client::Client;     // absolute from crate root
use super::config::Cfg;              // from parent module
use self::helpers::do_thing;         // explicit same-module
```

`crate::` is the current crate's root. `super::` is one level up. `self::` is here.

## Re-exports

A module can promote a child's item to its own namespace:

```rust
// src/lib.rs
pub use crate::net::client::Client;

// consumers can now write:
use my_crate::Client;                // instead of my_crate::net::client::Client
```

This is how `anyhow::Result` is reachable at `anyhow::Result` even though it's defined deeper.

## `use` patterns

```rust
use std::collections::{HashMap, BTreeMap};
use std::io::{self, Write};                // io:: plus io::Write
use std::fmt::Debug as DebugTrait;         // rename
use std::io::*;                            // glob — avoid in lib code
```

## Related
- [[crates-and-workspaces|Crates & workspaces]]
- [[../03-types-and-traits/structs|Struct visibility]]
