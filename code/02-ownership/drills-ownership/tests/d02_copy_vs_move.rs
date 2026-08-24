//! Drill 02 — Copy vs move (fix-it)
//! Two tests with the exact same shape. One compiles, one doesn't.
//!
//! PREDICT (before running): which test fails to compile, and what makes the types behave differently?
//! PREDICT:
//!
//! WHY (after it passes): what does it mean for a type to be `Copy`?
//! WHY:

#[test]
fn integers() {
    let a = 5;
    let b = a;
    assert_eq!(a + b, 10);
}

#[test]
fn strings() {
    let a = String::from("5");
    let b = a;
    assert_eq!(a, b);
}
