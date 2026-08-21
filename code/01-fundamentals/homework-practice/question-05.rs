fn shipping_band(weight: u32) -> &'static str {
    if weight < 5 {
        "light"
    } else if 5 <= weight && weight <= 20 {
        "standard"
    } else {
        "heavy"
    }
}
