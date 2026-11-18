---
title: Ch 4 - My mistakes and misunderstandings
tags: [rust, collections, mistakes, review]
---

# Ch 4 - My mistakes and misunderstandings

## Open mistakes (review these)

### 2026-10-22 - Owned text confused with Copy (string-views warm-up)
- **What I wrote:** "the difference between String and &str is that: String is a copy type and &str is borrowed"
- **Why it's wrong:** Owning text does not imply implicit copying. `String` does not implement `Copy`; assignment such as `let b = a;` transfers ownership. The first attempt repeated this move and then used `a`, producing E0382.
- **The rule:** `String` owns its text buffer; `&str` borrows text. Use `&` when access should be borrowed, and distinguish copying a shared reference from duplicating the text it references.
- **Status:** fresh - corrected with guidance and implemented; independent recall pending
- **See also:** [[strings]]; [[../02-ownership/ownership]]

### 2026-11-18 - `as` casts only its neighbour (median-mode, ex 1)
- **What I wrote:** `(numbers[middle] + numbers[middle - 1]) / 2 as f64`
- **Why it's wrong:** `as` binds tighter than `/`, so only the literal `2` became `f64`. The sum stayed `i32`, giving an `i32 / f64` mismatch, and integer addition/division would truncate anyway.
- **The rule:** Cast each operand before the arithmetic: `(a as f64 + b as f64) / 2.0`.
- **Status:** 🔴 fresh

### 2026-11-18 - `if`/`else` branches with different types (median-mode, ex 1)
- **What I wrote:** `if` branch `(... as f64 + ... as f64) / 2 as f64`, `else` branch `numbers[middle]`
- **Why it's wrong:** `if`/`else` is an expression producing one value for one variable, so every branch must have the same type. `f64` vs `i32` is E0308.
- **The rule:** When an `if` is assigned to a variable, check that each branch ends in the same type.
- **Status:** 🔴 fresh

### 2026-11-18 - Function body with no tail expression (median-mode, ex 1)
- **What I wrote:** `fn median_mode(number: Vec<i32>) -> (f64, i32){ let mut counts: HashMap<i32, usize> = HashMap::new(); }`
- **Why it's wrong:** A function returns its last expression without `;`. A body ending in a `let` statement returns `()`, which breaks the promised `(f64, i32)`.
- **The rule:** The returned value must be the final line with no semicolon, e.g. `(median, mode)`.
- **Status:** 🔴 fresh

### 2026-11-18 - Copying placeholders and examples literally (median-mode, ex 1)
- **What I wrote:** `*counts.entry(KEY).or_insert(0) += 1;` outside a loop; later pasted the `letters` / `letter_counts` example unchanged, so `counts` stayed `{}`; then `key; value;` as a loop body.
- **Why it's wrong:** `KEY` was a placeholder, not a variable (E0425). An analogous example shows a shape to adapt, not code to paste. A bare `key;` names a value and discards it (path statement with no effect).
- **The rule:** Map each name in an example to my own variables before typing. A `does not need to be mutable` or `unused variable` warning means the code meant to use that variable is missing.
- **Status:** 🔴 fresh

### 2026-11-18 - `mut` on the wrong binding after a move (median-mode, ex 1)
- **What I wrote:** `let mut numbers: Vec<i32> = vec![1, 1, 2, 2];` in `main`, with `fn median_mode(numbers: Vec<i32>)` still calling `numbers.sort()`
- **Why it's wrong:** Passing `numbers` moves the Vec into the function's parameter, a separate binding. Mutability belongs to the binding, so the parameter needs `mut` (E0596) and `main`'s binding does not.
- **The rule:** The variable that calls a `&mut self` method such as `sort()` is the one that needs `mut`; each new owner declares its own mutability.
- **Status:** 🔴 fresh
