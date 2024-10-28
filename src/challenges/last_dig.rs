pub fn last_dig(a: i64, b: i64, c: i64) -> bool {
    (a * b).to_string().chars().last() == c.to_string().chars().last()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn true_test_1() {
        assert!(last_dig(1, 1, 1), "Failed with inputs: 1, 1, 1");
    }

    #[test]
    fn true_test_2() {
        assert!(last_dig(12, 15, 10), "Failed with inputs: 12, 15, 10");
    }

    #[test]
    fn true_test_3() {
        assert!(
            last_dig(15228, 9209, 72162),
            "Failed with inputs: 15228, 9209, 72162"
        );
    }

    #[test]
    fn false_test_1() {
        assert!(!last_dig(15, 1, 1), "Failed with inputs: 15, 1, 1");
    }

    #[test]
    fn false_test_2() {
        assert!(!last_dig(123, 15, 10), "Failed with inputs: 123, 15, 10");
    }

    #[test]
    fn false_test_3() {
        assert!(
            !last_dig(5213, 99219, 6165),
            "Failed with inputs: 5213, 99219, 6165"
        );
    }

    #[test]
    fn false_test_4() {
        assert!(
            !last_dig(1523, 513, 512),
            "Failed with inputs: 1523, 513, 512"
        );
    }

    #[test]
    fn negative_numbers_test_1() {
        assert!(!last_dig(-15, 1, 1), "Failed with inputs: -15, 1, 1");
    }

    #[test]
    fn negative_numbers_test_2() {
        assert!(!last_dig(123, -15, 10), "Failed with inputs: 123, -15, 10");
    }

    #[test]
    fn negative_numbers_test_3() {
        assert!(last_dig(-12, 15, -10), "Failed with inputs: -12, 15, -10");
    }

    #[test]
    fn negative_numbers_test_4() {
        assert!(
            last_dig(15228, -9209, -72162),
            "Failed with inputs: 15228, -9209, -72162"
        );
    }
}
