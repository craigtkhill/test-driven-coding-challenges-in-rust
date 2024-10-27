pub fn reverse(value: bool) -> bool {
    if value == true {
        return false;
    }
    true
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
