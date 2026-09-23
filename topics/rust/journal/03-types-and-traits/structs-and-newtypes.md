---
title: Structs and Newtypes Journal
tags: [rust, journal, structs, newtypes]
---

# Structs and Newtypes Journal

> Back to [[../../journal|Journal index]].

## Entries

### 2026-09-23 - Typed IDs with newtypes
**Working on:** Typed IDs in `code/03-types-and-traits/typed-ids/`
**What clicked:** A one-field tuple struct creates a distinct domain type even when another wrapper has the same inner `u64`. Giving a function a `UserId` parameter preserves the meaning of the number and makes passing an `OrderId` a compile-time `E0308` mismatch. In `Vec<T>`, `T` is a placeholder that becomes the collection's single element type; for a mixed trait-object collection, that type is `Box<dyn Summary>`.
**What didn't:** Tuple-struct declaration syntax was unfamiliar: the first attempt modeled both IDs as fields of one struct, and angle-bracket placeholders were then copied as literal Rust. The initial explanation of newtype safety focused on passing a user ID but missed that changing the parameter to raw `u64` erases whether the number came from `UserId` or `OrderId`.
**Questions asked this session:**
- **Q:** `I dont really remember why is that`
  - **Prompt context:** Recall why a `Vec<Box<dyn Summary>>` can hold values whose inner concrete types differ.
  - **Prompt code:** `Vec<Box<dyn Summary>>`
  - **Liam's answer:** -
  - **Technical answer:** `Vec<T>` is homogeneous, meaning every element has one concrete element type. Here that type is `Box<dyn Summary>`; each box may hide a different concrete implementor behind the same trait-object interface, and method calls use dynamic dispatch through runtime metadata.
  - **Plain-English analogy / example:** The boxes are identical shipping containers labeled `Summary`, while their contents may differ. The vector stores the uniform containers, not the concrete contents directly.
  - **See also:** `topics/rust/03-types-and-traits/trait-objects.md`
- **Q:** `It's T type right`
  - **Prompt context:** Identify the concrete element type after substituting the generic placeholder in `Vec<T>` for the mixed summary collection.
  - **Prompt code:** `Vec<Box<dyn Summary>>`
  - **Liam's answer:** `It's T type right`
  - **Technical answer:** `T` is the parameter name in the generic definition of `Vec`, not the final type selected by this use. For `Vec<Box<dyn Summary>>`, Rust substitutes `Box<dyn Summary>` for `T`, so every vector element has that concrete outer type while its boxed value may have a different hidden concrete type.
  - **Plain-English analogy / example:** In `Vec<i32>`, `T` becomes `i32`; in `Vec<Box<dyn Summary>>`, `T` becomes `Box<dyn Summary>`. A blank on a form is not the filled-in answer.
  - **See also:** `topics/rust/03-types-and-traits/generics.md`; `topics/rust/03-types-and-traits/trait-objects.md`
- **Q:** `I am not familiar with the syntax`
  - **Prompt context:** Declare `UserId` and `OrderId` as separate one-field tuple structs after initially writing one named-field struct.
  - **Prompt code:** `struct TypeName{ UserId: u64, OrderId: u64, }`
  - **Liam's answer:** -
  - **Technical answer:** `struct Name(Type);` declares a tuple struct whose fields are accessed by position, such as `.0`. Each declaration introduces a new type, so two wrappers around `u64` are not interchangeable even though their inner representation matches.
  - **Plain-English analogy / example:**
    ```rust
    struct Meters(u64);
    let distance = Meters(15);
    assert_eq!(distance.0, 15);
    ```
  - **See also:** `topics/rust/03-types-and-traits/structs.md`
**Question to answer later:** -
**Next:** Begin Ch 3 exercise 8: derive and verify `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, and `Hash` for `Point`.
