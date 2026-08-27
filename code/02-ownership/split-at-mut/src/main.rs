fn main() {
    let mut number = [10, 20, 30, 40];
    let (left, right) = split_at_mut(&mut number, 2);
    println!("left: {left:?}");
    println!("right: {right:?}");
    left[0] = 100;
    right[0] = 300;

    println!("original: {number:?}");
}

fn split_at_mut<T>(v: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    v.split_at_mut(mid)
}
