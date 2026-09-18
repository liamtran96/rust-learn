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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn largest_value() {
        let number = [3, 8, 5];
        let best = largest(&number);
        assert_eq!(best, 8);
    }
    #[test]
    fn optional_returns_some_for_nonempty() {
        let number = [3, 8, 5];
        let best = largest_optional(&number);
        assert_eq!(best, Some(8));
    }
    #[test]
    fn optional_returns_none_for_empty() {
        let empty: [i32; 0] = [];
        let best = largest_optional(&empty);
        assert_eq!(best, None);
    }
    #[test]
    fn borrowed_returns_largest_string() {
        let animal = [
            String::from("cat"),
            String::from("zebra"),
            String::from("ant"),
        ];
        let best = largest_borrowed(&animal);
        assert_eq!(best, Some(&animal[1]));
    }
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
