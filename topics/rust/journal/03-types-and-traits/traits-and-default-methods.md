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

### 2026-09-22 - Static and dynamic trait dispatch
**Working on:** `Summary` trait in `code/03-types-and-traits/summary-trait/`
**What clicked:** A generic bound such as `T: Summary` gives one concrete type static dispatch at each call, while `Box<dyn Summary>` erases concrete types behind one runtime-dispatched interface so heterogeneous values can share a collection. Standalone notification functions consume the trait contract without becoming requirements that every implementing type must define, and iterating over a borrowed slice preserves ownership of its boxes.
**What didn't:** The two notification functions were first declared inside `Summary`, which made them required associated functions and prevented the trait from being dyn-compatible. The first generic call omitted its borrowed argument and tried to format the function's unit return value; constructing and traversing `Vec<Box<dyn Summary>>` was also unfamiliar.
**Questions asked this session:**
- **Q:** "what are they? `notify<T: Summary>(s: &T)` and `notify_dyn(items: &[Box<dyn Summary>])`"
  - **Prompt context:** Exercise 6 introduced two notification APIs immediately after defining `Summary`, and their different parameter forms needed decoding.
  - **Prompt code:** `fn notify<T: Summary>(s: &T)` and `fn notify_dyn(items: &[Box<dyn Summary>])`
  - **Liam's answer:** -
  - **Technical answer:** The generic function uses static dispatch: the compiler chooses a concrete `T` implementing `Summary` for each call. The trait-object function uses dynamic dispatch: each `Box<dyn Summary>` owns a possibly different concrete value and method calls are selected through a runtime vtable.
  - **Plain-English analogy / example:** A generic call is a production line configured for one model at a time. A `Vec<Box<dyn Summary>>` is a mixed delivery truck whose packages have different contents but all expose the same `summarize` label.
  - **See also:** `topics/rust/03-types-and-traits/traits.md`; `topics/rust/03-types-and-traits/trait-objects.md`
- **Q:** "ok what's next"
  - **Prompt context:** After learning the signatures, the unfinished source had placed both notification functions inside the trait declaration.
  - **Prompt code:** `trait Summary { fn summarize(&self) -> String; fn notify<T: Summary>(s: &T); fn notify_dyn(items: &[Box<dyn Summary>]); }`
  - **Liam's answer:** -
  - **Technical answer:** `notify` and `notify_dyn` are consumers of the `Summary` behavior, not behavior each implementor must provide, so they belong as free functions outside the trait. Keeping generic associated functions without a receiver inside the trait also prevents creation of the vtable required by `dyn Summary`.
  - **Plain-English analogy / example:** `Summary` is the plug standard; `notify` is an appliance using that plug. Making the appliance part of the standard would force every compatible device to manufacture its own appliance.
  - **See also:** `topics/rust/03-types-and-traits/trait-objects.md`
- **Q:** "still dont know how to do these 2"
  - **Prompt context:** The remaining tasks were to iterate through `notify_dyn` and construct a heterogeneous collection of two `Summary` implementors.
  - **Prompt code:** `fn notify_dyn(items: &[Box<dyn Summary>]) { }`
  - **Liam's answer:** -
  - **Technical answer:** Iterating over `&[Box<dyn Summary>]` yields shared references to the boxes, and dereference coercion lets each item call `summarize` without moving the boxed value. An explicit `Vec<Box<dyn Summary>>` annotation lets `Box::new` values of different implementing types coerce to the same trait-object element type.
  - **Plain-English analogy / example:**
    ```rust
    for item in items {
        println!("{}", item.summarize());
    }
    ```
  - **See also:** `topics/rust/03-types-and-traits/trait-objects.md`
- **Q:** "what should i add to make sure function work correctly `impl Summary for Article { fn summarize(&self) -> String { } }`"
  - **Prompt context:** The second type implemented the trait but its method body did not yet produce the promised owned `String`.
  - **Prompt code:** `impl Summary for Article { fn summarize(&self) -> String { } }`
  - **Liam's answer:** -
  - **Technical answer:** The body must evaluate to an owned `String`; `format!` creates one while borrowing fields through `&self`. Leaving the final expression without a semicolon returns that value from the method.
  - **Plain-English analogy / example:**
    ```rust
    fn label(&self) -> String {
        format!("Article: {}", self.title)
    }
    ```
  - **See also:** `topics/rust/03-types-and-traits/traits.md`; `topics/rust/pitfalls.md`
**Question to answer later:** When should a production API prefer a generic trait bound over a trait object?
**Next:** Begin Ch 3 exercise 7: use the newtype pattern to make `UserId` and `OrderId` distinct types.
