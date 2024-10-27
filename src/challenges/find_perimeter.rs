pub fn find_perimeter(length: i32, width: i32) -> i32 {
    2 * (length + width)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            find_perimeter(6, 7),
            26,
            "Perimeter should be 26 for length=6 and width=7"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            find_perimeter(20, 10),
            60,
            "Perimeter should be 60 for length=20 and width=10"
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            find_perimeter(2, 9),
            22,
            "Perimeter should be 22 for length=2 and width=9"
        );
    }
}
