//! Drill 09 — return a slice of the input (implement)
//! A `&str` return that borrows from the parameter — elision handles the lifetime here.
//!
//! PREDICT (before running): with `todo!()` in place, does this file compile? do the tests pass?
//! PREDICT:
//!
//! WHY (after it passes): why is returning `&str` here free (no allocation), and what
//! would returning `String` cost instead?
//! WHY:

/// Returns the first whitespace-separated word, or "" if there is none.
fn first_word(s: &str) -> &str {
    todo!("implement without allocating a String")
}

#[test]
fn plain_sentence() {
    assert_eq!(first_word("hello brave world"), "hello");
}

#[test]
fn leading_whitespace() {
    assert_eq!(first_word("   spaced out"), "spaced");
}

#[test]
fn empty_and_blank() {
    assert_eq!(first_word(""), "");
    assert_eq!(first_word("   "), "");
}
