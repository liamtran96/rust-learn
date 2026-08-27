---
title: Ownership Journal - Expressions Review
tags: [rust, journal, ownership]
---

# Expressions Review

> Ownership topic journal. Return to [[../../journal|Journal index]].

## Entries

### 2026-08-27 - Tail expressions and unit
**Working on:** Ownership drill d11 - `code/02-ownership/drills-ownership/tests/d11_scoped_return.rs`
**What clicked:** A block returns the value of its final expression when that expression has no trailing semicolon. Adding `;` turns `n + 1` into an expression statement whose value is discarded, leaving the function body to evaluate to unit `()` instead of the declared `i32`.
**What didn't:** The prediction correctly expected unit, but the first completion attempt left the semicolon in place. The initial explanation described a statement only as declaring something; statements also include semicolon-terminated expressions whose produced values are discarded.
**Questions asked this session:** -
**Question to answer later:** Which Rust expressions commonly look like statements but can produce values when used without a semicolon?
**Next:** Complete ownership drill d12: fill `PREDICT:`, run `cargo test --test d12_scanner_peek`, implement it, then fill `WHY:`.

