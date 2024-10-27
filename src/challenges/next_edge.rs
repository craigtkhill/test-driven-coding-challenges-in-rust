pub fn next_edge(side1: i32, side2: i32) -> i32 {
    side1 + side2 - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(next_edge(5, 4), 8, "Failed with sides 5 and 4");
    }

    #[test]
    fn test2() {
        assert_eq!(next_edge(8, 3), 10, "Failed with sides 8 and 3");
    }

    #[test]
    fn test3() {
        assert_eq!(next_edge(7, 9), 15, "Failed with sides 7 and 9");
    }

    #[test]
    fn test4() {
        assert_eq!(next_edge(10, 4), 13, "Failed with sides 10 and 4");
    }

    #[test]
    fn test5() {
        assert_eq!(next_edge(7, 2), 8, "Failed with sides 7 and 2");
    }
}
