use std::collections::HashMap;

fn main() {
    let numbers: Vec<i32> = vec![7];
    let (median, mode) = median_mode(numbers);
    println!("median:{median} mode:{mode}");
}
// Panics if `numbers` is empty: there is no median or mode of nothing
fn median_mode(mut numbers: Vec<i32>) -> (f64, i32) {
    numbers.sort();
    let length = numbers.len();
    let middle = length / 2;
    let median = if length.is_multiple_of(2) {
        (numbers[middle] as f64 + numbers[middle - 1] as f64) / 2_f64
    } else {
        numbers[middle] as f64
    };

    let mut counts: HashMap<i32, usize> = HashMap::new();
    for number in &numbers {
        *counts.entry(*number).or_insert(0) += 1;
    }
    let mut mode: i32 = 0;
    let mut best_count: usize = 0;

    for (key, value) in &counts {
        if *value > best_count {
            best_count = *value;
            mode = *key;
        }
    }
    (median, mode)
}
