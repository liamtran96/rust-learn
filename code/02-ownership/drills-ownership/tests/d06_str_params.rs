//! Drill 06 — take `&str`, not `&String` (fix-it)
//! Fix the *signature* of `shout` so both callers compile. Don't touch the callers.
//!
//! PREDICT (before running): which test fails to compile, and why does the other one work?
//! PREDICT: the second one failed to compile because String is copy type and we create a new variable with String type and then borrow it, it work but on the other hand because the argument is a reference to String so the parameter should be a reference type too
//!
//! WHY (after it passes): why does `&owned` still work after the signature change?
//! WHY: because String will be automatically change to &str type 
//! REVIEW: `&owned` has type `&String`; because `String` implements `Deref<Target = str>`, deref coercion converts that reference to `&str` at the function call. `String` itself is not `Copy`.

fn shout(s: &str) -> String {
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
