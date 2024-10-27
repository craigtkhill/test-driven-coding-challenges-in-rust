pub fn less_than_or_equal_to_zero(num: i32) -> bool {
    num <= 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            less_than_or_equal_to_zero(5),
            false,
            "Should return false for 5"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            less_than_or_equal_to_zero(0),
            true,
            "Should return true for 0"
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            less_than_or_equal_to_zero(-5),
            true,
            "Should return true for -5"
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            less_than_or_equal_to_zero(1),
            false,
            "Should return false for 1"
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            less_than_or_equal_to_zero(2),
            false,
            "Should return false for 2"
        );
    }

    #[test]
    fn test6() {
        assert_eq!(
            less_than_or_equal_to_zero(10000),
            false,
            "Should return false for 10000"
        );
    }

    #[test]
    fn test7() {
        assert_eq!(
            less_than_or_equal_to_zero(1),
            false,
            "Should return false for 1"
        );
    }
}
