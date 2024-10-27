pub fn reverse(value: bool) -> bool {
    // TODO: Implement this function to make the test pass
    todo!() // This will cause the test to fail until you implement it
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(reverse(false), true, "Should flip false to true");
    }

    #[test]
    fn test2() {
        assert_eq!(reverse(true), false, "Should flip true to false");
    }
}
