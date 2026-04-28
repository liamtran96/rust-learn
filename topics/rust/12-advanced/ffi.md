---
title: 12.3 FFI
tags: [rust, ffi, c]
---

# 12.3 FFI — Talking to C

## Calling C from Rust

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("{}", abs(-3));
    }
}
```

Calls through `extern "C"` are always `unsafe` — the compiler has no idea whether the C side upholds Rust's invariants.

## Types & `#[repr(C)]`

For data you pass across the boundary, use C-compatible layout:

```rust
#[repr(C)]
struct Point { x: f64, y: f64 }
```

Use types from `std::os::raw` or `libc`: `c_int`, `c_char`, `c_void`.

## Strings across FFI

- `CString` — owned, null-terminated bytes (Rust → C).
- `CStr` — borrowed, null-terminated view (C → Rust).
- Rust `&str` / `String` are **not** null-terminated — you must convert.

```rust
use std::ffi::{CString, CStr};

let c_str = CString::new("hello").unwrap();    // Rust → C
let borrowed = unsafe { CStr::from_ptr(ptr) }; // C → Rust
let rust_str = borrowed.to_str()?;             // validate UTF-8
```

## Exposing Rust to C

```rust
#[no_mangle]
pub extern "C" fn my_add(a: i32, b: i32) -> i32 { a + b }
```

`#[no_mangle]` keeps the symbol name; `extern "C"` gives the C calling convention. Cargo.toml:

```toml
[lib]
crate-type = ["cdylib"]     # produces a .so / .dylib / .dll
```

## Binding generation

- [`bindgen`](https://github.com/rust-lang/rust-bindgen) — autogenerate Rust bindings from C headers.
- [`cbindgen`](https://github.com/mozilla/cbindgen) — generate C headers from Rust.

## Going deeper

For panic safety across boundaries, opaque handle wrappers, Send/Sync over FFI, allocator mismatch, and CI integration with sanitizers, see [[../14-unsafe/ffi-safety|14.7 FFI safety beyond the basics]].

## Related
- [[unsafe|Unsafe]]
- [[../14-unsafe/index|Chapter 14 — Unsafe deep dive]]
