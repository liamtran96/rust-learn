fn main() {
    let mut scanner = Scanner {
        source: "rust",
        cursor: 0,
    };
    println!("{:?}", scanner.peek());
    println!("{:?}", scanner.advance());
    println!("{:?}", scanner.peek());
}
#[test]
fn peek_does_not_move_cursor() {
    let scanner = Scanner {
        source: "rust",
        cursor: 0,
    };
    assert_eq!(scanner.peek(), Some('r'));
    assert_eq!(scanner.cursor, 0);
}
#[test]
fn advance_moves_cursor() {
    let mut scanner = Scanner {
        source: "rust",
        cursor: 0,
    };
    assert_eq!(scanner.advance(), Some('r'));
    assert_eq!(scanner.cursor, 1);
    assert_eq!(scanner.peek(), Some('u'));
}
#[test]
fn advance_at_end_returns_none() {
    let mut scanner = Scanner {
        source: "r",
        cursor: 0,
    };
    assert_eq!(scanner.advance(), Some('r'));
    assert_eq!(scanner.peek(), None);
    assert_eq!(scanner.advance(), None);
    assert_eq!(scanner.cursor, 1);
}

struct Scanner<'a> {
    source: &'a str,
    cursor: usize,
}

impl<'a> Scanner<'a> {
    fn peek(&self) -> Option<char> {
        let text = self.source;
        let pos = self.cursor;
        let remaining = &text[pos..];
        remaining.chars().next()
    }
    fn advance(&mut self) -> Option<char> {
        let letter = self.peek();
        match letter {
            Some(ch) => {
                self.cursor += ch.len_utf8();
                Some(ch)
            }
            None => None,
        }
    }
}
