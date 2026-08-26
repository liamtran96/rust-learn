//! Drill 09 — return a slice of the input (implement)
//! A `&str` return that borrows from the parameter — elision handles the lifetime here.
//!
//! PREDICT (before running): with `todo!()` in place, does this file compile? do the tests pass?
//! PREDICT: no 
//!
//! WHY (after it passes): why is returning `&str` here free (no allocation), and what
//! would returning `String` cost instead?
//! WHY: because if we use String we need to make a copy and allocate a new heap memory but when we use the &str we just reference the str and then remove it
//! REVIEW: `split_whitespace` yields `&str` slices borrowed from `s`, so returning one reuses the input's bytes with no allocation or copy; an owned `String` would allocate a heap buffer and copy the word's bytes. The slice is not removed and cannot outlive `s`.

/// Returns the first whitespace-separated word, or "" if there is none.
fn first_word(s: &str) -> &str {
    let mut words = s.split_whitespace();
    let first: Option<&str> = words.next();
    first.unwrap_or("")
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
