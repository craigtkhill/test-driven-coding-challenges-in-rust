pub fn amplify(num: u32) -> Vec<i32> {
    (1..=num)
        .map(|i| {
            if i % 4 == 0 {
                (i * 10) as i32
            } else {
                i as i32
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amplify_single_number() {
        assert_eq!(amplify(1), vec![1]);
    }

    #[test]
    fn test_amplify_four_numbers() {
        assert_eq!(amplify(4), vec![1, 2, 3, 40]);
    }

    #[test]
    fn test_amplify_eight_numbers() {
        assert_eq!(amplify(8), vec![1, 2, 3, 40, 5, 6, 7, 80]);
    }

    #[test]
    fn test_amplify_twenty_five_numbers() {
        assert_eq!(
            amplify(25),
            vec![
                1, 2, 3, 40, 5, 6, 7, 80, 9, 10, 11, 120, 13, 14, 15, 160, 17, 18, 19, 200, 21, 22,
                23, 240, 25
            ]
        );
    }

    // Additional test to verify empty input handling
    #[test]
    fn test_amplify_zero() {
        assert_eq!(amplify(0), Vec::<i32>::new());
    }
}
