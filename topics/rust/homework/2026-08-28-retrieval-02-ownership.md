# Rust Retrieval Homework - 2026-08-28

**Focus:** ownership, borrowing, slices, and lifetimes
**Based on:** completed material through Phase 2, Week 3 (ownership drill d12, `strip_margin`, and `split_at_mut`)
**Timebox:** 30 minutes

## Instructions

Answer from memory first. For code questions, predict before running anything. Explain
the Rust rule behind each answer. Write answers directly below each prompt.

## Questions

### 1. Move, copy, borrow, or clone?

Explain the difference between moving, copying, borrowing, and cloning a value. For each
line below, state which operation occurs, who owns the value afterward, and whether the
source binding remains usable.

```rust
let count = 8_i32;
let other_count = count;

let title = String::from("Ownership");
let reader = &title;

let archived = title.clone();
let published = title;
```

Also explain why using `clone()` merely to make a borrow-checker error disappear can hide
the real ownership decision.

**Your answer (attempted 2026-09-03):**

1. moving is move the ownership from one to another
2. copying is copy to the new value
3. borrowing: borrow the reference
4. cloning: clone the value and the reference

**Follow-up attempt:**

1. let archived = title.clone();
2. let reader = &title;
3. no because it was moved the ownership and because the String is non-copy type so it is usable

**Revision (2026-09-04):** moving: move the ownership from one to another and for the
non-copy type the original one can useable

**Revision 2 (2026-09-04):** dont compile

**Revision 3 (2026-09-04):** because the ownership of title was moved to published and
String does not implement borrowing

**Revision 4 (2026-09-04):**

2. copy is value dupliacted automatically both can be usable
3. borrowing: borrow the reference both can usable
4. clone is explicit clne creates new resouce

**Revision 4 review:** Partly correct
**Revision 5 (2026-09-04):** give it &title because clone create a new resouce but we just
want to read the title

**Review status:** Correct
**Review:** Borrowing gives read access without transferring ownership or allocating a
second String. Cloning is appropriate only when an independent owned value is required.

**Reference answer:**

- **Move:** Assigning title to published transfers ownership of the original String to
  published. Because String is not Copy, title is no longer usable afterward.
- **Copy:** Assigning count to other_count copies the i32 automatically. Both variables
  remain usable and independently contain 8.
- **Borrow:** Assigning &title to reader creates a shared reference. title remains the
  owner, reader can read the value, and no second String is allocated. The reference
  cannot outlive the borrowed value.
  In this snippet reader is not used again, so the shared borrow ends before title moves.
  If reader were used afterward, Rust would reject moving title while it is borrowed.
- **Clone:** Calling title.clone() explicitly creates a separate owned String. Both values
  remain usable until title is later moved to published.
- Using clone() only to silence the borrow checker can hide whether code should borrow or
  take ownership, and it can add an unnecessary allocation and data copy. Use &title when
  only temporary read access is required; clone only when independent ownership is needed.

### 2. What a lifetime annotation actually says

In your own words, explain what `'a` means in this type and what relationship Rust checks:

```rust
struct Cursor<'a> {
    source: &'a str,
    pos: usize,
}
```

Does `'a` create a lifetime, keep `source` alive, or change anything at runtime? Explain
why Rust must reject using a `Cursor` after the text referenced by `source` has been
dropped.

**Your answer (attempted 2026-09-04):**

'a create a lifetime to tell Rust that this struct should be alive during runtime
Rust reject using a Cursor after the text referenced by source because it will be dropped

**Revision (2026-09-04):** 'a does not create a lifetime it names the relatonship between
Cursor and its borrowed reference ensuring the Cursor can not be used after the source is dropped

**Revision 2 (2026-09-04):** 'a is check at runtime. It has not reference effect at
reumtime, and it does not keep the source alive

**Revision 2 review:** Partly correct - no runtime effect and no lifetime extension are
correct; Rust checks the lifetime relationship at compile time, not runtime.
**Revision 3 (2026-09-04):** 'a is checked at complie time. It has no effect at runtime,
and it does not keep the source alive

**Review status:** Correct
**Review:** The answer now identifies the compile-time relationship, rejects runtime
effects, and explains why Cursor cannot safely outlive its borrowed source.

**Reference answer:**

- 'a is a generic lifetime parameter that names a validity relationship; it does not
  create a fixed duration or make a value live longer.
- In Cursor<'a>, the field source: &'a str says that the stored string slice must remain
  valid for the lifetime represented by 'a.
- Rust therefore permits a Cursor to be used only while its referenced source text is
  still valid.
- If the source owner were dropped first, source would become a dangling reference.
  Rust rejects that situation at compile time rather than allowing unsafe runtime access.
- Lifetime annotations do not allocate, keep the source alive, or change runtime behavior;
  they describe constraints checked by the borrow checker.

### 3. Trace two kinds of iteration

Without running the code, analyze Version A and Version B separately. For each version:

- state the type of `item` inside the loop;
- state whether the loop consumes or borrows `values`;
- decide whether the final `println!` compiles;
- explain why the fact that `i32` is `Copy` does or does not decide what happens to the
  vector binding.

```rust
// Version A
let values = vec![3, 6, 9];
for item in values {
    println!("{item}");
}
println!("{}", values.len());
```

```rust
// Version B
let values = vec![3, 6, 9];
for item in &values {
    println!("{item}");
}
println!("{}", values.len());
```

**Your answer (attempted 2026-09-04):**

**Version A**
1. state the type of item inside the loop: unit
2. state whether the loop consumes or borrors values: consume value
3. decide whether the final println! compile: error
4. because of the interation

**Version B**
1. state the type of item inside the loop: the value with i32 type
2. state whether the loop consumes or borrors values: borrow the value
3. decide whether the final println! compile: each item in the vector
4. because of the interation

**Revision (2026-09-04):**

- Version A moves each element out of Vec<i32>; item has type i32.
- Version B borrows each element through &values; item has type &i32.

**Revision review:** Correct item types
**Revision 2 (2026-09-04):** Yes, it prints 3 because Vec<i32> is a copy type so we can
reuse it.

**Revision 2 review:** Partly correct - the output and compilation result are correct,
but Vec<i32> is not Copy; the vector remains usable because the loop borrows it.
**Revision 3 (2026-09-04):** values remains usable becasue the loop interates over i32
creating shared reference instead of moving the ownership.

**Revision 3 review:** Retry - the borrowing idea is right, but i32 is the yielded element
type; the loop expression being iterated is &values.
**Revision 4 (2026-09-04):** the loop iterate over &value, so it borrow the Vec instead of
consuming it. i32: Copy applies to the reference, not to the Vec container.

**Revision 4 review:** Partly correct - the loop borrows &values, so the Vec is not
consumed. The Copy statement applies to the i32 element type, not to the reference or Vec.

**Revision 5 (2026-09-04):** the loop iterates over &values, so it borrows the Vec instead
of consuming it. i32: Copy means each interger element is copyable. It does not maje the
containing Vec<i32> copyable.

**Review status:** Correct
**Review:** Both iterator item types, the ownership effect of each loop, the final-use
results, and the distinction between Copy elements and a non-Copy Vec are now correct.

**Reference answer:**

**Version A**

- item has type i32 because the owned-vector iterator yields elements by value.
- The loop iterates over values by value, so IntoIterator consumes the Vec and moves each
  element out through the iterator.
- The final println does not compile because values was moved into the loop and consumed.
- The fact that i32 implements Copy applies to each element. It does not make Vec<i32>
  implement Copy or preserve the vector binding after owned iteration.

**Version B**

- item has type &i32 because the borrowed-vector iterator yields shared references.
- The loop iterates over &values, so it borrows the Vec instead of transferring ownership.
  The shared borrow ends after the loop.
- The final println compiles because values still owns the vector, and values.len() prints
  3.
- The Copy property of i32 is not why values survives. The Vec remains usable because it
  was borrowed rather than consumed.


### 4. Diagnose the overlapping borrows

Predict whether this compiles. If it does not, identify the two operations whose borrows
conflict and explain why `Vec::push` matters even though it only appends one element.

```rust
let mut scores = vec![10, 20, 30];
let first = &scores[0];

scores.push(40);
println!("first={first}");
```

Then describe a minimal reordering that preserves all four values and makes the borrows
non-overlapping. Name the point where the shared borrow ends under non-lexical lifetime
analysis.

**Your answer (attempted 2026-09-04):**

it does not compile because first was changes

**Review status:** Retry - the compile result is correct, but first is not changed. The
answer still needs to identify the shared borrow and mutable borrow that overlap, why
push matters, a safe reordering, and the shared borrow's final use.

**Your answer (second attempt, 2026-09-04):**

scores.push(40) modify the original one

**Review status:** Retry - this correctly recognizes that push modifies scores, meaning
it requires mutable access. The answer still needs to explain why that mutable borrow
conflicts with the shared reference first while first remains needed later.

**Your answer (third attempt, 2026-09-04):**

it does not compile because first shared borrow, while scroes.push(40) need a mutable borrow

**Review status:** Partly correct - this correctly identifies the overlapping shared and
mutable borrows. The answer still needs to explain why push can affect an existing
element reference, give a safe reordering, and identify the shared borrow's final use.

**Your answer (fourth attempt, 2026-09-04):**

if the vector's current memory is full can not push. i dont know

**Review status:** Retry - a full Vec can still accept push. It may allocate a larger
buffer and move its elements there, which would make an existing reference into the old
buffer invalid. The answer still needs a safe reordering and the shared borrow's final use.

**Your answer (fifth attempt, 2026-09-04):**

push may allocate the vector's buffer, so first could point to new memory and the old will be released

**Review status:** Partly correct - push may allocate a new buffer and release the old
one, but first would still point to the old memory, not automatically follow the elements
to the new memory. The answer still needs a safe reordering and the final shared use.

**Your answer (sixth attempt, 2026-09-04):**

invalid

**Review status:** Partly correct - first could become an invalid dangling reference to
the released old buffer. The borrow conflict and reallocation explanation are now correct;
the answer still needs a safe reordering and the shared borrow's final use under NLL.

**Your answer (seventh attempt, 2026-09-04):**

before

**Review status:** Partly correct - moving push before the creation of first safely makes
the mutable borrow finish before the shared borrow begins. Only the point where the shared
borrow ends under NLL remains to complete the question.

### 5. Write a disjoint-slice transformation

Write the body of this function. It must split `values` at `mid`, add `10` to every value
in the left slice, and add `100` to every value in the right slice.

```rust
fn adjust_halves(values: &mut [i32], mid: usize) {
    // Your implementation
    let (left, right) = values.split_at_mut(mid);
    for value in left.iter_mut() {
          *value += 10;
      }

      for value in right.iter_mut() {
          *value += 100;
      }
}
```

Requirements:

- use the standard safe operation that returns two disjoint mutable slices;
- mutate through the references instead of merely calculating and discarding results;
- do not clone or allocate another collection;
- explain what the two slices contain when `mid` is `0`, when it equals `values.len()`,
  and what happens when `mid` is greater than `values.len()`.

**Your answer:**

**Edge-case attempt (2026-09-04):**

1. mid = 0 then left is [] and right is values
2. mid == values.len() then left is values and right is []
3. mid > values.len() then left and right is []

**Review status:** Partly correct - the first two cases are correct. When mid is greater
than the slice length, split_at_mut panics instead of returning two empty slices. The
function body still needs a brief explanation of its two disjoint mutable slices.

**Your answer (correction, 2026-09-04):**

mid > values.len(): split_at_mut panics. And the index of [] start with 0 or 1?

**Review status:** Correct.

**Reference answer:**

```rust
fn adjust_halves(values: &mut [i32], mid: usize) {
    let (left, right) = values.split_at_mut(mid);

    for value in left.iter_mut() {
        *value += 10;
    }

    for value in right.iter_mut() {
        *value += 100;
    }
}
```

split_at_mut returns two disjoint mutable slices without copying or allocating. The left
slice contains the first mid elements and the right slice contains the remaining elements.
When mid is 0, left is empty and right contains all values. When mid equals values.len(),
left contains all values and right is empty. When mid is greater than values.len(), the
method panics.

### 6. Transfer: a borrowed command parser

A command-line application receives an owned `String` such as `"deploy production"`.
It needs a function that returns only the first whitespace-separated word without
allocating a new string.

Describe an appropriate function signature using `&str`, including the return type. Then
explain:

- who owns the original text;
- what the returned value contains and whether it owns or copies the word;
- why the returned value cannot be stored and used after the original `String` is dropped;
- what ownership and allocation tradeoff would change if the function returned `String`
  instead.

You do not need to implement the function.

**Your answer:**

**Your answer (first attempt, 2026-09-04):**

the function should return &str

**Review status:** Partly correct - &str is the correct borrowed return type and avoids a
new allocation. The answer still needs the complete signature and the ownership,
validity, and String-return tradeoff explanations.

## Confidence check

For each question, rate confidence from 1 (guessing) to 5 (certain) before checking notes.
