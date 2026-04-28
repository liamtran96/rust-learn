---
title: 14.8 The Unsafe-Code Review Checklist
tags: [rust, unsafe, code-review, soundness]
---

# 14.8 The Unsafe-Code Review Checklist

A practical artifact: paste the relevant block of this checklist into a PR description (or a `// SAFETY:` comment) when you write or review unsafe code. The discipline matters more than the exact wording.

> Inspired by the [Rustonomicon](https://doc.rust-lang.org/nomicon/) and the [`std` library's review practice](https://doc.rust-lang.org/std/) — every `unsafe` block in std has an explanatory comment.

## The `// SAFETY:` comment template

Every `unsafe` block (and every `unsafe fn`) deserves a comment. Structure it as:

```rust
// SAFETY:
// 1. Precondition: <what must be true at this point>.
//    Established by: <what code or invariant guarantees it>.
// 2. The operation does not violate Rust's aliasing rules because
//    <reason>.
// 3. Drop / panic safety: <how partial state is handled>.
unsafe { ptr.write(value) }
```

For an `unsafe fn`, the comment goes on the **function** and lists the obligations the caller must uphold:

```rust
/// Returns a reference to element `i` without bounds checking.
///
/// # Safety
///
/// Caller must ensure `i < self.len()`.
pub unsafe fn get_unchecked(&self, i: usize) -> &T {
    // SAFETY: caller upheld i < self.len() by precondition above.
    unsafe { &*self.ptr.as_ptr().add(i) }
}
```

A good rule: **if you can't write the comment, the unsafe block is wrong**.

## The author checklist (before opening the PR)

For each `unsafe` block:

- [ ] **Smallest scope.** Everything that *can* be safe is outside the block.
- [ ] **Documented.** A `// SAFETY:` comment names each precondition and where it's established.
- [ ] **Validity.** No invalid value of any type ever exists, even momentarily (no uninit `bool`, no null `&T`, etc.).
- [ ] **Aliasing.** No two `&mut` overlap. No `&T` and `&mut T` overlap. Pointers don't outlive their borrow chain.
- [ ] **Initialization.** Every read of memory I claim is `T` was written as a valid `T` first.
- [ ] **Alignment.** Every dereference is at an aligned address. (If from external source, used `read_unaligned`/`write_unaligned`.)
- [ ] **Provenance.** Pointer was derived from the allocation it accesses, or `with_exposed_provenance` is justified.
- [ ] **Drop safety.** If a panic could occur mid-operation, partially-initialized state is handled by a guard or by ordering writes last.
- [ ] **Send/Sync.** Any `unsafe impl Send`/`Sync` is justified by a comment that references the type's invariants.
- [ ] **FFI.** No panic crosses a non-`-unwind` extern boundary. Allocations freed by their matching allocator. ABI types match the C declaration.
- [ ] **Tests.** A test exercises the unsafe path *directly* (not just through a happy-path integration test).
- [ ] **Miri.** `cargo +nightly miri test` is clean. Re-run with `-Zmiri-tree-borrows` and `-Zmiri-strict-provenance`.
- [ ] **Public API.** The wrapping safe API cannot be misused to violate the unsafe block's invariants.

## The reviewer checklist

For each `unsafe` block in the diff:

- [ ] Is there a `// SAFETY:` comment? If not, **block on it**.
- [ ] Does the comment name *all* the preconditions, or only the obvious one?
- [ ] Can I think of a sequence of safe-API calls that would violate any of the preconditions? Try hard.
- [ ] Is the unsafe block as small as it could be? Could lines be moved out?
- [ ] Does the type's documentation explain its invariants?
- [ ] Does CI run Miri on this code path?
- [ ] If it's an `unsafe fn`, are the requirements on the caller listed under `# Safety`?
- [ ] If it's an `unsafe impl`, is there a comment justifying it against the type's docs (e.g., `Send`)?

When in doubt, ask the author to articulate soundness in the PR description.

## Red flags in `unsafe` PRs

| Red flag | Why it's bad |
|---|---|
| `unsafe { transmute(...) }` | Almost always either wrong or hides a worse bug. |
| `unsafe impl Sync` with no comment | The author didn't think hard enough. |
| Casting `&T → *mut T → mutate` | UB unless `T: UnsafeCell<...>`. |
| `static mut FOO: T` | Use `Mutex` / `OnceCell` / atomics. |
| `unsafe fn` without `# Safety` doc | Caller has no way to know what to uphold. |
| `unsafe { ... 50 lines ... }` | Too big. Narrow it. |
| `unwrap()` inside an unsafe block | Panic + unsafe = drop-guard hazard. |
| `#[cfg(not(miri))]` over the unsafe path | Lying to the only tool that catches you. |

## Worked example: a code review you'd accept

```rust
/// A bump arena's allocator.
///
/// # Safety invariants
/// - `next` always points within `[start, end]`.
/// - Memory at `[start, next)` is currently allocated to issued references.
/// - Memory at `[next, end]` is unallocated and writable.
pub struct Arena { /* ... */ }

impl Arena {
    pub fn alloc<T>(&self, value: T) -> &mut T {
        let layout = Layout::new::<T>();
        let next = self.next.get();
        // SAFETY:
        // - `align_offset` is defined for any pointer.
        // - `next.add(off)` stays within the same allocation because we
        //   assert `aligned + size <= self.end` below before using it.
        let off = next.align_offset(layout.align());
        let aligned = unsafe { next.add(off) };

        let new_next = unsafe { aligned.add(layout.size()) };
        assert!(new_next <= self.end, "arena out of memory");
        self.next.set(new_next);

        // SAFETY:
        // - `aligned` is in our allocation, aligned for T, with `size_of::<T>()` bytes available
        //   (checked by the assert above).
        // - We have not handed out any reference to this region before.
        // - We `write` before forming a reference, so the slot holds a valid T.
        // - The returned reference's lifetime is tied to &self, and we never reallocate or
        //   move the buffer — the reference remains valid.
        unsafe {
            let p = aligned as *mut T;
            p.write(value);
            &mut *p
        }
    }
}
```

A reviewer reads this and can verify each bullet against the code. **That's the bar.**

## The escape hatch you should reach for first: don't write unsafe

Order of preference, when faced with a borrow-checker problem:

1. Restructure to use indices instead of references (`Vec<usize>` of "node ids").
2. Use `Cell` / `RefCell` / `Mutex` / `OnceLock` / atomics.
3. Use `Rc<RefCell<T>>` or `Arc<Mutex<T>>`.
4. Use a published unsafe-using crate (`bumpalo`, `slotmap`, `petgraph`, `parking_lot`).
5. Use std's escape-hatch APIs (`split_at_mut`, `chunks_mut`, `swap`, `iter::zip`).
6. *Then* consider writing unsafe yourself.

Most "I need unsafe" turns out to be "I need to step back to step 1."

## Exit criteria
- [ ] You have a personal `// SAFETY:` template you actually use.
- [ ] You can review someone else's unsafe PR using this checklist and either approve it with confidence or articulate the soundness gap.
- [ ] You've audited at least one of your own unsafe blocks against this list and either improved or removed it.
- [ ] You can list 3 things to try *before* reaching for unsafe.
