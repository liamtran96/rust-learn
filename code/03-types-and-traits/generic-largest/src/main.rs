fn main() {
    println!("Hello, world!");
}

fn largest<T: PartialOrd + Copy>(xs: &[T]) -> &T{
let mut best = &xs[0];
    for x in &xs[1..] {
        if x > best { best = x; }
    }
    best
}