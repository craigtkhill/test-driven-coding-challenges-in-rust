pub fn divisible_by_five(number: i32) -> bool {
    number % 5 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seven_is_not_divisible_by_five() {
        assert_eq!(divisible_by_five(7), false);
    }

    #[test]
    fn test_five_is_divisible_by_five() {
        assert_eq!(divisible_by_five(5), true);
    }

    #[test]
    fn test_fifteen_is_divisible_by_five() {
        assert_eq!(divisible_by_five(15), true);
    }

    #[test]
    fn test_thirtythree_is_not_divisible_by_five() {
        assert_eq!(divisible_by_five(33), false);
    }

    #[test]
    fn test_negative_eighteen_is_not_divisible_by_five() {
        assert_eq!(divisible_by_five(-18), false);
    }

    #[test]
    fn test_nine_nine_nine_is_not_divisible_by_five() {
        assert_eq!(divisible_by_five(999), false);
    }

    #[test]
    fn test_two_is_not_divisible_by_five() {
        assert_eq!(divisible_by_five(2), false);
    }
}
