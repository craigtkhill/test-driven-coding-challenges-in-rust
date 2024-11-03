pub fn points(a: i32, b: i32) -> i32 {
    (a * 2) + (b * 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points_1_1() {
        assert_eq!(points(1, 1), 5, "points(1, 1) should return 5");
    }

    #[test]
    fn test_points_1_2() {
        assert_eq!(points(1, 2), 8, "points(1, 2) should return 8");
    }

    #[test]
    fn test_points_2_1() {
        assert_eq!(points(2, 1), 7, "points(2, 1) should return 7");
    }

    #[test]
    fn test_points_2_2() {
        assert_eq!(points(2, 2), 10, "points(2, 2) should return 10");
    }

    #[test]
    fn test_points_large_numbers() {
        assert_eq!(points(69, 420), 1398, "points(69, 420) should return 1398");
    }

    // Helper test to verify properties of the function
    #[test]
    fn test_points_properties() {
        // Test that second parameter has more weight
        assert!(
            points(1, 2) > points(2, 1),
            "points(1, 2) should be greater than points(2, 1)"
        );

        // Test that increasing either parameter increases the result
        assert!(
            points(2, 2) > points(1, 2),
            "Increasing first parameter should increase result"
        );
        assert!(
            points(1, 2) > points(1, 1),
            "Increasing second parameter should increase result"
        );
    }
}
