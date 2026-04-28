---
title: 14.6 Common Patterns — Arena, Linked List, Vec
tags: [rust, unsafe, patterns, datastructures]
---

# 14.6 Common Patterns — Arena, Linked List, Vec

The patterns most often used in unsafe Rust. Each is a worked example you can adapt. **Run them under Miri before trusting them.**

> Best companion: [Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/) — the canonical hands-on tutorial.

## Pattern 1 — Bump arena allocator

A region allocator: bump a pointer for each allocation, free everything at once on drop. No individual frees → much simpler than a real allocator.

```rust
use std::alloc::{alloc, dealloc, Layout};
use std::cell::Cell;
use std::ptr::NonNull;

pub struct Arena {
    /// Start of the block.
    start: NonNull<u8>,
    /// One-past-end.
    end: *mut u8,
    /// Next free byte.
    next: Cell<*mut u8>,
    /// Layout used to allocate (so we can free).
    layout: Layout,
}

impl Arena {
    pub fn with_capacity(bytes: usize) -> Self {
        let layout = Layout::from_size_align(bytes, 16).unwrap();
        // SAFETY: layout has nonzero size; alloc may return null on OOM.
        let raw = unsafe { alloc(layout) };
        let start = NonNull::new(raw).expect("OOM");
        let end = unsafe { raw.add(bytes) };
        Self { start, end, next: Cell::new(raw), layout }
    }

    pub fn alloc<T>(&self, value: T) -> &mut T {
        let layout = Layout::new::<T>();
        let next = self.next.get();
        // Round up to alignment
        let aligned = unsafe {
            let off = next.align_offset(layout.align());
            next.add(off)
        };
        let new_next = unsafe { aligned.add(layout.size()) };
        assert!(new_next <= self.end, "arena out of memory");
        self.next.set(new_next);

        // SAFETY: aligned points to `layout.size()` bytes of writable memory in
        // our arena, properly aligned for T. We write before returning a reference.
        unsafe {
            let p = aligned as *mut T;
            p.write(value);
            &mut *p
        }
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        // SAFETY: we allocated with `self.layout` in `with_capacity`.
        unsafe { dealloc(self.start.as_ptr(), self.layout); }
    }
}
```

**What's sound here?**
- Single allocation, one `dealloc` matching it.
- `alloc<T>()` returns `&mut T` whose lifetime ties to `&self` — bumps cannot reallocate, so the reference is stable.
- `Cell<*mut u8>` allows interior mutation of `next` through `&Arena`.

**Caveats v1 doesn't handle:**
- **Drop**: arena leaks any non-trivial `Drop`. Real arenas track destructors or restrict to `T: Copy`.
- **Aliasing**: handing out `&mut T` from `&self` is correct here only because we never give out two refs to the same allocation; that requires the bump to be monotonic and `alloc<T>` to take `&self` (not `&mut self`) — but we use `Cell`, not `&mut`, to mutate `next`. Good.
- **Panic safety**: if `value: T`'s constructor panics before `p.write(value)`, no UB (we haven't written yet). After the write, T's destructor doesn't run on arena drop — which is the leak above.

> Real-world arenas: [`bumpalo`](https://docs.rs/bumpalo) (production-grade), [`typed-arena`](https://docs.rs/typed-arena), [`slotmap`](https://docs.rs/slotmap). Read their source for the polished version.

## Pattern 2 — Singly-linked stack (intrusive-ish)

The "too many linked lists" classic. The minimal sound version:

```rust
use std::ptr::NonNull;

pub struct Stack<T> {
    head: Option<NonNull<Node<T>>>,
    len: usize,
}

struct Node<T> {
    value: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Self { Self { head: None, len: 0 } }

    pub fn push(&mut self, value: T) {
        let node = Box::new(Node { value, next: self.head });
        self.head = Some(NonNull::from(Box::leak(node)));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head?;
        // SAFETY: head was produced by Box::leak; we have exclusive access via &mut self.
        let node = unsafe { Box::from_raw(head.as_ptr()) };
        self.head = node.next;
        self.len -= 1;
        Some(node.value)
    }

    pub fn len(&self) -> usize { self.len }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}
```

**Soundness sketch**:
- Every `NonNull<Node<T>>` came from `Box::leak`. We re-`Box::from_raw` exactly once on pop.
- `&mut self` ensures no concurrent access.
- Drop empties the stack, so no leaks.

**Sound but sub-optimal**: this calls `Box` per node (per-element allocation). Real high-performance lists use an arena.

**Run it under Miri.** Push 100, pop 100, drop with 50 elements remaining. Should be clean.

## Pattern 3 — `MyVec<T>` skeleton

A great learning exercise. Build, run Miri, fix, repeat. The minimal interface:

```rust
use std::alloc::{alloc, dealloc, realloc, Layout};
use std::marker::PhantomData;
use std::mem;
use std::ptr::{self, NonNull};

pub struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
    _marker: PhantomData<T>,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            cap: if mem::size_of::<T>() == 0 { usize::MAX } else { 0 },
            _marker: PhantomData,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.cap { self.grow(); }
        unsafe {
            // SAFETY: ptr.add(self.len) points to one-past-end; capacity now > len.
            ptr::write(self.ptr.as_ptr().add(self.len), value);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 { return None; }
        self.len -= 1;
        // SAFETY: index < old len, so the slot was initialized.
        Some(unsafe { ptr::read(self.ptr.as_ptr().add(self.len)) })
    }

    pub fn len(&self) -> usize { self.len }

    fn grow(&mut self) {
        // ZST: never grow.
        if mem::size_of::<T>() == 0 { return; }

        let new_cap = if self.cap == 0 { 4 } else { self.cap * 2 };
        let new_layout = Layout::array::<T>(new_cap).expect("layout overflow");

        let new_ptr = if self.cap == 0 {
            unsafe { alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            unsafe { realloc(self.ptr.as_ptr() as *mut u8, old_layout, new_layout.size()) }
        };

        self.ptr = NonNull::new(new_ptr as *mut T).expect("OOM");
        self.cap = new_cap;
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        // Drop initialized elements
        while self.pop().is_some() {}
        // Free allocation
        if self.cap != 0 && mem::size_of::<T>() != 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe { dealloc(self.ptr.as_ptr() as *mut u8, layout); }
        }
    }
}

// SAFETY: MyVec owns its T, so it's Send/Sync iff T is.
unsafe impl<T: Send> Send for MyVec<T> {}
unsafe impl<T: Sync> Sync for MyVec<T> {}
```

**What's missing for a real `Vec`:** `index`, `iter`, `clone`, `extend`, `Deref<Target=[T]>`, panic safety on `grow`, capacity overflow checks, alignment-aware allocator handling. Add them one at a time. **Run Miri after each.**

## Pattern 4 — Splitting a slice mutably

How `<[T]>::split_at_mut` is implemented (simplified):

```rust
pub fn split_mid<T>(s: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    assert!(mid <= s.len());
    let len = s.len();
    let ptr = s.as_mut_ptr();
    unsafe {
        // SAFETY: the two halves do not overlap and are within the original slice.
        // We never observe `s` between the construction of the two slices.
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

The key insight: we hand out two `&mut [T]` whose memory ranges are *disjoint*. The borrow checker can't see disjointness through pointers, but the model is satisfied: at any point, each byte is reachable through exactly one of the two slices.

## Anti-patterns to avoid

- **`unsafe { transmute }` to "force" a cast.** Almost always wrong. Use `try_into`, `as`, or fix the design.
- **Holding `*mut T` and `&mut T` to the same data simultaneously.** UB the moment you use both.
- **Casting `&T` → `*mut T` → mutate.** UB. Use `UnsafeCell`.
- **`unsafe impl Sync for X {}` because it compiles.** State the soundness reason in a `// SAFETY:` or `// SOUND:` comment.

## Exit criteria
- [ ] You've implemented the `MyVec<T>` above and it's Miri-clean for at least: push 1000, pop 500, drop with elements remaining, both `T = u32` and `T = String`.
- [ ] You can explain the soundness sketch for `split_mid` aloud.
- [ ] You know which of these you'd never write in real production code (the answer: all of them, use `Vec`/`bumpalo`/`std`).
- [ ] You have read at least the first 3 chapters of [Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/).
