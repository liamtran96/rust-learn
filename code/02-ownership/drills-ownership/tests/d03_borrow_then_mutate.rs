//! Drill 03 — shared borrow held across mutation (fix-it)
//! `push` may reallocate the vector's buffer. What would happen to `r` if it did?
//!
//! PREDICT (before running): does this compile? why / why not?
//! PREDICT:not because we can not borrow and then mutate the v
//!
//! WHY (after it passes): why does the borrow checker reject the original, even though
//! this particular push probably wouldn't have moved the buffer?
//! WHY:  because r point into the vector's heap buffer, and push need to mutable access and may reallocate the buffer. The reallocation cound invalidate r. So rust rejects the overlapping borrow regradless of whether this particular run reallocate

#[test]
fn borrow_survives_push() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    let r = &v[0];

    assert_eq!(*r, 1);
    assert_eq!(v.len(), 4);
}
