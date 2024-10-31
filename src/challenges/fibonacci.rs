pub fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut current = 1;

    for _ in 2..=n {
        let next = prev + current;
        prev = current;
        current = next;
    }

    current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_zero() {
        assert_eq!(fibonacci(0), 0, "0th Fibonacci number should be 0");
    }

    #[test]
    fn test_fibonacci_one() {
        assert_eq!(fibonacci(1), 1, "1st Fibonacci number should be 1");
    }

    #[test]
    fn test_fibonacci_two() {
        assert_eq!(fibonacci(2), 1, "2nd Fibonacci number should be 1");
    }

    #[test]
    fn test_fibonacci_three() {
        assert_eq!(fibonacci(3), 2, "3rd Fibonacci number should be 2");
    }

    #[test]
    fn test_fibonacci_sequence() {
        // Test the first 10 Fibonacci numbers
        let expected = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
        for (n, expected_value) in expected.iter().enumerate() {
            assert_eq!(
                fibonacci(n as u32),
                *expected_value,
                "Fibonacci number at position {} should be {}",
                n,
                expected_value
            );
        }
    }

    #[test]
    fn test_fibonacci_larger_numbers() {
        assert_eq!(fibonacci(10), 55, "10th Fibonacci number should be 55");
        assert_eq!(fibonacci(15), 610, "15th Fibonacci number should be 610");
        assert_eq!(fibonacci(20), 6765, "20th Fibonacci number should be 6765");
    }

    #[test]
    fn test_fibonacci_known_large_value() {
        // Testing a larger known Fibonacci number to ensure efficiency
        assert_eq!(
            fibonacci(40),
            102334155,
            "40th Fibonacci number should be 102334155"
        );
    }
    // Optional test for very large numbers - uncomment if needed
    #[test]
    fn test_fibonacci_very_large() {
        assert_eq!(
            fibonacci(50),
            12586269025,
            "50th Fibonacci number should be 12586269025"
        );
    }
}
