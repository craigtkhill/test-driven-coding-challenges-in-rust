pub fn triangle(n: i32) -> i32 {
    // TODO: Implement this function to make the tests pass
    todo!() // This will cause the tests to fail until you implement it
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(triangle(1), 1, "triangle(1) should return 1");
    }

    #[test]
    fn test2() {
        assert_eq!(triangle(2), 3, "triangle(2) should return 3");
    }

    #[test]
    fn test3() {
        assert_eq!(triangle(3), 6, "triangle(3) should return 6");
    }

    #[test]
    fn test4() {
        assert_eq!(triangle(8), 36, "triangle(8) should return 36");
    }

    #[test]
    fn test5() {
        assert_eq!(
            triangle(2153),
            2318781,
            "triangle(2153) should return 2318781"
        );
    }
}
