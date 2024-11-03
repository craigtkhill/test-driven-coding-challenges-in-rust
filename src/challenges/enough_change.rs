pub fn change_enough(_coins: [u32; 4], _price: f64) -> bool {
    let quarter = _coins[0] as f64 * 0.25 as f64;
    let dime = _coins[1] as f64 * 0.10;
    let nickel = _coins[2] as f64 * 0.5;
    let penny = _coins[3] as f64 * 0.01;
    (quarter + dime + nickel + penny) >= _price
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enough_change_test_1() {
        assert!(
            change_enough([0, 0, 20, 5], 0.75),
            "Should have enough change with 20 nickels and 5 pennies"
        );
    }

    #[test]
    fn enough_change_test_2() {
        assert!(
            change_enough([30, 40, 20, 5], 12.55),
            "Should have enough change with mixed coins"
        );
    }

    #[test]
    fn enough_change_test_3() {
        assert!(
            change_enough([1, 0, 2555, 219], 127.75),
            "Should have enough change with large number of nickels"
        );
    }

    #[test]
    fn enough_change_test_4() {
        assert!(
            change_enough([1, 335, 0, 219], 35.21),
            "Should have enough change with many dimes"
        );
    }

    #[test]
    fn not_enough_change_test_1() {
        assert!(
            !change_enough([2, 100, 0, 0], 14.11),
            "Should not have enough change with quarters and dimes only"
        );
    }

    #[test]
    fn not_enough_change_test_2() {
        assert!(
            !change_enough([10, 0, 0, 50], 13.85),
            "Should not have enough change with quarters and pennies only"
        );
    }

    #[test]
    fn not_enough_change_test_3() {
        assert!(
            !change_enough([1, 0, 5, 219], 19.99),
            "Should not have enough change with given combination"
        );
    }
}
