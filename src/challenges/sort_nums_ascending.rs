pub fn sort_nums_ascending(nums: Vec<i32>) -> Vec<i32> {
    let mut sorted = nums.clone();
    sorted.sort();
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_sorting() {
        assert_eq!(
            sort_nums_ascending(vec![1, 2, 10, 50, 5]),
            vec![1, 2, 5, 10, 50],
            "Should sort simple positive numbers"
        );
    }

    #[test]
    fn test_mixed_numbers() {
        assert_eq!(
            sort_nums_ascending(vec![80, 29, 4, -95, -24, 85]),
            vec![-95, -24, 4, 29, 80, 85],
            "Should sort mixed positive and negative numbers"
        );
    }

    #[test]
    fn test_duplicates() {
        assert_eq!(
            sort_nums_ascending(vec![47, 51, -17, -16, 91, 47, -85, -8, -16, -27]),
            vec![-85, -27, -17, -16, -16, -8, 47, 47, 51, 91],
            "Should handle duplicate numbers correctly"
        );
    }

    #[test]
    fn test_mostly_negative() {
        assert_eq!(
            sort_nums_ascending(vec![-51, -73, 65, 69, -76, 74, -14]),
            vec![-76, -73, -51, -14, 65, 69, 74],
            "Should sort array with more negative numbers"
        );
    }

    #[test]
    fn test_all_positive() {
        assert_eq!(
            sort_nums_ascending(vec![45, 98, 35, 65, 97, 21, 33]),
            vec![21, 33, 35, 45, 65, 97, 98],
            "Should sort array with all positive numbers"
        );
    }

    #[test]
    fn test_all_negative() {
        assert_eq!(
            sort_nums_ascending(vec![-23, -69, -54, -2, -32]),
            vec![-69, -54, -32, -23, -2],
            "Should sort array with all negative numbers"
        );
    }

    #[test]
    fn test_small_array() {
        assert_eq!(
            sort_nums_ascending(vec![-21, -9, -96]),
            vec![-96, -21, -9],
            "Should sort small arrays correctly"
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(
            sort_nums_ascending(vec![0]),
            vec![0],
            "Should handle single-element arrays"
        );
    }
}
