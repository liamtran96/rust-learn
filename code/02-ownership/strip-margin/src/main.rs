fn main() {
    println!("Hello, world!");
    println!(
        "{}",
        strip_margin("  |hello\n    |world        asjdhajdh", '|')
    );
    let text: &str = "hello";
  let tail: &str = &text[1..];

  println!("{tail}"); // "ello"
}

fn strip_margin(s: &str, prefix: char) -> String {
    s.lines()
        .map(|line| {
            line.find(prefix)
                .map(|index| &line[index + prefix.len_utf8()..])
                .unwrap_or(line)
        })
        .collect::<Vec<_>>()
        .join("\n")
}
