pub fn long_burp(n: usize) -> String {
    // TODO: Implement this function to make the tests pass
    todo!() // This will cause the tests to fail until you implement it
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(long_burp(3), "Burrrp", "Should return 'Burrrp' for n=3");
    }

    #[test]
    fn test2() {
        assert_eq!(
            long_burp(9),
            "Burrrrrrrrrp",
            "Should return 'Burrrrrrrrrp' for n=9"
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            long_burp(10),
            "Burrrrrrrrrrp",
            "Should return 'Burrrrrrrrrrp' for n=10"
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            long_burp(13),
            "Burrrrrrrrrrrrrp",
            "Should return 'Burrrrrrrrrrrrrp' for n=13"
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            long_burp(18),
            "Burrrrrrrrrrrrrrrrrrp",
            "Should return 'Burrrrrrrrrrrrrrrrrrp' for n=18"
        );
    }

    #[test]
    fn test6() {
        assert_eq!(long_burp(1), "Burp", "Should return 'Burp' for n=1");
    }
}
