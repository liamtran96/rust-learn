//! Drill 07 — returning a reference to a local (fix-it)
//! `s` is dropped when `greeting` returns. A reference to it can't outlive the function.
//! Fix the signature *and* body — the caller must receive something it owns or that lives long enough.
//!
//! PREDICT (before running): does this compile? what error do you expect?
//! PREDICT: no it doesnt the error i expect is the local s value will be destroyed when the greeting end
//!
//! WHY (after it passes): where does the returned value live now?
//! WHY:in the heap
//! REVIEW: Ownership of the returned `String` moves to the caller; the caller owns the `String` value, while its text buffer remains on the heap until that returned owner is dropped.

fn greeting() -> String {
    let s = String::from("hello");
    s
}

#[test]
fn no_dangling_reference() {
    assert_eq!(greeting(), "hello");
}
