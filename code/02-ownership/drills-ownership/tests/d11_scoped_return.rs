//! Drill 11 — the trailing semicolon (fix-it)
//! In Rust the last *expression* of a block is its value. A `;` turns it into a
//! statement, and the block's value becomes `()`.
//!
//! PREDICT (before running): does this compile? what will the error say the function returns?
//! PREDICT:
//!
//! WHY (after it passes): expression vs statement — one sentence each.
//! WHY:

fn add_one(n: i32) -> i32 {
    n + 1;
}

#[test]
fn returns_the_sum() {
    assert_eq!(add_one(41), 42);
}
