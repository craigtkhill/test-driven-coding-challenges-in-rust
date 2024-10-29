pub fn pairs(input: (i32, i32)) -> Vec<i32> {
    let vector = vec![input.0, input.1];
    return vector;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_numbers() {
        let input = (1, 2);
        let expected = vec![1, 2];
        assert_eq!(
            pairs(input),
            expected,
            "Failed to convert pair (1, 2) to vector"
        );
    }

    #[test]
    fn test_medium_numbers() {
        let input = (21, 82);
        let expected = vec![21, 82];
        assert_eq!(
            pairs(input),
            expected,
            "Failed to convert pair (21, 82) to vector"
        );
    }

    #[test]
    fn test_large_numbers() {
        let input = (4213, 526);
        let expected = vec![4213, 526];
        assert_eq!(
            pairs(input),
            expected,
            "Failed to convert pair (4213, 526) to vector"
        );
    }

    // Additional test cases
    #[test]
    fn test_negative_numbers() {
        let input = (-5, -10);
        let expected = vec![-5, -10];
        assert_eq!(
            pairs(input),
            expected,
            "Failed to convert pair with negative numbers"
        );
    }

    #[test]
    fn test_zero_values() {
        let input = (0, 0);
        let expected = vec![0, 0];
        assert_eq!(
            pairs(input),
            expected,
            "Failed to convert pair with zero values"
        );
    }

    #[test]
    fn test_mixed_signs() {
        let input = (-42, 17);
        let expected = vec![-42, 17];
        assert_eq!(
            pairs(input),
            expected,
            "Failed to convert pair with mixed signs"
        );
    }
}
