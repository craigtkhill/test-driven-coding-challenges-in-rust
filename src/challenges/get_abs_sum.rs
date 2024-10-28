pub fn get_abs_sum(numbers: &[i32]) -> i32 {
    // TODO: Implement this function to make the tests pass
    todo!() // This will cause the tests to fail until you implement it
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            get_abs_sum(&[2, -1, -3, 4, 8]),
            18,
            "Should sum absolute values correctly for mixed numbers"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            get_abs_sum(&[-1]),
            1,
            "Should handle single negative number"
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            get_abs_sum(&[-1, -3, -5, -4, -10, 0]),
            23,
            "Should handle negative numbers and zero"
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            get_abs_sum(&[8, 9, 10, 32, 101, -10]),
            170,
            "Should handle larger numbers"
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            get_abs_sum(&[500]),
            500,
            "Should handle single positive number"
        );
    }
}
