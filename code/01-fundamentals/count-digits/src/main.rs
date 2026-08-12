fn main() {
    println!("result:{}", count_digits(1123000));
    println!("hello world!!");
    let mut x = 5;
    x = x + 1;
    println!("{x}");
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
            //Example with number = 123:
            //Loop 1:  123 / 10 = 12   → count = 1
            //Loop 2:   12 / 10 = 1    → count = 2
            //Loop 3:    1 / 10 = 0    → count = 3
            //Loop 4:    0 == 0        → break, return 3
        }
    }
}
