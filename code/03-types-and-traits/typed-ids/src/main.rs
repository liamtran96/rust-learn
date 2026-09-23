fn main() {
    let user_id = UserId(7);
    let order_id = OrderId(9);
    type_id_check(user_id);
    println!("{}", order_id.0);
}
struct UserId(u64);
struct OrderId(u64);

fn type_id_check(id: UserId) {
    println!("{}", id.0);
}
