use std::collections::HashMap;

fn main() {
    let words: Vec<String> = vec![
        String::from("eat"),
        String::from("tea"),
        String::from("tan"),
    ];
    println!("words: {:?}", words);
    let mut boxes: HashMap<Vec<char>, Vec<String>> = HashMap::new();
    println!("boxes: {:?}", boxes);

    for word in words {
        let mut letters: Vec<char> = word.chars().collect();
        println!("letters: {:?}", letters);

        letters.sort();
        boxes.entry(letters).or_insert(Vec::new()).push(word);
    }
    let group: Vec<Vec<String>> = boxes.into_values().collect();
    println!("{:?}", group);
}
