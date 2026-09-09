fn main() {
    let mut values = vec![1, 1, 2, 1, 2, 2];
    dedup_in_place(&mut values);
    println!("{values:?}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_a_run_of_duplicates() {
        let mut values = vec![1, 1, 1, 2];
        dedup_in_place(&mut values);
        assert_eq!(values, [1, 2]);
    }
    #[test]
    fn empty_vector() {
        let mut values: Vec<i32> = vec![];
        dedup_in_place(&mut values);
        assert_eq!(values, []);
    }
    #[test]
    fn no_adjacent_duplicates() {
        let mut values = vec![1, 2, 1];
        dedup_in_place(&mut values);
        assert_eq!(values, [1, 2, 1]);
    }
    #[test]
    fn duplicates_at_both_edges() {
        let mut values = vec![1, 1, 2, 3, 3];
        dedup_in_place(&mut values);
        assert_eq!(values, [1, 2, 3]);
    }
    #[test]
    fn non_integer_type() {
        let mut values = vec!["red", "red", "blue"];
        dedup_in_place(&mut values);
        assert_eq!(values, ["red", "blue"]);
    }
}

fn dedup_in_place<T: PartialEq>(values: &mut Vec<T>) {
    let mut index = 1;
    while index < values.len() {
        if values[index] == values[index - 1] {
            values.remove(index);
        } else {
            index += 1;
        }
    }
}
