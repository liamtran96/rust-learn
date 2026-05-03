use std::io;


fn main() {
    // println!("Enter a temperature in clesius: ");
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("Failed to read line");
    // println!("You typed:{input}");

    // let celcius: f64 = input.trim().parse().expect("Please type a number");
    // println!("Parse Celsius is: {celcius}");
    // let fahrenheit = celsius_to_fahrenheit(celcius);
    //  println!("{celcius:.2} C = {fahrenheit:.2} F")
    println!("Conver C to F");
    let mut unit = String::new();
    io::stdin().read_line(&mut unit).expect("Failed to read line");
    let unit = unit.trim().to_uppercase();

    println!("Enter a temperature: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let temp: f64 = input.trim().parse().expect("Please type a number");
    match unit.as_str() {
        "C" => println!("{temp:.2} C = {:.2} F",c_to_f(temp)),
        "F" => println!("{temp:.2} F = {:.2} C",f_to_c(temp)),
        _ => println!("Invalid unit"),
    }
}

fn c_to_f(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}
fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}
