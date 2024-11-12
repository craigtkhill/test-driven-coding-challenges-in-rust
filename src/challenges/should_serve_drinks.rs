pub fn should_serve_drinks(age: i32, on_break: bool) -> bool {
    if on_break {
        return false;
    }
    age >= 18
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_underage_on_break() {
        assert_eq!(
            should_serve_drinks(17, true),
            false,
            "Should not serve drinks to someone who is underage and on break"
        );
    }

    #[test]
    fn test_adult_on_break() {
        assert_eq!(
            should_serve_drinks(30, true),
            false,
            "Should not serve drinks to someone who is on break, regardless of age"
        );
    }

    #[test]
    fn test_adult_not_on_break() {
        assert_eq!(
            should_serve_drinks(24, false),
            true,
            "Should serve drinks to an adult who is not on break"
        );
    }

    #[test]
    fn test_barely_adult_not_on_break() {
        assert_eq!(
            should_serve_drinks(18, false),
            true,
            "Should serve drinks to someone who is exactly 18 and not on break"
        );
    }

    #[test]
    fn test_child_not_on_break() {
        assert_eq!(
            should_serve_drinks(3, false),
            false,
            "Should not serve drinks to a child, even if not on break"
        );
    }
}
