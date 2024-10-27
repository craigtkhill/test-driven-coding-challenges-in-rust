pub fn addition(num: i32) -> i32 {
    num + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(addition(2), 3, "Should be 3");
    }

    #[test]
    fn test2() {
        assert_eq!(addition(-9), -8, "Should be -8");
    }

    #[test]
    fn test3() {
        assert_eq!(addition(0), 1, "Should be 1");
    }

    #[test]
    fn test4() {
        assert_eq!(addition(999), 1000, "Should be 1000");
    }

    #[test]
    fn test5() {
        assert_eq!(addition(73), 74, "Should be 74");
    }
}
