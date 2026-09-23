fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn debug_formats_point() {
        let point = Point { x: 2, y: 3 };
        let output = format!("{point:?}");
        assert_eq!(output, "Point { x: 2, y: 3 }");
    }
    #[test]
    fn points_compare_by_coordinates() {
        let first = Point { x: 2, y: 3 };
        let same = Point { x: 2, y: 3 };
        let different = Point { x: 2, y: 4 };
        assert_eq!(first, same);
        assert_ne!(first, different);
    }
}
