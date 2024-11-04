pub fn flip_int_bool(value: i32) -> i32 {
    if value == 1 {
        0
    } else if value == 0 {
        1
    } else {
        panic!("must be 1 or 0")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_one_to_zero() {
        assert_eq!(flip_int_bool(1), 0, "Should flip 1 to 0");
    }

    #[test]
    fn test_flip_zero_to_one() {
        assert_eq!(flip_int_bool(0), 1, "Should flip 0 to 1");
    }

    // Additional edge case tests
    #[test]
    #[should_panic]
    fn test_invalid_input() {
        // Should panic for any input that's not 0 or 1
        flip_int_bool(2);
    }

    #[test]
    #[should_panic]
    fn test_negative_input() {
        // Should panic for negative inputs
        flip_int_bool(-1);
    }
}
