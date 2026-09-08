fn main() {
    let message = String::from("red,blue");

    show_text(&message);

    println!("After function: {message}");
}

// fn split_text(text: &str, separator: char) -> Vec<&str> {

// }
fn show_text(text: &str) {
    println!("Inside function:  {text}");
}
