pub fn remainder(x: i32, y: i32) -> i32 {
    // TODO: Implement this function to make the tests pass
    todo!() // This will cause the tests to fail until you implement it
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(remainder(7, 2), 1, "Should return 1 when dividing 7 by 2");
    }

    #[test]
    fn test2() {
        assert_eq!(remainder(3, 4), 3, "Should return 3 when dividing 3 by 4");
    }

    #[test]
    fn test3() {
        assert_eq!(
            remainder(-9, -45),
            -9,
            "Should return -9 when dividing -9 by -45"
        );
    }

    #[test]
    fn test4() {
        assert_eq!(remainder(5, 5), 0, "Should return 0 when dividing 5 by 5");
    }
}
