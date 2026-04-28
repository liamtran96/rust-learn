---
title: 14.4 MaybeUninit & UnsafeCell
tags: [rust, unsafe, maybeuninit, unsafecell]
---

# 14.4 `MaybeUninit<T>` and `UnsafeCell<T>`

> Authoritative: [`std::mem::MaybeUninit`](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html) · [`std::cell::UnsafeCell`](https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html).

Two primitives that look obscure until you need them, then they become inevitable. Both exist to model things the type system can't otherwise express.

## `MaybeUninit<T>` — uninitialized memory, legally

### The problem

You want to allocate memory for a `T` *without* having a `T` to put in it yet. Maybe you'll write to it later, maybe you'll read sensors into it, maybe you're building a `Vec<T>` and only the first `len` slots are initialized.

You **cannot** just have an "uninit `T`":

```rust
let x: u32 = unsafe { std::mem::uninitialized() }; // DEPRECATED — instant UB
```

Why? Because *uninitialized memory is not a valid `u32`*. The mere existence of an uninit `u32` violates `u32`'s validity invariant. (For some types like `bool`, that means UB. For all types, the optimizer is allowed to assume it never happens.)

### The solution

```rust
use std::mem::MaybeUninit;

let mut slot: MaybeUninit<u32> = MaybeUninit::uninit();
slot.write(42);                          // initialize
let value: u32 = unsafe { slot.assume_init() }; // promise: I wrote it
assert_eq!(value, 42);
```

`MaybeUninit<T>` is a wrapper that **says nothing about the validity of the memory inside**. It's safe to construct and hold. Only `assume_init` requires `unsafe` — that's the moment you swear "yes, this is initialized as a valid `T`."

### Common patterns

**Array of uninit, init in place:**
```rust
let mut data: [MaybeUninit<u32>; 1024] = unsafe { MaybeUninit::uninit().assume_init() };
//                                       ^^^^^ legal: an array of MaybeUninit is itself initialized
//                                             (the inner cells aren't, but that's the point)
for slot in &mut data {
    slot.write(0);
}
let initialized: [u32; 1024] = unsafe {
    // SAFETY: every element written above
    std::mem::transmute(data)
};
```

Newer std API: `[MaybeUninit::<u32>::uninit(); 1024]` and `MaybeUninit::array_assume_init`.

**Inside a `Vec<T>`'s buffer:**

The `Vec` you call `push` on has capacity for, say, 16 `T`s but maybe only 3 initialized slots. The first 3 are valid `T`s; slots 3..16 are `MaybeUninit<T>`. `Vec` keeps track via `len`.

**Returning from FFI:**

```rust
extern "C" {
    fn fill_buffer(out: *mut u8, len: usize) -> i32;
}

let mut buf: [MaybeUninit<u8>; 4096] = unsafe { MaybeUninit::uninit().assume_init() };
let written = unsafe { fill_buffer(buf.as_mut_ptr() as *mut u8, buf.len()) };

let initialized = unsafe {
    // SAFETY: C wrote `written` bytes
    std::slice::from_raw_parts(buf.as_ptr() as *const u8, written as usize)
};
```

### Pitfalls

- **`assume_init` requires the value to be valid.** Calling it on uninitialized memory is UB.
- **Don't `mem::replace` an uninit with a valid `T` and pretend the old one was valid.** Use `assume_init` only when you know.
- **Drop is not run for `MaybeUninit<T>`.** If you initialized it and want to run `Drop`, call `assume_init_drop()` or read it out and let it drop naturally.
- **Forgetting partial initialization on panic.** If you initialize half an array and a constructor panics, the half-initialized prefix needs `Drop` — implement that yourself with a guard struct (see "drop guard" pattern in the [Nomicon](https://doc.rust-lang.org/nomicon/leaking.html)).

## `UnsafeCell<T>` — the only legal interior mutability

### The problem

Every safe abstraction that mutates through a shared reference (`&T`) — `Cell`, `RefCell`, `Mutex`, `RwLock`, `OnceCell`, atomics — needs to bypass `&T` immutability. You can't just `*(&t as *const T as *mut T) = ...`. That's UB: you've written through a `&T`, and the optimizer is allowed to assume that doesn't happen.

### The solution

`UnsafeCell<T>` is a **language-level marker**. The compiler treats data inside it specially: it knows the value can change behind a shared reference and will not optimize as aggressively.

```rust
use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T: Copy> Cell<T> {
    pub fn set(&self, val: T) {
        // SAFETY: we don't hand out &T to the inner value, so no aliasing.
        unsafe { *self.value.get() = val; }
    }
    pub fn get(&self) -> T {
        unsafe { *self.value.get() }
    }
}
```

`UnsafeCell<T>::get(&self) -> *mut T` is the only safe way to obtain a mutable pointer through a shared reference. It's how interior mutability works **all the way down**.

### Rules to remember

1. **Only `UnsafeCell` data may be mutated through `&T`.** Mutating any other `&T`-reachable memory is UB.
2. `UnsafeCell<T>` is **not** `Sync` even if `T` is. You opt in (`Mutex<T>: Sync where T: Send`, etc.).
3. `&UnsafeCell<T>` does **not** assert non-aliasing. The whole point.
4. **You still uphold `&mut`/`&` exclusivity for any references you hand out.** `RefCell` enforces this at runtime; `Mutex` enforces with locks; `Cell<T>` avoids the issue by `T: Copy` and never handing out references.

### When to reach for `UnsafeCell` directly

Almost never. Use the wrappers (`Cell`, `RefCell`, `Mutex`, `RwLock`, `OnceCell`, `AtomicU32`, …) instead. Reach for `UnsafeCell` directly when you're building a *new* synchronization primitive — and at that point you should be reading the [Nomicon's Send/Sync chapter](https://doc.rust-lang.org/nomicon/send-and-sync.html) and the [`atomic` docs](https://doc.rust-lang.org/std/sync/atomic/index.html).

### `Pin` shows up here too

Self-referential types (a struct that holds a pointer to its own field) are unsound under normal aliasing — moving the struct invalidates the pointer. `Pin<P>` plus `UnsafeCell` model "this won't move." Async generators use this internally. You'll meet `Pin` when implementing a `Future` by hand; reach for [`pin-project`](https://docs.rs/pin-project) before raw `unsafe`.

## How they compose

Real abstractions usually combine both:

```rust
// A toy lock-free single-producer single-consumer slot
pub struct Slot<T> {
    cell: UnsafeCell<MaybeUninit<T>>,
    written: AtomicBool,
}
```

The `UnsafeCell` allows mutation through `&Slot<T>`. The `MaybeUninit` allows the cell to start empty. The `AtomicBool` synchronizes producer and consumer. Drop and Sync impls require careful reasoning — start small.

## Exit criteria
- [ ] You can explain why `mem::uninitialized::<u32>()` was deprecated.
- [ ] You can write `MaybeUninit`-based array initialization with a drop-guard for panic safety.
- [ ] You can state the rule "only `UnsafeCell` may be mutated through `&T`" and give an example of why.
- [ ] You know which standard wrappers (`Cell`, `RefCell`, `Mutex`, atomics) to reach for before touching `UnsafeCell` directly.
