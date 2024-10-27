// src/challenges/addition.rs
pub fn addition(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(addition(2, 3), 5, "Should return 5 for inputs 2 and 3");
    }

    #[test]
    fn test2() {
        assert_eq!(
            addition(-3, -6),
            -9,
            "Should return -9 for inputs -3 and -6"
        );
    }

    #[test]
    fn test3() {
        assert_eq!(addition(7, 3), 10, "Should return 10 for inputs 7 and 3");
    }
}
