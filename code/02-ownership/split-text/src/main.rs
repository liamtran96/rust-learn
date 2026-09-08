fn main() {
    let message = String::from("red💥blue💥green");
    let pieces = split_text(&message, '💥');
    println!("{pieces:?}");
}

fn split_text(text: &str, separator: char) -> Vec<&str> {
    let mut pieces = Vec::new();
    let mut start = 0;
    for (position, character) in text.char_indices() {
        if character == separator {
            let piece = &text[start..position];
            pieces.push(piece);
            start = position + character.len_utf8();
        }
    }
    let final_piece = &text[start..];
    pieces.push(final_piece);
    pieces
}

#[cfg(test)]
mod tests {
    use super::split_text;

    #[test]
    fn splits_on_a_character() {
        let result = split_text("red,blue", ',');

        assert_eq!(result, vec!["red", "blue"]);
    }
    #[test]
    fn preserves_empty_pieces() {
        let result = split_text(",red,,green,", ',');

        assert_eq!(result, vec!["", "red", "", "green", ""]);
    }
    #[test]
    fn handles_no_separator() {
        let result = split_text("red", ',');
        let empty_result = split_text("", ',');
        assert_eq!(result, vec!["red"]);
        assert_eq!(empty_result, vec![""])
    }
    #[test]
    fn handles_unicode_separator() {
        let result = split_text("red💥blue💥green", '💥');
        assert_eq!(result, vec!["red", "blue", "green"]);
    }
}
