# Split a Mutable Slice

## What you are building

Write a function that divides one mutable slice at an index and returns two
mutable slices. The returned slices must cover different parts of the input,
so Rust can safely allow both mutable borrows to exist at the same time.

Changing an element through either returned slice changes the original
collection.

## Expected input and output

This exercise uses function arguments and a return value. It does not read
keyboard input, and it does not require a particular printed format.

- Input: a mutable slice containing `[10, 20, 30, 40]` and `mid` set to `2`.
- Output: a tuple whose left slice is `[10, 20]` and whose right slice is
  `[30, 40]`.

The returned slices borrow the original data; they are not copied collections.

## Function shape

```rust
fn split_at_mut<T>(v: &mut [T], mid: usize) -> (&mut [T], &mut [T])
```

Read it one piece at a time:

- `fn` starts a function declaration.
- `split_at_mut` is the function name.
- `<T>` introduces a generic type named `T`; the function should work with
  slices of integers, strings, or any other element type.
- `v:` names the first parameter and separates its name from its type.
- `&mut [T]` means an exclusive, mutable borrow of a slice of `T` values.
  `[T]` is the slice data; `&mut` is the mutable reference to it.
- `mid: usize` is the index where the slice is divided. Slice indexes use
  `usize`.
- `->` introduces the return type.
- `(&mut [T], &mut [T])` is a tuple containing two mutable slice references.
  Both borrow from `v`, but they must not overlap.

Rust can infer that the returned references live no longer than the borrowed
input, so this signature does not need a written lifetime such as `'a`.

## Work in small steps

1. Put the function signature in `src/main.rs` and give it a temporary
   placeholder body so the file can compile while you set up an example.
2. In `main`, make a mutable array or vector, borrow it as a mutable slice,
   and choose a middle index.
3. Try forming the two slice ranges yourself. If the borrow checker rejects
   it, stop and read the complete error: identify which first mutable borrow
   Rust believes is still active when the second one begins.
4. Once you understand that error, look at the standard slice operation whose
   name matches this exercise. Use its documentation to finish safely.
5. Check a middle split, `mid == 0`, and `mid == v.len()`. Also observe and
   explain what happens when `mid` is greater than the length.

## Concepts to notice

- A mutable reference is exclusive: no other reference may access the same
  region while that mutable borrow is active.
- Two mutable references are allowed when Rust can prove their regions do not
  overlap.
- Returning a tuple lets the caller receive both parts at once.
- No cloning is needed; both outputs are views into the original data.

## Likely pitfall

Two ranges may look non-overlapping to you, but separate indexing expressions
do not necessarily prove that fact to the borrow checker. Do not use `clone()`
to bypass the lesson. Read the diagnostic, then use the safe slice operation
designed to express this guarantee.

## Notes

- [[topics/rust/02-ownership/index]]
- [[topics/rust/02-ownership/borrowing]]
- [[topics/rust/02-ownership/slices]]
