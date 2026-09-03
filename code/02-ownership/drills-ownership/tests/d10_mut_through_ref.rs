//! Drill 10 — mutate through an exclusive borrow (implement)
//! The parameter is `&mut [i32]`, not `&mut Vec<i32>` — the caller can still pass a Vec.
//!
//! PREDICT (before running): the test passes `&mut v` where `v: Vec<i32>`. Why does that
//! satisfy a `&mut [i32]` parameter?
//! PREDICT: because rust will be automatically convert &mut Vec<i32> to &mut [i32]
//!
//! WHY (after it passes): why is `&mut [i32]` the better signature than `&mut Vec<i32>`?
//! WHY: because double_all does not need vector-specific operations and only need mytauble element access-not vector operations
//! REVIEW: `&mut [i32]` grants exactly the exclusive element access `double_all` needs and accepts vectors, arrays, or subslices; `&mut Vec<i32>` would unnecessarily require a vector and expose operations such as `push`. `&mut Vec<i32>` coerces to `&mut [i32]` because `Vec<T>` dereferences mutably to `[T]`.

/// Doubles every element in place.
fn double_all(values: &mut [i32]) {
    for value in values.iter_mut() {
        *value *= 2;
    }
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
