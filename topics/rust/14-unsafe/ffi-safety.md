---
title: 14.7 FFI Safety Beyond the Basics
tags: [rust, unsafe, ffi, abi]
---

# 14.7 FFI Safety Beyond the Basics

[[../12-advanced/ffi|Ch 12.3]] covers the syntax: `extern "C"`, `#[repr(C)]`, `CString`/`CStr`, `bindgen`/`cbindgen`. This page is about the **soundness** issues that bite when you go past "hello, libc."

## The seven traps

### 1. Panicking across an FFI boundary

If a Rust callback panics and the unwind crosses into C, **that's UB** (unless the C side compiled with the matching unwinding ABI, which it almost never did).

**Fix:** wrap any callback that runs from C in `catch_unwind` and convert panics to error codes.

```rust
use std::panic::{catch_unwind, AssertUnwindSafe};

extern "C" fn my_callback(ctx: *mut MyCtx, arg: i32) -> i32 {
    let result = catch_unwind(AssertUnwindSafe(|| {
        // … your Rust logic that may panic …
        do_work(unsafe { &mut *ctx }, arg)
    }));
    match result {
        Ok(value) => value,
        Err(_)   => -1, // sentinel error
    }
}
```

Or compile with `panic = "abort"` in `Cargo.toml`:
```toml
[profile.release]
panic = "abort"
```
Now panics terminate the process — no UB across FFI, but no recovery either.

### 2. ABI mismatch

Calling an `extern "C"` function with a Rust signature that doesn't match the C declaration is UB. Common slips:

- C `int` is **not** Rust `i32` everywhere — use `c_int`.
- C `long` is 32-bit on Windows, 64-bit on Linux/macOS — use `c_long`.
- C `size_t` is `usize`, but **only** because they happen to match on every modern platform. Be explicit.
- C arrays decay to pointers; **always pass length separately**.
- C uses `bool` from `<stdbool.h>` (1 byte), Rust `bool` is also 1 byte but only valid as 0/1. C must guarantee that.

**Fix:** use `bindgen` for headers you don't control. For headers you do control, run `cbindgen` and let the build verify.

### 3. Lifetime erasure

C pointers carry no lifetime. When you receive `*const T` from C, you cannot prove how long it's valid. When you pass `*const T` to C, the C side has no way to know when it's free to use.

**Fix:** wrap in safe abstractions that own or borrow appropriately.

```rust
pub struct Handle<'a> {
    raw: *mut c_void,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> Handle<'a> {
    pub fn new<T>(t: &'a mut T) -> Self {
        Handle { raw: t as *mut T as *mut c_void, _marker: PhantomData }
    }
}
```

`PhantomData<&'a ()>` says "this handle borrows for `'a`" — the borrow checker now enforces that `t` outlives the handle. The C function that takes `Handle::raw` must respect that lifetime by contract.

### 4. Send / Sync over FFI

C handles often have stronger or weaker thread-safety than your Rust signature suggests. The compiler defaults conservatively, but for opaque types (`*mut SqlStmt`, `*mut CurlHandle`) you might need:

```rust
pub struct CurlHandle { raw: *mut curl_sys::CURL }

// SAFETY: libcurl docs guarantee a CURL* may be moved between threads
// as long as it's not used concurrently from two threads.
unsafe impl Send for CurlHandle {}
// libcurl is NOT Sync — do not impl.
```

**Get this from the C library's documentation, not from "it compiles."**

### 5. Opaque types and C ownership

C structs whose layout is hidden behind `typedef struct foo *foo_t;` should be modeled as opaque in Rust:

```rust
#[repr(C)]
pub struct foo { _private: [u8; 0] } // can't be constructed; can't be sized

extern "C" {
    pub fn foo_new() -> *mut foo;
    pub fn foo_free(p: *mut foo);
}

pub struct Foo { raw: NonNull<foo> }

impl Foo {
    pub fn new() -> Option<Self> {
        let raw = unsafe { foo_new() };
        Some(Foo { raw: NonNull::new(raw)? })
    }
}

impl Drop for Foo {
    fn drop(&mut self) {
        // SAFETY: raw was returned from foo_new and not freed yet.
        unsafe { foo_free(self.raw.as_ptr()); }
    }
}
```

The opaque field `_private: [u8; 0]` ensures Rust can't construct `foo` directly or assume a size — it forces you through the C API.

### 6. Strings (UTF-8 vs platform encoding)

Rust strings are guaranteed UTF-8. C strings are bytes. They are not interchangeable.

| Direction | Rust type | C type | Conversion |
|---|---|---|---|
| Rust → C | `&str` | `*const c_char` | `CString::new(s)?.as_ptr()` (keep `CString` alive!) |
| C → Rust | `*const c_char` | `&str` | `CStr::from_ptr(p).to_str()?` |

**Pitfalls:**
- `CString::new` rejects strings with internal `\0`.
- `CStr::from_ptr` requires the pointer to be non-null and the string null-terminated.
- `OsString`/`OsStr` for filesystem paths on Windows (UTF-16) — don't assume UTF-8.

### 7. Allocator mismatch

If a C library hands you a `char*` allocated with `malloc`, you must `free` it with the C `free` (or the library's own free function). Calling Rust's `dealloc` on it is UB. Same in reverse.

**Fix:** model with explicit free functions:

```rust
pub struct CString { raw: *mut c_char }
impl Drop for CString {
    fn drop(&mut self) {
        // SAFETY: raw came from libfoo_alloc_string; libfoo expects libfoo_free.
        unsafe { libfoo_free(self.raw); }
    }
}
```

## Cross-language unwinding (`extern "C-unwind"`)

Rust 1.71+ stabilized `extern "C-unwind"` — declares a function whose calls *may* unwind. Use this for C++/Rust interop where C++ exceptions are expected to propagate. Don't use it for plain C.

## Linking and the build

- **Static vs dynamic**: choose in the `*-sys` crate's `build.rs`. Static is simpler to ship, dynamic respects system upgrades.
- **`pkg-config`**: standard for Linux; bake into `build.rs`.
- **vcpkg**: standard for Windows.
- **`#[link(name = "...")]`**: declarative; build script does the heavy lifting.

## CI for FFI safety

Add to your test matrix:
1. Build on each target you ship.
2. `cargo miri test` — catches some FFI UB (uninit reads, aliasing through Rust↔C boundaries that round-trip).
3. AddressSanitizer build — `RUSTFLAGS="-Z sanitizer=address" cargo +nightly test`.
4. ThreadSanitizer build — same with `thread`.
5. Run examples through `valgrind --error-exitcode=1` when available.

ASan and TSan catch things Miri can't (real-syscall UB) at the cost of running natively. Use both.

## Exit criteria
- [ ] You can explain why panicking across an `extern "C"` boundary is UB and how to prevent it.
- [ ] You can correctly model an opaque C handle with `Drop` calling the C-side free function.
- [ ] You can articulate when to `unsafe impl Send` for a C handle and where to find the answer.
- [ ] You handle string conversions in both directions without `unwrap()` in production paths.
- [ ] Your CI runs Miri *and* one sanitizer for FFI-touching code.
