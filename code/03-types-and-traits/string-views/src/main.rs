fn main() {
    let a = String::from("hello");
    let b: &str = &a;
    println!("Hello, world!{}", { &a });
    println!("Hello, world!{}", { b });
}
