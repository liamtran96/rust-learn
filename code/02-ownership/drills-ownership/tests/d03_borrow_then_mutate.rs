//! Drill 03 — shared borrow held across mutation (fix-it)
//! `push` may reallocate the vector's buffer. What would happen to `r` if it did?
//!
//! PREDICT (before running): does this compile? why / why not?
//! PREDICT:
//!
//! WHY (after it passes): why does the borrow checker reject the original, even though
//! this particular push probably wouldn't have moved the buffer?
//! WHY:

#[test]
fn borrow_survives_push() {
    let mut v = vec![1, 2, 3];
    let r = &v[0];
    v.push(4);
    assert_eq!(*r, 1);
    assert_eq!(v.len(), 4);
}
