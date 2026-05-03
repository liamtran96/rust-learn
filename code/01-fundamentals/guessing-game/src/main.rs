use rand::RngExt;

fn main() {
    // let t: (i32, f64, char) = (42, 3.14, 'z');
    // let (a, b, c) = t;         // destructuring
    // let first = t.0; 
    // println!("a: {}, b: {}, c: {}, first: {}", a, b, c, first); 
    // const GUESS_NUMBER: i32 = 42;
    let secret_number = rand::rng().random_range(1..=100);
    let mut guess = String::new();
    println!("You guessed: {}", guess);
    let _result = loop {
        println!("Please input your guess.");
        guess.clear();
        std::io::stdin().read_line(&mut guess).expect("Failed to read line");
        match guess.trim().parse::<i32>(){
            Ok(guess) => {
                if guess < secret_number {
                    println!("Too small!")
                }
                else if guess > secret_number{
                    println!("Too big!")
                }
                else{
                    println!("You win!");
                    break "Congratulations!";
                }
            }
            Err(_) => println!("Please input a number!"),
        }

    };
}
