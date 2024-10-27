pub fn is_same_num(a: i32, b: i32) -> bool {
    a == b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(is_same_num(4, 8), false, "4 and 8 should not be equal");
    }

    #[test]
    fn test2() {
        assert_eq!(is_same_num(2, 2), true, "2 and 2 should be equal");
    }

    #[test]
    fn test3() {
        assert_eq!(is_same_num(0, 8), false, "0 and 8 should not be equal");
    }

    #[test]
    fn test4() {
        assert_eq!(is_same_num(1, 1), true, "1 and 1 should be equal");
    }

    #[test]
    fn test5() {
        assert_eq!(is_same_num(12, 3), false, "12 and 3 should not be equal");
    }

    #[test]
    fn test6() {
        assert_eq!(is_same_num(5, 98), false, "5 and 98 should not be equal");
    }
}
