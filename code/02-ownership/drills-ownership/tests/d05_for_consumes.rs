//! Drill 05 — `for` consumes the collection (fix-it)
//! `for x in v` takes ownership of `v`. The vector must still be usable afterward.
//!
//! PREDICT (before running): does this compile? why / why not?
//! PREDICT: no it doesn't because the x take the ownership from v and then we use the v vector 
//!
//! WHY (after it passes): what changed about the type of `x` in the loop body?
//! WHY: I will add the & to v because v is not a copy type and i use & to reference to the v we use it temporary. Before borrowing x has type i32, after borrowing x has type &i32

#[test]
fn loop_then_reuse() {
    let v = vec![1, 2, 3];
    let mut sum = 0;
    for x in &v {
        sum += x;
    }
    assert_eq!(sum, 6);
    assert_eq!(v.len(), 3);
}
