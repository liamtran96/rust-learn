//! Drill 11 — the trailing semicolon (fix-it)
//! In Rust the last *expression* of a block is its value. A `;` turns it into a
//! statement, and the block's value becomes `()`.
//!
//! PREDICT (before running): does this compile? what will the error say the function returns?
//! PREDICT: no because the trailling semicolon at the return is this case it returen unit type 
//!
//! WHY (after it passes): expression vs statement — one sentence each.
//! WHY: expression return value and statement is like decalare something
//! REVIEW: An expression evaluates to a value and, when placed last without `;`, becomes the block's value; a statement performs an action, and terminating an expression with `;` discards its value so a block with no tail expression evaluates to `()`.

fn add_one(n: i32) -> i32 {
    n + 1
}

#[test]
fn returns_the_sum() {
    assert_eq!(add_one(41), 42);
}
