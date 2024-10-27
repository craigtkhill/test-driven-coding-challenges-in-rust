pub fn squared(num: i32) -> i32 {
    // TODO: Implement this function to make the tests pass
    todo!() // This will cause the tests to fail until you implement it
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(squared(10), 100, "squaring 10");
    }

    #[test]
    fn test2() {
        assert_eq!(squared(69), 4761, "squaring 69");
    }

    #[test]
    fn test3() {
        assert_eq!(squared(666), 443556, "squaring 666");
    }

    #[test]
    fn test4() {
        assert_eq!(squared(-21), 441, "squaring -21");
    }

    #[test]
    fn test5() {
        assert_eq!(squared(21), 441, "squaring 21");
    }
}
