//! Drill 04 — two exclusive borrows (fix-it)
//! Only one `&mut` to a value may be live at a time. Fix by reordering or scoping,
//! not by deleting an increment — the final assert must stay `12`.
//!
//! PREDICT (before running): does this compile? why / why not?
//! PREDICT:
//!
//! WHY (after it passes): when does a borrow's life end?
//! WHY:

#[test]
fn one_at_a_time() {
    let mut score = 10;
    let a = &mut score;
    let b = &mut score;
    *a += 1;
    *b += 1;
    assert_eq!(score, 12);
}
