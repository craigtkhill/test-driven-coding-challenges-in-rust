pub fn convert(hours: i32, minutes: i32) -> i32 {
    // TODO: Implement this function to make the tests pass
    todo!() // This will cause the tests to fail until you implement it
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            convert(1, 0),
            3600,
            "1 hour 0 minutes should be 3600 seconds"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            convert(1, 3),
            3780,
            "1 hour 3 minutes should be 3780 seconds"
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            convert(0, 30),
            1800,
            "0 hours 30 minutes should be 1800 seconds"
        );
    }
}
