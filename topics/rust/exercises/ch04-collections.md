---
title: Ch 4 — Collections Exercises
tags: [rust, exercises, collections]
---

# Ch 4 — Collections Exercises

1. **Median & mode** — given a `Vec<i32>`, return `(f64, i32)`.
2. **Anagram groups** — given `Vec<String>`, return `Vec<Vec<String>>` grouping anagrams.
3. **Frequency top-N** — given text, return top-N most frequent words.
4. **Pig Latin** — transform a sentence per the standard rules. Beware: input may have non-ASCII.
5. **Employees by department** — accept inputs like `"Add Alice to Engineering"` or `"List Engineering"`, `"List all"` — store in a `HashMap<String, Vec<String>>`.
6. **Two-sum** — given `&[i32]` and target `i32`, return `Option<(usize, usize)>` in O(n) using a `HashMap`.
7. **Run-length encoding** — `"aaabbc"` → `"a3b2c1"` and back.

Bonus: rewrite #3 with **only** iterator chains — no explicit `for` loops.
