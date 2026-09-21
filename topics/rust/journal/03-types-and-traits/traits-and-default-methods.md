---
title: Traits and Default Methods Journal
tags: [rust, journal, traits, methods]
---

# Traits and Default Methods Journal

> Back to [[../../journal|Journal index]].

## Entries

### 2026-09-21 - Animal trait and default method
**Working on:** `Animal` trait in `code/03-types-and-traits/animal-trait/`
**What clicked:** A trait is a compiler-enforced behavior contract; `Dog` and `Cat` can store different data yet provide the same required methods. An owned `String` field can be exposed as a borrowed `&str`, and a default `describe` method can call each implementor's `name` and `sound` methods.
**What didn't:** Trait syntax was initially unfamiliar. Method receivers were confused with parameter types, returning an owned `String` through `&self` caused a move error, and the first tests compared whole structs with display strings instead of checking method results.
**Questions asked this session:**
- **Q:** "what the hell is trait?"
  - **Prompt context:** The next exercise required defining `Animal` with required methods and a default method before implementing it for `Dog` and `Cat`.
  - **Prompt code:** `trait Animal { fn name(&self) -> &str; fn sound(&self) -> String; }`
  - **Liam's answer:** -
  - **Technical answer:** A trait declares behavior that concrete types can implement. Its method signatures form a contract checked by the compiler, while methods with bodies provide default behavior that implementations may inherit.
  - **Plain-English analogy / example:** A trait is like a USB standard: it specifies the operations a compatible device must support without specifying the device's internal data. `Dog` and `Cat` are different devices that both satisfy the `Animal` contract.
  - **See also:** `topics/rust/03-types-and-traits/traits.md`
- **Q:** "why do we use &self here not self?"
  - **Prompt context:** Decide how the read-only `name`, `sound`, and `describe` methods should receive the current animal.
  - **Prompt code:** `fn name(&self) -> &str;`
  - **Liam's answer:** -
  - **Technical answer:** `&self` temporarily borrows the current animal, so the method can inspect it without consuming it. A receiver of `self` would move the value into the call, and a borrowed name could not safely refer into an owner consumed by that method.
  - **Plain-English analogy / example:** `&self` is borrowing a library book to read it; `self` is transferring ownership of the book. Read-only methods need only the temporary loan, allowing later calls on the same value.
  - **See also:** `topics/rust/02-ownership/borrowing.md`; `topics/rust/03-types-and-traits/traits.md`
- **Q:** "why dont we use &str here, name: String"
  - **Prompt context:** Choose the field type for a `Dog` whose `name(&self)` method returns `&str`.
  - **Prompt code:** `struct Dog { id: u32, name: String }`
  - **Liam's answer:** -
  - **Technical answer:** `String` makes the dog own its name, so the text remains valid for the dog's lifetime. The method borrows the dog and returns an `&str` view into that owned `String`; storing `&str` instead would make the dog borrow from another owner and require an explicit lifetime relationship.
  - **Plain-English analogy / example:** The `Dog` owns the name like a person owns an ID card. Returning `&str` lets a caller read the card without taking it away.
  - **See also:** `topics/rust/02-ownership/ownership.md`; `topics/rust/02-ownership/lifetimes.md`
- **Q:** "wait did u add the bacon for animal-trait?"
  - **Prompt context:** Confirm whether the new crate had repository-root Bacon check and run aliases after scaffolding.
  - **Prompt code:** -
  - **Liam's answer:** -
  - **Technical answer:** The aliases had not survived the initial patch-helper failure, so `animal-trait` and `animal-trait-run` were added to `bacon.toml`. `bacon --project . --list-jobs` then confirmed both jobs and their `cargo check` and `cargo run` commands.
  - **Plain-English analogy / example:** `bacon animal-trait` watches and checks the crate; `bacon animal-trait-run` watches and runs it. These are repository-root shortcuts pointing at `code/03-types-and-traits/animal-trait/`.
  - **See also:** `bacon.toml`
**Question to answer later:** How do static trait bounds and `dyn Trait` choose between compile-time and runtime dispatch?
**Next:** Begin Ch 3 exercise 6: implement `Summary`, then compare `notify<T: Summary>` with `notify_dyn(&[Box<dyn Summary>])`.
