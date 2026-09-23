fn main() {
    let point = Point { x: 2, y: 2 };
    println!("{point:?}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    fn requires_clone<T: Clone>() {}
    fn requires_equal<T: Eq>() {}

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
    #[test]
    fn point_is_copy() {
        let original = Point { x: 2, y: 3 };
        let copied = original;
        assert_eq!(original, copied);
    }
    #[test]
    fn point_implements_clone() {
        requires_clone::<Point>();
    }

    #[test]
    fn point_implements_eq() {
        requires_equal::<Point>();
    }

    #[test]
    fn point_works_in_hash_set() {
        let point = Point { x: 2, y: 3 };
        let mut values = HashSet::new();
        values.insert(point);
        assert!(values.contains(&point));
    }
}
