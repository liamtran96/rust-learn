//! Drill 05 — `for` consumes the collection (fix-it)
//! `for x in v` takes ownership of `v`. The vector must still be usable afterward.
//!
//! PREDICT (before running): does this compile? why / why not?
//! PREDICT:
//!
//! WHY (after it passes): what changed about the type of `x` in the loop body?
//! WHY:

#[test]
fn loop_then_reuse() {
    let v = vec![1, 2, 3];
    let mut sum = 0;
    for x in v {
        sum += x;
    }
    assert_eq!(sum, 6);
    assert_eq!(v.len(), 3);
}
