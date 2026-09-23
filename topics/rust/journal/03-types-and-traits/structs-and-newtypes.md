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

### 2026-09-23 - Point derives started
**Working on:** Point derives in `code/03-types-and-traits/point-derives/`
**What clicked:** A derive attribute asks the compiler to generate trait `impl` blocks using each field's trait behavior. `Debug` produces a textual representation, while `PartialEq` compares two `Point` values field by field. A formatted value is a `String`, so its expected test value must also be text rather than another `Point`.
**What didn't:** Derive and test syntax were initially unfamiliar. The Debug assertion first used an invalid brace expression and then compared the formatted `String` with a `Point`; `Clone`, `Copy`, `Eq`, and `Hash` still need verification, and strict Clippy still reports that the normal binary never constructs `Point`.
**Questions asked this session:**
- **Q:** `i still dont understand can u please help me explain more detail about derive(...) asks the compiler to generate standard trait implementations`
  - **Prompt context:** Understand the derive explanation in the new exercise brief before implementing `Point`.
  - **Prompt code:** `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]`
  - **Liam's answer:** -
  - **Technical answer:** A trait is a capability contract, and an implementation (`impl`) supplies that behavior for a type. A derive macro runs during compilation and generates the routine `impl` for each listed trait by composing the corresponding behavior of the struct's fields.
  - **Plain-English analogy / example:** A struct is a blank employee record; deriving traits is asking the compiler to issue selected badges such as printable, comparable, or hashable according to standard company rules.
  - **See also:** `topics/rust/03-types-and-traits/structs.md`
- **Q:** `why do we need the derive`
  - **Prompt context:** Understand why defining `Point` fields alone does not permit debugging, equality, copying, or hashing operations.
  - **Prompt code:** `struct Point { x: i32, y: i32 }`
  - **Liam's answer:** -
  - **Technical answer:** Rust does not automatically assign every possible behavior to a custom type. Deriving a trait opts into its standard implementation when that field-by-field behavior is appropriate, avoiding repetitive manual code while keeping the capability explicit.
  - **Plain-English analogy / example:** Defining a record says what information it stores; deriving a capability says which standard operations Rust is allowed to perform on that record.
  - **See also:** `topics/rust/03-types-and-traits/structs.md`
- **Q:** `give me some hints`
  - **Prompt context:** Begin the first test demonstrating the behavior generated by `Debug`.
  - **Prompt code:** `struct Point { x: i32, y: i32 }`
  - **Liam's answer:** -
  - **Technical answer:** `format!` applies formatting and returns an owned `String` instead of printing to stdout. A test can construct a value, format it with `:?`, and compare the resulting text with an independently written string literal.
  - **Plain-English analogy / example:** `Debug` is like turning an object into a labeled inspection card; the card is text, not the original object.
  - **See also:** `topics/rust/03-types-and-traits/structs.md`
- **Q:** `please i am not familiar with the syntax please help me`
  - **Prompt context:** Write a second test that compares equal and unequal `Point` values using the derived `PartialEq` behavior.
  - **Prompt code:** -
  - **Liam's answer:** -
  - **Technical answer:** `#[test]` marks a zero-argument function for the Rust test harness. `assert_eq!` and `assert_ne!` are assertion macros; they compare their two expressions using `PartialEq` and fail the test when the expected relationship is false.
  - **Plain-English analogy / example:** An equality assertion is a checkpoint that says the two supplied values must match; an inequality assertion says they must differ.
  - **See also:** `topics/rust/03-types-and-traits/structs.md`
**Question to answer later:** How can `Clone`, `Copy`, `Eq`, and `Hash` each be verified without triggering Clippy's `clone_on_copy` lint?
**Next:** Construct and debug-print one `Point` in `main`, then rerun `cargo clippy -- -D warnings` before adding the remaining trait tests.
