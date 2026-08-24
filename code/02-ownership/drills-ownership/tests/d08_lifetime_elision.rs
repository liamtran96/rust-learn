//! Drill 08 — when elision isn't enough (fix-it)
//! With two reference parameters, the compiler can't guess which one the return
//! value borrows from. Say it explicitly with a lifetime parameter.
//!
//! PREDICT (before running): does this compile? why can't the compiler infer the lifetime here
//! when it could for `first_word(s: &str) -> &str`?
//! PREDICT:
//!
//! WHY (after it passes): in plain words, what promise does `<'a>` make to the caller?
//! WHY:

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

#[test]
fn picks_the_longer() {
    assert_eq!(longest("long", "hi"), "long");
    assert_eq!(longest("a", "bb"), "bb");
}
