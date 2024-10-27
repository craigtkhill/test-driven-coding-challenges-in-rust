pub fn circuit_power(voltage: i32, current: i32) -> i32 {
    // TODO: Implement this function to make the tests pass
    todo!() // This will cause the tests to fail until you implement it
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            circuit_power(230, 10),
            2300,
            "Should return 2300 when voltage is 230 and current is 10"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            circuit_power(480, 20),
            9600,
            "Should return 9600 when voltage is 480 and current is 20"
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            circuit_power(110, 3),
            330,
            "Should return 330 when voltage is 110 and current is 3"
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            circuit_power(13800, 276),
            3808800,
            "Should return 3808800 when voltage is 13800 and current is 276"
        );
    }
}
