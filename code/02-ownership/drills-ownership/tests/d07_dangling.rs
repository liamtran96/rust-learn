//! Drill 07 — returning a reference to a local (fix-it)
//! `s` is dropped when `greeting` returns. A reference to it can't outlive the function.
//! Fix the signature *and* body — the caller must receive something it owns or that lives long enough.
//!
//! PREDICT (before running): does this compile? what error do you expect?
//! PREDICT:
//!
//! WHY (after it passes): where does the returned value live now?
//! WHY:

fn greeting() -> &String {
    let s = String::from("hello");
    &s
}

#[test]
fn no_dangling_reference() {
    assert_eq!(greeting(), "hello");
}
