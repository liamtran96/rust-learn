---
title: 14.5 Miri — Running, Reading, Fixing
tags: [rust, unsafe, miri, ub]
---

# 14.5 Miri — Running, Reading, Fixing

> Authoritative: [`rust-lang/miri`](https://github.com/rust-lang/miri) on GitHub. The README is the up-to-date manual.

Miri is the only practical way to detect UB in your Rust code. It interprets your program's MIR (mid-level IR), tracking allocations, provenance, aliasing (Stacked or Tree Borrows), and uninit memory. **If you're writing unsafe code without running Miri, you're not done.**

## Install

Miri rides on a nightly toolchain:

```bash
rustup toolchain install nightly --component miri
# or, if you already have nightly:
rustup +nightly component add miri
```

Verify:
```bash
cargo +nightly miri --version
```

## Run

The most useful commands:

```bash
# Run all tests under Miri
cargo +nightly miri test

# Run a specific test
cargo +nightly miri test -- my_test_name

# Run the binary
cargo +nightly miri run

# Switch to Tree Borrows (experimental, fewer false positives)
MIRIFLAGS="-Zmiri-tree-borrows" cargo +nightly miri test

# Disable isolation (lets Miri use the real filesystem/clock — needed for some tests)
MIRIFLAGS="-Zmiri-disable-isolation" cargo +nightly miri test
```

Miri is **slow** — expect 10–100× the runtime of native. Run it on small focused tests, not your full integration suite.

## Set it up in CI

Add a job that runs only on changes to crates with `unsafe`:

```yaml
# .github/workflows/miri.yml
name: Miri
on: [push, pull_request]
jobs:
  miri:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@nightly
        with:
          components: miri
      - run: cargo miri test
        env:
          MIRIFLAGS: -Zmiri-strict-provenance
      - run: cargo miri test
        env:
          MIRIFLAGS: -Zmiri-tree-borrows
```

`-Zmiri-strict-provenance` rejects integer-to-pointer casts that lose provenance — turns a class of subtle bugs into hard errors.

## Read Miri's output

A Miri error looks like:

```
error: Undefined Behavior: trying to retag from <wildcard> for SharedReadOnly permission at alloc1[0x0], but that tag does not exist in the borrow stack for this location
   --> src/lib.rs:42:13
    |
 42 |             *p
    |             ^^ trying to retag ...
    |
    = help: this indicates a potential bug in the program: it performed an invalid operation, but the rules it violated are still experimental
```

How to read it:

| Phrase | Meaning |
|---|---|
| `trying to retag` | You're using a reference/pointer; Miri is checking the borrow stack. |
| `<wildcard>` | The pointer's provenance came from int-to-ptr; weak. |
| `does not exist in the borrow stack` | The borrow you're using has been popped (a sibling/parent invalidated it). |
| `disabled` | Same — different phrasing in newer Miri. |
| `dangling reference` | Allocation freed before this access. |
| `using uninitialized data` | Read of `MaybeUninit` slot you didn't write. |
| `data race` | Two threads accessed same memory without sync. |
| `unaligned access` | `*ptr` where `ptr` isn't aligned for `T`. |

The fix is almost always: change which pointer you derive from, narrow the lifetime of a reference, or replace `&T`/`&mut T` with `*const T`/`*mut T` derived from the source allocation directly.

## What Miri detects (and doesn't)

**Detects:**
- Stacked Borrows / Tree Borrows violations
- Use-after-free, double-free, invalid free
- Uninitialized reads
- Misaligned access
- Data races (under `-Zmiri-strict-provenance`)
- Validity violations for primitives (`bool`, `char`, references, enum tags)
- Some ABI mismatches in calls
- Memory leaks (with `-Zmiri-leak-check`, default in test)

**Doesn't detect:**
- UB in code Miri can't reach (no test exercises it).
- Some UB that depends on platform specifics (raw syscalls, intrinsics not modeled).
- All possible execution interleavings (Miri picks one path; concurrency is best-effort).

> **Miri finding UB → your code is unsound.**
> **Miri finding nothing → your code might still be unsound** (path coverage gap or unmodeled UB).

## Tactics for getting Miri-clean

1. **Write tiny tests that exercise the unsafe path directly.** Don't rely on integration tests — they may not trigger the bad sequence.
2. **Cover happy path AND boundary conditions** (empty input, max capacity, ZSTs, panic-on-realloc).
3. **Run with both `-Zmiri-strict-provenance` and the default.** Different lenses.
4. **Run with both Stacked and Tree Borrows.** Sound under both = future-proof.
5. **When Miri yells, do not silence with `#[cfg(not(miri))]`.** That's lying. Fix the code.
6. **For data-race detection**, run tests that actually spawn threads, use `loom` for exhaustive interleaving on lock-free code.

## Companion tools

| Tool | Purpose |
|---|---|
| [`loom`](https://docs.rs/loom) | Exhaustive interleaving for lock-free code. Use *with* Miri, not instead. |
| [`cargo-careful`](https://github.com/RalfJung/cargo-careful) | Run tests on a debug-instrumented stdlib. Catches some UB Miri can't reach. |
| [AddressSanitizer / ThreadSanitizer](https://doc.rust-lang.org/unstable-book/compiler-flags/sanitizer.html) | LLVM-level UB detectors; faster than Miri, less precise. |
| [`kani`](https://model-checking.github.io/kani/) | Bounded model checker — proves properties for all inputs up to a bound. |

For most projects: **Miri is enough**. Add the others when you have specific concerns (concurrency: loom; perf-critical syscalls: ASan/TSan).

## Exit criteria
- [ ] Miri is installed and you've run it on at least one of your unsafe-using projects.
- [ ] You've intentionally introduced a UB bug (e.g. `&mut` aliasing) and watched Miri catch it.
- [ ] You have a CI job that runs Miri on PRs touching unsafe code.
- [ ] You can read a Miri error and locate the exact line that violated aliasing.
