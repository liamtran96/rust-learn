fn main() {
    println!("{}", count_digits(0));
}
fn count_digits(mut number: i32) -> i32 {
    let mut count: i32 = 0;
    if number == 0 {
        return 1;
    }
    loop {
        if number == 0 {
            break count;
        } else {
            count += 1;
            number /= 10;
        }
    }
}
