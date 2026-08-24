//! Drill 10 — mutate through an exclusive borrow (implement)
//! The parameter is `&mut [i32]`, not `&mut Vec<i32>` — the caller can still pass a Vec.
//!
//! PREDICT (before running): the test passes `&mut v` where `v: Vec<i32>`. Why does that
//! satisfy a `&mut [i32]` parameter?
//! PREDICT:
//!
//! WHY (after it passes): why is `&mut [i32]` the better signature than `&mut Vec<i32>`?
//! WHY:

/// Doubles every element in place.
fn double_all(values: &mut [i32]) {
    todo!("mutate through the exclusive borrow")
}

#[test]
fn doubles_in_place() {
    let mut v = vec![1, 2, 3];
    double_all(&mut v);
    assert_eq!(v, [2, 4, 6]);
}

#[test]
fn empty_is_fine() {
    let mut v: Vec<i32> = vec![];
    double_all(&mut v);
    assert!(v.is_empty());
}
