//! Drill 01 — moves (fix-it)
//! A `String` owns its heap buffer. What happens when you assign it to a second variable?
//!
//! PREDICT (before running): does this compile? why / why not?
//! PREDICT:
//!
//! WHY (after it passes): what rule did the fix rely on?
//! WHY:

#[test]
fn moved_value() {
    let s = String::from("hi");
    let t = s;
    // Fix minimally so both values print. No .clone() unless you justify it in WHY.
    assert_eq!(format!("{s} {t}"), "hi hi");
}
