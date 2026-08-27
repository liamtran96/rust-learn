---
title: Fundamentals Journal - Retrieval Review
tags: [rust, journal, fundamentals]
---

# Retrieval Review

> Fundamentals topic journal. Return to [[../../journal|Journal index]].

## Entries

### 2026-08-21 — Retrieval homework: expressions, bindings, and safe CLI input
**Working on:** `topics/rust/homework/2026-08-18-retrieval-mixed.md` with saved code in `code/01-fundamentals/homework-practice/`
**What clicked:** Completed all six retrieval questions after guided retries. Tail expressions pass values out of blocks, `()` means no meaningful value, shadowing creates a new binding while assignment changes an existing mutable binding, `break value` gives a `loop` its value, string literals have program-long storage, and safe CLI input validates length before borrowing and parsing.
**What didn't:** Semicolon placement and `println!` returning `()` still caused repeated type-flow mistakes. The first explanations tied shadowing to memory addresses, treated string literals as if they died with the function scope, and indexed `args[1]` before validating the vector length; assembling `match` syntax also required several focused retries.
**Questions asked this session:**
- **Q:** What is the difference between a statement and an expression, what does a trailing semicolon do, and what is `()`?
  - **Technical answer:** An expression evaluates to a value. A trailing semicolon can turn a value-producing expression into a statement that discards that value, so a block with no tail expression evaluates to unit, written `()`.
  - **Plain-English analogy / example:**
    ```rust
    let number = { 1 + 1 };  // i32: 2
    let unit = { 1 + 1; };   // (): value discarded
    println!("{number} {unit:?}");
    ```
  - **See also:** `topics/rust/01-fundamentals/functions.md`, `topics/rust/01-fundamentals/control-flow.md`
- **Q:** What does “binding” mean, and how does shadowing differ from assignment?
  - **Technical answer:** A binding connects a name to a value. Repeating `let` creates a new binding that hides the old one; assignment omits `let`, changes an existing binding, and requires that binding to be mutable. Rust does not promise separate memory addresses for shadowed bindings.
  - **Plain-English analogy / example:**
    ```rust
    let x = 5;
    let x = "five"; // new binding; type may change
    let mut y = 5;
    y = 6;          // same binding; value changes
    ```
  - **See also:** `topics/rust/01-fundamentals/variables.md`
- **Q:** Why does a type annotation not convert `i32` into `i64` for arithmetic?
  - **Technical answer:** An annotation constrains or checks a type; it does not perform a numeric conversion. `i32` and `i64` are distinct types, so both operands must be made the same type explicitly before addition.
  - **Plain-English analogy / example:**
    ```rust
    let a = 100_i32;
    let b = 200_i64;
    let total = i64::from(a) + b;
    println!("{total}");
    ```
  - **See also:** `topics/rust/01-fundamentals/data-types.md`
- **Q:** Why did `direction` return `()` instead of `String`, and did I describe semicolon suppression incorrectly?
  - **Technical answer:** Each `.to_string();` was an expression statement whose `String` result was discarded, making each branch produce `()`. Removing the semicolons lets both branches and the complete `if` expression produce `String`; the correction was right, but “cannot suppress” stated the rule backwards.
  - **Plain-English analogy / example:**
    ```rust
    let direction = if true {
        "up".to_string()
    } else {
        "down".to_string()
    };
    ```
  - **See also:** `topics/rust/01-fundamentals/control-flow.md`
- **Q:** Should I add `let total =+ 1`, and where must the loop counter be declared?
  - **Technical answer:** `=+` is not an increment operator, and the `total` being declared does not exist while Rust evaluates its right-hand side. A separate mutable counter must exist before the loop; `break counter` then becomes the loop expression’s final value.
  - **Plain-English analogy / example:**
    ```rust
    let mut counter = 0;
    let total = loop {
        counter += 1;
        break counter;
    };
    ```
  - **See also:** `topics/rust/01-fundamentals/variables.md`, `topics/rust/01-fundamentals/control-flow.md`
- **Q:** How can `1_000` and `u32::MAX` be passed when I thought the argument was `i32`?
  - **Technical answer:** The function parameter was `u32`, not `i32`. Underscores only improve literal readability, and an unsuffixed literal such as `1_000` is inferred as `u32` from the function parameter; `u32::MAX` is already a `u32` and equals `4_294_967_295`.
  - **Plain-English analogy / example:**
    ```rust
    let inferred: u32 = 1_000;
    let explicit = 1_000_u32;
    assert_eq!(inferred, explicit);
    assert_eq!(u32::MAX, 4_294_967_295);
    ```
  - **See also:** `topics/rust/01-fundamentals/data-types.md`
- **Q:** Why can `shipping_band` return `&'static str`, and do its literals disappear when the function leaves scope?
  - **Technical answer:** A string literal is embedded in the compiled program and has the type `&'static str`; `'static` means the referenced bytes remain valid for the program run. A local reference variable can leave scope, but the literal data does not disappear then. Creating a `String` would instead create owned runtime text, normally with a heap allocation.
  - **Plain-English analogy / example:**
    ```rust
    fn label() -> &'static str {
        "standard" // embedded literal; valid for the program run
    }
    println!("{}", label());
    ```
  - **See also:** `topics/rust/01-fundamentals/functions.md`, `topics/rust/04-collections/strings.md`
- **Q:** How do we know the literals are stored in the binary, what is a compiled binary, and what does it look like?
  - **Technical answer:** `rustc` translates source into a machine-readable executable containing CPU instructions and program data. Searching the compiled `.exe` found `heavy`, `standard`, and `light` at concrete byte offsets; a hex dump showed the Windows `MZ` header and the literals’ UTF-8 byte values.
  - **Plain-English analogy / example:**
    ```text
    inspect-static.rs --rustc--> inspect-static.exe
    light -> 6C 69 67 68 74
    4D 5A -> MZ, the Windows executable signature
    ```
  - **See also:** `topics/rust/01-fundamentals/toolchain.md`
- **Q:** Why could I not open `findstr.exe /M /C:"light" program.exe`, and what was the `.pdb` file?
  - **Technical answer:** `findstr.exe` is a terminal command, and `program.exe` was only a placeholder that had to be replaced by a real compiled filename. A `.pdb` is a Windows debug-symbol file produced alongside some builds; it maps machine code back to source information and can be regenerated.
  - **Plain-English analogy / example:**
    ```text
    rustc inspect-static.rs -o inspect-static.exe
    findstr.exe /M /C:"light" inspect-static.exe
    inspect-static.pdb -> debugger map, not the program itself
    ```
  - **See also:** `topics/rust/01-fundamentals/toolchain.md`
- **Q:** How do I create a folder, save the homework code, and move the code into it?
  - **Technical answer:** A plain directory was created under the Chapter 1 code area, and each code answer was saved as its own `.rs` file. Git does not track an empty directory, so a placeholder made it visible until real files existed; no existing Cargo crate source was moved or broken.
  - **Plain-English analogy / example:**
    ```text
    code/01-fundamentals/homework-practice/
    |-- question-04.rs
    |-- question-05.rs
    `-- question-06.rs
    ```
  - **See also:** `topics/rust/cheatsheets/cargo-commands.md`
- **Q:** What should I do next? I mean my homework.
  - **Technical answer:** Repository progress still named temperature-converter cleanup as the roadmap action, but the immediate conversational task was the unfinished retrieval set. Question 5 (`shipping_band`) came next, followed by Question 6’s safe CLI flow; this did not change the official 7/7 Chapter 1 count.
  - **Plain-English analogy / example:**
    ```text
    Immediate task: retrieval homework Q5 -> Q6
    Official progress: Chapter 1 remains 7/7
    Roadmap next: temperature-converter cleanup
    ```
  - **See also:** `topics/rust/homework/2026-08-18-retrieval-mixed.md`, `topics/rust/progress.md`
- **Q:** I do not know how to do Exercise 6, and I do not remember the syntax—how should safe CLI parsing flow?
  - **Technical answer:** Collect arguments, validate that index 1 exists, borrow it, then call `parse::<u32>()`. Parsing returns `Result`; `Ok(value)` yields the number, while `Err(_)` prints an error and returns early. The complete flow was rebuilt incrementally instead of copied from the temperature converter.
  - **Plain-English analogy / example:**
    ```rust
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 { return; }
    let input = &args[1];
    let parsed = input.parse::<u32>();
    ```
  - **See also:** `topics/rust/01-fundamentals/data-types.md`
- **Q:** Why use `args.len()` before `args[1]`, and what does `&args[1]` do?
  - **Technical answer:** A vector with length 1 has only index 0, so indexing at 1 first can panic. `args.len() < 2` checks the collection before access, and `&args[1]` borrows the `String`—temporarily refers to it—without moving it out of the vector.
  - **Plain-English analogy / example:**
    ```rust
    if args.len() < 2 {
        return; // index 1 does not exist
    }
    let input = &args[1]; // safe borrow after guard
    ```
  - **See also:** `topics/rust/01-fundamentals/data-types.md`
- **Q:** What do `Ok` and `Err` mean here, and why can the `Ok` arm not just call `println!`?
  - **Technical answer:** `Ok(value)` and `Err(error)` are the two variants of `Result`; matching them either extracts the parsed `u32` or handles failure. Because `let quantity: u32 = match ...` requires the match expression to produce `u32`, the `Ok` arm must yield the number; `println!` returns `()`, so printing belongs after the match has created `quantity`.
  - **Plain-English analogy / example:**
    ```rust
    let quantity: u32 = match parsed {
        Ok(value) => value,
        Err(_) => return,
    };
    println!("{quantity}");
    ```
  - **See also:** `topics/rust/01-fundamentals/control-flow.md`, `topics/rust/01-fundamentals/functions.md`
**Question to answer later:** —
**Next:** Resume Phase 2 by reading `topics/rust/02-ownership/slices.md`, then predict and improve Ch 2 snippet D.

