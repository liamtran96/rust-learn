---
title: 12. Advanced Topics
tags: [rust, advanced]
---

# 12. Advanced Topics

Optional. Pick up as you need them.

## Contents
- [[unsafe|12.1 Unsafe Rust]]
- [[macros|12.2 Macros — declarative & procedural]]
- [[ffi|12.3 FFI — talking to C]]
- [[advanced-traits|12.4 Advanced traits — associated types, HRTBs, specialization]]

## When to go here

- You're building a data structure the borrow checker can't accept directly (linked list, arena).
- You're wrapping a C library.
- You're writing a macro to reduce boilerplate or build a DSL.
- You're pushing for the last N% of performance and need layout / dispatch control.

**Most real Rust programmers rarely touch this chapter.** It's here so you know what exists.
