//! Drill 12 — a struct that borrows (implement)
//! `Scanner<'a>` holds a `&'a str` and a byte cursor. `peek` looks, `advance` consumes.
//! ASCII input only for now — Unicode widths come in a later chapter.
//!
//! PREDICT (before running): `peek` takes `&self` but `advance` takes `&mut self`. Why must they differ?
//! PREDICT:
//!
//! WHY (after it passes): what does the `'a` on the struct tie together?
//! WHY:

struct Scanner<'a> {
    source: &'a str,
    pos: usize,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a str) -> Self {
        Scanner { source, pos: 0 }
    }

    /// Returns the next character without consuming it.
    fn peek(&self) -> Option<char> {
        todo!("look at the character at pos, if any")
    }

    /// Returns the next character and moves past it.
    fn advance(&mut self) -> Option<char> {
        todo!("return the character at pos and step the cursor")
    }
}

#[test]
fn peek_does_not_consume() {
    let s = Scanner::new("ab");
    assert_eq!(s.peek(), Some('a'));
    assert_eq!(s.peek(), Some('a'));
}

#[test]
fn advance_walks_the_input() {
    let mut s = Scanner::new("ab");
    assert_eq!(s.advance(), Some('a'));
    assert_eq!(s.peek(), Some('b'));
    assert_eq!(s.advance(), Some('b'));
    assert_eq!(s.advance(), None);
}

#[test]
fn empty_input() {
    let mut s = Scanner::new("");
    assert_eq!(s.peek(), None);
    assert_eq!(s.advance(), None);
}
