---
title: Collections Journal - Strings And Borrowed Views
tags: [rust, journal, collections, strings]
---

# Strings And Borrowed Views

Return to [[../../journal|Journal index]].

## Entries

### 2026-10-22 - Owned string and borrowed view warm-up
**Working on:** `code/03-types-and-traits/string-views/` - introductory collections warm-up, not a numbered Chapter 4 exercise.
**What clicked:** Liam correctly predicted that using a moved `String` fails, then changed a move into `let b: &str = &a;` and printed both variables. The idiom is shared borrowing with conversion from `&String` to a string slice, `&str`.
**What didn't:** `String` was initially described as a `Copy` type. The first code submission still moved `a`; the next borrowed correctly but did not print `b`. Ownership terminology was corrected with guidance; independent explanation of the final owner/view relationship remains to be recalled.
**Questions asked this session:**
- **Q:** "what's next"
  - **Prompt context:** Resume the recorded first move after the Chapter 3 signature retrieval session.
  - **Prompt code:** -
  - **Liam's answer:** -
  - **Technical answer:** Resume the existing `string-views` scaffold and create, borrow, and print owned text. Official progress remains Phase 3 / Week 5, 21 numbered exercises and 5 shipped projects.
  - **Plain-English analogy / example:** A small ownership refresher before the collections exercises; no new crate is needed.
  - **See also:** `WORKFLOW.md`; `code/03-types-and-traits/string-views/BRIEF.md`
- **Q:** What is the difference between `String` and `&str`? (retrieval prompt)
  - **Prompt context:** Recall which type owns text before implementing an owner and borrowed view.
  - **Prompt code:** -
  - **Liam's answer:** "the difference between String and &str is that: String is a copy type and &str is borrowed"
  - **Technical answer:** `String` owns a text buffer and does not implement `Copy`, the trait permitting implicit duplication during assignment. A shared string slice, `&str`, borrows existing text without owning its buffer; copying that reference does not copy the text.
  - **Plain-English analogy / example:** `String` owns the book; `&str` is a view into its pages.
  - **See also:** `topics/rust/04-collections/strings.md`; `topics/rust/02-ownership/ownership.md`
- **Q:** Will the move example compile, and who owns the text afterward? (retrieval prompt)
  - **Prompt context:** Predict whether the original binding remains usable after assignment to another variable.
  - **Prompt code:**
    ```rust
    let a = String::from("hello");
    let b = a;
    println!("{a}");
    ```
  - **Liam's answer:** "no becasue a was gone"
  - **Technical answer:** The prediction is correct: assignment transfers ownership to `b`, so using `a` afterward fails. The text has not disappeared or been freed; `b` now owns it.
  - **Plain-English analogy / example:** Handing someone a book changes its owner without destroying the book.
  - **See also:** `topics/rust/02-ownership/ownership.md`
- **Q:** "done please check"
  - **Prompt context:** Review the first submitted owner/view implementation.
  - **Prompt code:**
    ```rust
    fn main() {
        let a = String::from("hello");
        let b = a;
        println!("Hello, world!{}",{a});
    }
    ```
  - **Liam's answer:** Submitted the code above.
  - **Technical answer:** `cargo check` produced E0382, meaning a value was used after its ownership moved, and warned that `b` was unused. This task requires a shared borrow using `&`, rather than cloning the text as the compiler's general suggestion proposed.
  - **Plain-English analogy / example:** Let a reader view your book while you retain ownership; do not hand it over or photocopy it.
  - **See also:** `topics/rust/02-ownership/borrowing.md`
- **Q:** "dotn" and subsequent "done" review requests
  - **Prompt context:** Verify the revised borrow and finish printing both bindings.
  - **Prompt code:**
    ```rust
    let a = String::from("hello");
    let b : &str = &a;
    println!("Hello, world!{}",{a});
    ```
  - **Liam's answer:** Initially submitted the code above; later added a second print using `b` and explicitly borrowed `a` in the first print.
  - **Technical answer:** The borrow compiled; the initial warning remained because `b` was unused. Printing both values removed it. The explicit `&a` inside the final print is valid; optional cleanup is to use named formatting such as `println!("{a}")` without extra argument blocks.
  - **Plain-English analogy / example:** Print the owner's text once and the reader's view once; both display the same text buffer.
  - **See also:** `topics/rust/04-collections/strings.md`
**Verification:** `cargo fmt` applied by Codex. Final fmt/check/test/strict Clippy/run passed; the test harness contains 0 tests. Runtime printed `Hello, world!hello` on each of two lines. No solution logic was written by Codex.
**Official sources:** [Ownership and moves](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html), [String slices](https://doc.rust-lang.org/book/ch04-03-slices.html), [Shared references and Copy](https://doc.rust-lang.org/std/marker/trait.Copy.html), [println!](https://doc.rust-lang.org/std/macro.println.html).
**Question to answer later:** Independently explain why `a` still owns the text after `let b: &str = &a;`.
**Next:** Begin Chapter 4 exercise 1, median and mode; first recall the owner/view distinction, then clarify the median and mode requirements before scaffolding.
