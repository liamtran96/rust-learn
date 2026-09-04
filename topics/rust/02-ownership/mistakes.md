---
title: Ch 2 — My mistakes & misunderstandings
tags: [rust, ownership, mistakes, review]
---

# Ch 2 — My mistakes & misunderstandings

> Personal log of what I got wrong, why I got it wrong, and the rule that fixes it.
> Review these entries at the start of each Ch 2 study session until each rule feels obvious.

## Open mistakes (review these)

### 2026-09-04 - Copy elements treated as a Copy vector (retrieval Question 3)
- **What I wrote:** "Yes, It prints 3 because Vec<i32> is a copy type so we can reuse it"
- **Why it's wrong:** The element type i32 implements Copy, but Vec<i32> owns heap storage and does not implement Copy. The vector remains usable only when the loop iterates over &values and borrows the container instead of consuming it.
- **The rule:** Element traits do not automatically apply to the containing collection; for x in values consumes a Vec, while for x in &values borrows it.
- **Status:** 🟥 fresh

### 2026-09-04 - Vector mutation described without reallocation and NLL (retrieval Question 4)
- **What I wrote:** "it does not compile because first was changes" and later "if the vector's current memory is full can not push"
- **Why it's wrong:** first is not changed, and a full Vec can still grow. push may allocate a larger buffer and move the elements, so a shared reference into the old buffer cannot overlap the mutable borrow; under NLL the shared borrow ends after its final use.
- **The rule:** A shared reference into a Vec and a mutable borrow of that Vec cannot overlap; Vec growth may relocate elements, and a borrow lasts through its last actual use.
- **Status:** 🟥 fresh

### 2026-09-04 - Invalid slice split expected empty outputs (retrieval Question 5)
- **What I wrote:** "mid > values.len() then left and right is []"
- **Why it's wrong:** split_at_mut accepts boundary positions from 0 through len inclusive, but a position greater than len is outside the slice and panics at runtime. An empty slice has length zero and no valid element index, although zero is a valid split boundary.
- **The rule:** Slice indices are zero-based, split boundaries include len, and split_at_mut panics when mid > len.
- **Status:** 🟥 fresh

### 2026-09-04 - Owned parser input confused with a borrowed input (retrieval Question 6)
- **What I wrote:** Chose String for the input of a helper intended to borrow text and return &str.
- **Why it's wrong:** Taking String by value moves ownership into the helper. An &str input lets the caller remain the owner and lets the returned &str borrow the same storage; returning String instead creates an independently owned copy that can outlive the source.
- **The rule:** Use &str for a non-owning text input and borrowed output; use String when the result must own its characters, accepting the allocation and copy.
- **Status:** 🟥 fresh

### 2026-08-28 - Lifetime annotation treated as creating a struct lifetime (ownership drill d12)
- **What I wrote:** "'a creat a lifr time for struct Scanner"
- **Why it's wrong:** A lifetime annotation does not create or extend the lifetime of a value. Here, `'a` names the relationship between `Scanner<'a>` and its borrowed `source: &'a str`, allowing the compiler to reject a scanner that could be used after its source text is gone.
- **The rule:** Lifetimes describe compiler-checked validity relationships between references; they do not cause values to live longer.
- **Status:** d12 - fresh

### 2026-08-27 - Split position treated as padding or overlap (`split_at_mut`)
- **What I wrote:** "left= [10, 0] right [30,40]" and later "left = [10, 20, 30, 40] right=[40]" for split positions inside `[10, 20, 30, 40]`.
- **Why it's wrong:** Splitting a slice neither inserts placeholder values nor duplicates an element across the outputs. A split at `mid` returns the first `mid` elements on the left and every remaining element on the right, preserving order and keeping the mutable slices disjoint.
- **The rule:** For a valid split, `left.len() == mid`, `left.len() + right.len() == original.len()`, and concatenating left then right reconstructs the original without overlap, padding, or copying.
- **Status:** 🟥 fresh

### 2026-08-27 - Variables used outside their declaring block (`split_at_mut`)
- **What I wrote:** Put `(left, right)` and later `left[0] = 10; right[0] = 20;` inside `split_at_mut`, although `left` and `right` were declared by tuple destructuring inside `main`.
- **Why it's wrong:** A local binding exists only within the block where it is declared. The generic helper knows only its parameters `v` and `mid`; the names `left` and `right` belong to the caller after it destructures the returned tuple.
- **The rule:** Function bodies are separate scopes. A callee returns values using its own parameters and locals; the caller chooses names for those returned values and uses them within the caller's block.
- **Status:** 🟥 fresh

### 2026-08-27 - Borrowing a substring seemed redundant (`strip_margin`)
- **What I wrote:** "do we need borrow here" and asked why `line[start..]` cannot be used as a standalone `str` value.
- **Why it's wrong:** `line` is a `&str` for the whole line, but range indexing selects a region whose output type is `str`, a dynamically sized string value. The leading `&` creates a fixed-size `&str` containing the selected region's address and length; it does not copy the text or create `&&str`.
- **The rule:** A `str` is variable-length text data and is used behind a pointer; write `&text[start..]` to create a borrowed `&str` view of a substring.
- **Status:** 🆕 fresh

### 2026-08-27 - Word splitting destroyed line structure (`strip_margin`)
- **What I wrote:** `let words = s.split_whitespace();` followed by `words.collect::<Vec<&str>>().join(" ")`
- **Why it's wrong:** `split_whitespace` treats every run of whitespace, including newlines, as a separator. Joining with a space therefore flattens the input into words and loses the line boundaries that `strip_margin` must process independently.
- **The rule:** Choose an iterator whose units match the transformation: use `lines` for line-by-line work, and use `split_whitespace` only when whitespace-separated words are the intended units.
- **Status:** 🆕 fresh

### 2026-08-27 - Arithmetic result discarded instead of assigned (ownership drill d10)
- **What I wrote:** `let _ = *value * 2;` and then `*value * 2;`
- **Why it's wrong:** Both expressions calculate a doubled integer without writing it through the mutable reference, so the original slice remains unchanged. The empty-input test still passes because the loop body never runs, which does not verify mutation behavior.
- **The rule:** Dereference an `&mut T` and assign through it to change the referent; compound assignment such as `*value *= 2` both calculates and stores the result.
- **Status:** 🟥 fresh

### 2026-08-26 - `todo!()` and a discarded tail value (ownership drill d09)
- **What I wrote:** Predicted "no" to whether the file with `todo!()` compiles, then wrote `first.unwrap_or("");` before leaving `todo!()` in place.
- **Why it's wrong:** `todo!()` has the never type and can stand where a `&str` is expected, so the file compiles but panics when that line executes. The semicolon turns the desired `&str` into a discarded statement result; it must be the tail expression to become the function's return value.
- **The rule:** `todo!()` is a compiling runtime panic placeholder. A block returns its final expression only when that expression has no trailing semicolon.
- **Status:** 🟥 fresh

### 2026-08-26 - Lifetime annotation treated as extending values (ownership drill d08)
- **What I wrote:** "`<'a>` ... keep those parameter valid during run time"
- **Why it's wrong:** A lifetime annotation does not keep a value alive or alter runtime behavior. It names a relationship the compiler checks: both possible source references must be valid for `'a`, and the returned reference may be used for no longer than that shared period.
- **The rule:** Lifetimes describe and constrain relationships between references; they do not extend the lifetimes of the referenced values.
- **Status:** 🟥 fresh

### 2026-08-26 - Heap location confused with returned ownership (ownership drill d07)
- **What I wrote:** "WHY:in the heap"
- **Why it's wrong:** A `String` does store its text buffer on the heap, but memory location alone does not explain why the return is safe. The decisive change is that ownership of the `String` moves out of the function to the caller, so the heap buffer is not freed when the local function scope ends.
- **The rule:** A function may return an owned local value by moving it to the caller. It cannot return a reference to that local because the local owner would be dropped when the function returns.
- **Status:** 🟥 fresh

### 2026-08-26 - Owned `String` confused with deref coercion (ownership drill d06)
- **What I wrote:** "String is copy type" and "String will be automatically change to &str type."
- **Why it's wrong:** `String` is not `Copy`, and the owned `String` does not change its type. At the call to a function expecting `&str`, Rust coerces the argument from `&String` to `&str` because `String` dereferences to `str`.
- **The rule:** String literals already have type `&str`; an owned `String` remains a `String`, while a shared `&String` can undergo deref coercion to `&str` for function and method arguments.
- **Status:** 🟥 fresh

### 2026-08-26 - Copy confused with borrowing and iteration ownership (ownership drill d05)
- **What I wrote:** "because v is not a copy type and i use & to reference to the v we use it temporary"; the prediction also said "the x take the ownership from v."
- **Why it's wrong:** Whether the element type is `Copy` does not decide whether the vector is consumed. The owned-vector iterator takes `v` by value, while `x` receives one element at a time; iterating over `&v` instead borrows the vector and changes `x` from `i32` to `&i32`.
- **The rule:** `for x in collection` follows the collection's `IntoIterator` implementation. An owned `Vec<T>` is consumed and yields `T`; a shared borrow `&Vec<T>` is not consumed and yields `&T`.
- **Status:** 🟥 fresh

### 2026-08-17 — Reference target and `Vec` mutation (predict-and-fix B)
- **What I wrote:** Said the snippet compiled, that `r = &v[0]` referred to `4`, and that the concern was “maybe the original `v` can be reused somewhere.”
- **Why it's wrong:** Index `0` refers to the existing first element, `1`; the later `push(4)` appends at index `3`. More importantly, `push` may reallocate the vector's buffer, so Rust cannot allow that mutable borrow while `r` still needs an address inside the old buffer.
- **The rule:** A shared borrow `&T` and a mutable borrow `&mut T` cannot overlap. Use the shared reference for the last time before mutating its owner; non-lexical lifetimes let the borrow end at that final use.
- **Status:** 🟥 fresh

### 2026-08-24 — Returned slice used after its owner was dropped (predict-and-fix D)
- **What I wrote:** Predicted “yes” when asked whether `word` could be printed after the inner block containing `text: String` had ended.
- **Why it's wrong:** `word` is a borrowed `&str` pointing into `text`; leaving the inner block drops `text` and invalidates that data. Allowing the later print would create a dangling reference.
- **The rule:** A borrowed slice cannot outlive the value it references. Use the slice before its owner is dropped, or move the owner into a scope that lasts through the slice's final use.
- **Status:** 🟥 fresh

### 2026-08-25 - Move direction reversed (ownership drill d01)
- **What I wrote:** "move the ownership from t to s"
- **Why it's wrong:** In `let t = s`, the value starts in `s`; the assignment transfers that owned `String` into `t`, not the other way around. After the move, `t` is valid and `s` is invalid.
- **The rule:** For a non-`Copy` value, `let destination = source` moves ownership from `source` to `destination`. Using `&source` creates a borrow instead and leaves the source as owner.
- **Status:** 🟥 fresh

### 2026-08-25 - Clone confused with Copy and borrowing (ownership drill d02)
- **What I wrote:** "Copy type mean it has the ownership" and used `let b = a.clone()` to make the `String` test pass.
- **Why it's wrong:** Ownership and `Copy` are different concepts: every value has an owner, but only some types are implicitly duplicated by assignment. Cloning a `String` explicitly duplicates its heap data and was unnecessary when the test only needed shared read access.
- **The rule:** `Copy` makes ordinary assignment leave the source usable; a move transfers a non-`Copy` value; `&value` borrows without ownership transfer; `.clone()` is reserved for a genuinely required independent owned copy.
- **Status:** 🟥 fresh

### 2026-08-25 - Borrow lifetime confused with lexical scope (ownership drill d04)
- **What I wrote:** "when it return new value" and then "when it out of scope right"
- **Why it's wrong:** A borrow does not depend on returning a value, and it need not remain active for the reference variable's entire lexical scope. Rust can end the borrow after the reference's final use, even though the name remains in scope.
- **The rule:** Going out of scope always ends a borrow, but non-lexical lifetime analysis may end it earlier at the reference's last use, allowing a later non-overlapping borrow.
- **Status:** 🟥 fresh
