fn digit_count(mut number: u32) -> u32 {
    let mut counter = 0;
    let total = loop {
        counter += 1;
        number /= 10;

        if number == 0 {
            break counter;
        }
    };

    total
}
