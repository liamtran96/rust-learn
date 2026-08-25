//! Drill 02 — Copy vs move (fix-it)
//! Two tests with the exact same shape. One compiles, one doesn't.
//!
//! PREDICT (before running): which test fails to compile, and what makes the types behave differently?
//! PREDICT:the firs test cam complie but the second one can not
//! the fiest one has type is number like it is a traditional type we can assign it to new variable but for the second one type is String copy type it has reference I mean the ownership
//! WHY (after it passes): what does it mean for a type to be `Copy`?
//! WHY: Copy type mean we copy a value to the new one and it has new ownershp
// because the value is copied during assigment the original variable remains usable

#[test]
fn integers() {
    let a = 5;
    let b = a;
    assert_eq!(a + b, 10);
}

#[test]
fn strings() {
    let a = String::from("hi");
    let b = &a;
    assert_eq!(&a, b);
}
