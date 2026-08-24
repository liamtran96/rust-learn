//! Drill 06 — take `&str`, not `&String` (fix-it)
//! Fix the *signature* of `shout` so both callers compile. Don't touch the callers.
//!
//! PREDICT (before running): which test fails to compile, and why does the other one work?
//! PREDICT:
//!
//! WHY (after it passes): why does `&owned` still work after the signature change?
//! WHY:

fn shout(s: &String) -> String {
    s.to_uppercase()
}

#[test]
fn works_with_owned_string() {
    let owned = String::from("hi");
    assert_eq!(shout(&owned), "HI");
}

#[test]
fn works_with_literal() {
    assert_eq!(shout("hi"), "HI");
}
