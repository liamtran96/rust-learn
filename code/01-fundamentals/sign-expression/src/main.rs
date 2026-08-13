fn main() {
    println!("{}", sign(-2));
}

fn sign(n: i32) -> &'static str {
    if n < 0 {
        "negative"
    } else if n > 0 {
        "positive"
    } else {
        "zero"
    }
}
// Ngăn chương trình return một giá trị tham chiếu mà đã bị xóa bỏ khỏi chương trình trước đó
