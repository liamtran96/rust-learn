fn main() {
    println!("Hello, world!");
    let number = [3, 8, 5];
    let result = largest(&number);
    let animal = [
        String::from("cat"),
        String::from("zebra"),
        String::from("ant"),
    ];
    let borrow_result = largest_borrowed(&animal);
    let optional_result = largest_optional(&number);
    println!("{result}");
    println!("{optional_result:?}");
    println!("{borrow_result:?}");
    println!("{animal:?}");

    println!("{number:?}");
    let empty: [i32; 0] = [];
    let empty_result = largest_optional(&empty);
    println!("{empty_result:?}");
}

fn largest<T: PartialOrd + Copy>(xs: &[T]) -> T {
    let mut best = &xs[0];
    for x in &xs[1..] {
        if x > best {
            best = x;
        }
    }
    *best
}
fn largest_optional<T: PartialOrd + Copy>(xs: &[T]) -> Option<T> {
    match xs.first() {
        None => None,
        Some(_) => Some(largest(xs)),
    }
}

fn largest_borrowed<T: PartialOrd>(xs: &[T]) -> Option<&T> {
    match xs.first() {
        None => None,
        Some(first) => {
            let mut best = first;
            for item in xs {
                if item > best {
                    best = item;
                }
            }
            Some(best)
        }
    }
}
