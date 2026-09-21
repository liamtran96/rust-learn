fn main() {
    let pair = Pair {
        first: 2,
        second: "liam",
    };
    let result = pair.swap();
    println!("first: {}, second: {}", result.first, result.second);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn swap_to_value() {
        let pair = Pair {
            first: 1,
            second: "liam",
        };
        let result = pair.swap();
        assert_eq!(result.first, "liam");
        assert_eq!(result.second, 1);
    }
}

struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn swap(self) -> Pair<U, T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}
