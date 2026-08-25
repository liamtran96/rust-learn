//! Drill 04 — two exclusive borrows (fix-it)
//! Only one `&mut` to a value may be live at a time. Fix by reordering or scoping,
//! not by deleting an increment — the final assert must stay `12`.
//!
//! PREDICT (before running): does this compile? why / why not?
//! PREDICT:no because we can not mutate 2 value at the same time
//!
//! WHY (after it passes): when does a borrow's life end?
//! WHY: when the reference's last use

#[test]
fn one_at_a_time() {
    let mut score = 10;
    let a = &mut score;
    *a += 1;

    let b = &mut score;
    *b += 1;
    assert_eq!(score, 12);
}
