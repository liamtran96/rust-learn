//! Drill 08 — when elision isn't enough (fix-it)
//! With two reference parameters, the compiler can't guess which one the return
//! value borrows from. Say it explicitly with a lifetime parameter.
//!
//! PREDICT (before running): does this compile? why can't the compiler infer the lifetime here
//! when it could for `first_word(s: &str) -> &str`?
//! PREDICT: i can not compile and it could for when just have one parameter
//!
//! WHY (after it passes): in plain words, what promise does `<'a>` make to the caller?
//! WHY: create a name for lifetime relationship and keep those parameter valid during run time  
//! REVIEW: `'a` names a relationship rather than extending either input's life: both inputs must be valid for `'a`, and the returned reference may be used only for that shared period, so it cannot outlive the shorter borrow.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

#[test]
fn picks_the_longer() {
    assert_eq!(longest("long", "hi"), "long");
    assert_eq!(longest("a", "bb"), "bb");
}
