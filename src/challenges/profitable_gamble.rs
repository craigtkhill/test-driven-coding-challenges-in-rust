pub fn profitable_gamble(_prob: f64, _prize: f64, _pay: f64) -> bool {
    _prob * _prize > _pay
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profitable_gamble_1() {
        assert!(
            profitable_gamble(0.2, 50.0, 9.0),
            "Should return true when prob = 0.2, prize = 50, pay = 9"
        );
    }

    #[test]
    fn test_profitable_gamble_2() {
        assert!(
            !profitable_gamble(0.9, 1.0, 2.0),
            "Should return false when prob = 0.9, prize = 1, pay = 2"
        );
    }

    #[test]
    fn test_profitable_gamble_3() {
        assert!(
            profitable_gamble(0.9, 3.0, 2.0),
            "Should return true when prob = 0.9, prize = 3, pay = 2"
        );
    }

    #[test]
    fn test_profitable_gamble_4() {
        assert!(
            profitable_gamble(0.33, 10.0, 3.30),
            "Should return true when prob = 0.33, prize = 10, pay = 3.30"
        );
    }

    #[test]
    fn test_profitable_gamble_5() {
        assert!(
            !profitable_gamble(0.0, 1000.0, 0.01),
            "Should return false when prob = 0, prize = 1000, pay = 0.01"
        );
    }

    #[test]
    fn test_profitable_gamble_6() {
        assert!(
            profitable_gamble(0.1, 1000.0, 7.0),
            "Should return true when prob = 0.1, prize = 1000, pay = 7"
        );
    }

    #[test]
    fn test_profitable_gamble_7() {
        assert!(
            !profitable_gamble(0.0, 0.0, 0.0),
            "Should return false when prob = 0, prize = 0, pay = 0"
        );
    }
}
