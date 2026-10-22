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
