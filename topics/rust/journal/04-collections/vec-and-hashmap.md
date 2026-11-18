---
title: Collections Journal - Vec And HashMap
tags: [rust, journal, collections, vec, hashmap]
---

# Vec And HashMap

Return to [[../../journal|Journal index]].

## Entries

### 2026-11-18 - Median and mode
**Working on:** Chapter 4 exercise 1, median and mode - `code/04-collections/median-mode/`
**What clicked:** Counting with `*counts.entry(*number).or_insert(0) += 1`; the "best so far" loop over `&counts` with `*value` / `*key` dereferences; casting each operand with `as f64` before arithmetic; returning a tuple `(median, mode)` as the tail expression and destructuring it with `let (median, mode) = ...`; ownership moving a `Vec` into a function, where the parameter needs its own `mut`.
**What didn't:** Both `if`/`else` branches must have one type (E0308). A function with no tail expression returns `()` (E0308). Placeholder `KEY` and a whole analogous example were copied literally instead of adapted. `key; value;` written as loop body. `mut` added to `main`'s binding instead of the function parameter (E0596). Clippy flagged `% 2 == 0` and `2 as f64`.
**Questions asked this session:**
- **Q:** "what is happening in this repo"
  - **Prompt context:** Start of session; orientation before continuing the scaffolded median-mode crate.
  - **Prompt code:** -
  - **Liam's answer:** -
  - **Technical answer:** The repo is a learning workspace: an Obsidian vault under `topics/rust/` (roadmap, study plan, journal, chapter notes) and hand-written Cargo crates under `code/<phase>/`. Progress sits at Phase 3 / Week 5, starting Chapter 4 collections with a partial median calculation.
  - **Plain-English analogy / example:** The vault is the textbook and notebook; `code/` is the workbench.
  - **See also:** `WORKFLOW.md`; `topics/rust/roadmap.md`
- **Q:** "what should i do next"
  - **Prompt context:** Median code worked only for odd lengths; mode not started.
  - **Prompt code:** -
  - **Liam's answer:** -
  - **Technical answer:** Five steps: fix the even-length median, satisfy clippy, count values in a `HashMap<i32, usize>` (a map from key to value) for the mode, extract a `median_mode` function returning `(f64, i32)`, then test edge cases `[7]` and `[]`.
  - **Plain-English analogy / example:** Get one piece correct and printing before wrapping it in a function.
  - **See also:** `topics/rust/exercises/ch04-collections.md`
- **Q:** "give me some hint"
  - **Prompt context:** Even-length median and the HashMap counting step.
  - **Prompt code:** -
  - **Liam's answer:** -
  - **Technical answer:** An even-length sorted list has two middle values at `middle - 1` and `middle`; the median is their average. For counting, `entry(key)` finds the map slot, `or_insert(0)` creates it if missing and returns `&mut usize` (a mutable reference), and `*` writes through that reference.
  - **Plain-English analogy / example:**
    ```rust
    let mut counts: HashMap<char, usize> = HashMap::new();
    for letter in &['a', 'b', 'a'] {
        *counts.entry(*letter).or_insert(0) += 1;
    } // {'a': 2, 'b': 1}
    ```
  - **See also:** `topics/rust/04-collections/hashmap.md`
- **Q:** "how to convert befor add and divine"
  - **Prompt context:** Averaging two `i32` values without losing the `.5`.
  - **Prompt code:** -
  - **Liam's answer:** -
  - **Technical answer:** `as` binds tighter than `+` and `/`, so `a as f64 + b as f64` casts each operand first. Integer division truncates (drops the fraction), so casting after dividing is too late. Rust never mixes `f64` and `i32` in arithmetic, so divide by the float literal `2.0`.
  - **Plain-English analogy / example:**
    ```rust
    let (a, b) = (7, 8);
    let wrong = ((a + b) / 2) as f64;        // 7
    let right = (a as f64 + b as f64) / 2.0; // 7.5
    ```
  - **See also:** `topics/rust/01-fundamentals/data-types.md`
- **Q:** "(numbers[middle] + numbers[middle - 1]) / 2 as f64 can i do this"
  - **Prompt context:** Proposed form of the even-length median.
  - **Prompt code:** `(numbers[middle] + numbers[middle - 1]) / 2 as f64`
  - **Liam's answer:** -
  - **Technical answer:** No. `as` casts only the operand next to it, the literal `2`. The left side stays `i32`, and `i32 / f64` does not compile because Rust has no implicit numeric conversion.
  - **Plain-English analogy / example:** The parse is `(i32 sum) / (2 as f64)` - a label stuck on the wrong box.
  - **See also:** `topics/rust/01-fundamentals/data-types.md`
- **Q:** "like this one (numbers[middle] as f64 + numbers[middle - 1] as f64) / 2 as f64"
  - **Prompt context:** Corrected attempt at the cast.
  - **Prompt code:** `(numbers[middle] as f64 + numbers[middle - 1] as f64) / 2 as f64`
  - **Liam's answer:** -
  - **Technical answer:** Correct: both operands are `f64` before addition. Clippy later flagged `2 as f64` as `unnecessary_cast`; write the float literal `2.0` (or `2_f64`) instead.
  - **Plain-English analogy / example:** `/ 2.0` - type the number as a float rather than converting it.
  - **See also:** `topics/rust/cheatsheets/cargo-commands.md`
- **Q:** "when the array is empty i got the error"
  - **Prompt context:** Edge case `vec![]` passed to `median_mode`.
  - **Prompt code:** `let numbers: Vec<i32> = vec![];`
  - **Liam's answer:** -
  - **Technical answer:** `length` is 0, which counts as even, so `numbers[middle]` runs as `numbers[0]`. Vec indexing is bounds-checked at runtime and panics with "index out of bounds: the len is 0 but the index is 0". The return type `(f64, i32)` cannot express "no answer", so the panic is documented; `Option<(f64, i32)>` is the later fix.
  - **Plain-English analogy / example:** Asking for the first page of an empty notebook - Rust stops instead of reading garbage.
  - **See also:** `topics/rust/04-collections/vec.md`
**Question to answer later:** Should `median_mode` return `Option<(f64, i32)>` for empty input once Chapter 5 covers `Option`? Should ties pick the smallest value instead of depending on HashMap order?
**Next:** Chapter 4 exercise 2, anagram groups, from `topics/rust/exercises/ch04-collections.md`.
