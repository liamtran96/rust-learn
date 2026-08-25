//! Drill 01 — moves (fix-it)
//! A `String` owns its heap buffer. What happens when you assign it to a second variable?
//! cause the error
//! PREDICT (before running): does this compile? why / why not?
//! PREDICT: no it does not compile because when we assign t for s without & the s value lost we can not print it
//!
//! WHY (after it passes): what rule did the fix rely on?
//! WHY:  yahh the first one move the ownership from s to t and the second one using & it shares the reference t borrow the String withot taking the ownership

#[test]
fn moved_value() {
    let s = String::from("hi");
    let t = &s;
    // Fix minimally so both values print. No .clone() unless you justify it in WHY.
    assert_eq!(format!("{s} {t}"), "hi hi");
}
