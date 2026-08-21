use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("You should input the number");
        return;
    }

    let input = &args[1];
    let parse_result = input.parse::<u32>();
    let quantity: u32 = match parse_result {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid quantity");
            return;
        }
    };

    println!("Quantity: {quantity}");
}
